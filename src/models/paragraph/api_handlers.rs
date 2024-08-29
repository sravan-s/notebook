use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{app_state, models::notebook, utils};

use super::model::ParagraphFull;

pub async fn get_paragraphs(
    notebook_id: String,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let db_pool = app_state.db_pool.clone();
    let notebook_full: Result<Option<notebook::model::NotebookFull>> =
        sqlx::query_as(notebook::db::GET_NOTEBOOK_BY_ID)
            .bind(&notebook_id)
            .fetch_optional(&db_pool)
            .await
            .context("Error in fetching notebook");

    let notebook_full = match notebook_full {
        Ok(Some(nb)) => nb,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                format!("Notebook with ID: {} not found", notebook_id).into_response(),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error in fetching notebook".into_response(),
            )
        }
    };

    let notebook_id = notebook_full.id;
    let paragraphs_list = notebook_full.paragraphs;

    let paragraphs =
        super::model::paragraphs_from_string(paragraphs_list, notebook_id, db_pool).await;

    match paragraphs.iter().any(|x| x.is_none()) {
        true => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Paragraphs have some error".into_response(),
        ),
        false => (StatusCode::OK, Json(paragraphs).into_response()),
    }
}

pub async fn create_paragraph(
    notebook_id: String,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let notebook_id = match notebook_id.parse::<i64>() {
        Ok(i) => i,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "notebook_id cannot be converted to a number".into_response(),
            );
        }
    };
    let db_pool = app_state.db_pool.clone();
    let tx = db_pool
        .begin()
        .await
        .context("Start transaction for create_paragraph");

    let mut tx = match tx {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldnt connect to database".into_response(),
            );
        }
    };

    let now = utils::get_sys_time_in_secs();

    let paragraph_insert_result = sqlx::query(super::db::INSERT_PARAGRAPH)
        //notebook_id
        .bind(notebook_id)
        //created_at
        .bind(now)
        //updated_at
        .bind(now)
        .execute(&mut *tx)
        .await
        .context("Insert paragraph");

    let paragraph_id = match paragraph_insert_result {
        Ok(r) => r.last_insert_rowid(),
        _ => {
            tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Coulndt Insert paragraph to DB".into_response(),
            );
        }
    };

    // add to notebooks
    let notebook: Result<notebook::model::NotebookFull> =
        sqlx::query_as(notebook::db::GET_NOTEBOOK_BY_ID)
            .bind(notebook_id)
            .fetch_one(&mut *tx)
            .await
            .context("Add paragrpah to notebook");

    let notebook = match notebook {
        Ok(n) => n,
        Err(_) => {
            tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldnt insert paragraph to notebook".into_response(),
            );
        }
    };

    let paragraphs = notebook.paragraphs;
    let paragraphs = utils::append_paragraph(&paragraphs, paragraph_id);

    let notebook = sqlx::query(notebook::db::UPDATE_NOTEBOOK_PARAGRAPHS)
        .bind(paragraphs)
        .bind(notebook.id)
        .execute(&mut *tx)
        .await;

    if notebook.is_err() {
        tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldnt update paragraph list".into_response(),
        );
    }

    let paragraph: Result<ParagraphFull> = sqlx::query_as(super::db::GET_PARAGRAPH_BY_ID)
        .bind(paragraph_id)
        .fetch_one(&mut *tx)
        .await
        .context("Fetch created paragraph");

    match paragraph {
        Ok(p) => {
            tx.commit().await;
            return (StatusCode::CREATED, Json(p).into_response());
        }
        Err(_) => {
            tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldnt fetch created paragraph".into_response(),
            );
        }
    }
}

pub async fn get_paragraph_by_id(
    paragraph_id: String,
    notebook_id: String,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let notebook_id = match notebook_id.parse::<i64>() {
        Ok(i) => i,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "notebook_id cannot be converted to a number".into_response(),
            );
        }
    };

    let paragraph_id = match paragraph_id.parse::<i64>() {
        Ok(i) => i,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "notebook_id cannot be converted to a number".into_response(),
            );
        }
    };

    let paragraph: Result<ParagraphFull> = sqlx::query_as(super::db::GET_PARAGRAPH_BY_ID)
        .bind(paragraph_id)
        .fetch_one(&app_state.db_pool)
        .await
        .context("Fetching paragraph");

    match paragraph {
        Ok(p) => {
            if p.notebook_id != notebook_id {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No paragraph within given ID in the notebook".into_response(),
                );
            }
            return (StatusCode::OK, Json(p).into_response());
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldnt fetch paragraph".into_response(),
            );
        }
    }
}

pub async fn update_paragraph() -> impl IntoResponse {
    (StatusCode::OK, "sucess".into_response())
}

pub async fn delete_paragraph() -> impl IntoResponse {
    (StatusCode::OK, "success".into_response())
}
