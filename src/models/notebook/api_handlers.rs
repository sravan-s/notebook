use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state,
    models::{paragraph, secret},
    utils,
};

pub async fn get_notebooks(app_state: Arc<app_state::AppState>) -> impl IntoResponse {
    let results: Result<Vec<super::model::NotebookSummary>> =
        sqlx::query_as(super::db::GET_NOTEBOOKS_NON_ARCHIVED)
            .fetch_all(&app_state.db_pool)
            .await
            .context("Error in running query GET_NOTEBOOKS_NON_ARCHIVED");
    match results {
        Ok(r) => (StatusCode::OK, Json(r).into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().into_response(),
        ),
    }
}

pub async fn get_notebook_with_id(
    notebook_id: String,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let result: Result<Option<super::model::NotebookFull>> =
        sqlx::query_as(super::db::GET_NOTEBOOK_BY_ID)
            .bind(notebook_id)
            .fetch_optional(&app_state.db_pool)
            .await
            .context("Error in fetching notebook");
    match result {
        Ok(Some(r)) => (StatusCode::OK, Json(r).into_response()),
        Ok(None) => (StatusCode::NOT_FOUND, "Notebook not found".into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().into_response(),
        ),
    }
}

pub async fn run_notebook_with_id(
    _notebook_id: String,
    _app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    // to do firecracker
    (StatusCode::OK, Json("{}").into_response())
}

pub async fn create_notebook(
    payload: super::model::CreateNotebookPayload,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let now = utils::get_sys_time_in_secs();
    let pool = app_state.db_pool.clone();
    let tx = pool.begin().await.context("Create notebook.transaction");
    if tx.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Create notebook DB error".into_response(),
        );
    }
    let mut tx = tx.unwrap();
    let insert_notebook_result = sqlx::query(super::db::INSERT_NOTEBOOK)
        // name, description, created_at, updated_at, dependencies
        .bind(payload.name)
        .bind(payload.description)
        .bind(now)
        .bind(now)
        .bind(payload.dependencies)
        .bind("".to_string()) // paragraphs - empty string
        .execute(&mut *tx)
        .await;
    //.context("Error in inserting notebook");

    if insert_notebook_result.is_err() {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Couldnt save notebook: {}",
                insert_notebook_result.err().unwrap()
            )
            .into_response(),
        );
    }
    let notebook_id = insert_notebook_result.unwrap().last_insert_rowid();

    let insert_secrets_result = sqlx::query(secret::db::INSERT_SECRET)
        // notebook_id, secrets
        .bind(notebook_id)
        .bind(payload.secrets)
        .execute(&mut *tx)
        .await
        .context("Error adding secret");
    if insert_secrets_result.is_err() {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Couldnt save Secrets: {}",
                insert_secrets_result.err().unwrap()
            )
            .into_response(),
        );
    }

    let transaction_result = tx.commit().await;

    if transaction_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            transaction_result
                .err()
                .unwrap()
                .to_string()
                .into_response(),
        );
    }

    let fetched: Result<Option<super::model::NotebookSummary>> =
        sqlx::query_as(super::db::GET_NOTEBOOK_BY_ID)
            .bind(notebook_id)
            .fetch_optional(&app_state.db_pool)
            .await
            .context("Error in fetching created notebook");
    match fetched {
        Ok(Some(r)) => (StatusCode::OK, Json(r).into_response()),
        Ok(None) => (StatusCode::NOT_FOUND, "Notebook not found".into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().into_response(),
        ),
    }
}

pub async fn reorder_paragraphs(
    notebook_id: String,
    paragraphs: String,
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
    let paragraphs_doesnt_exist = paragraph::model::paragraphs_from_string(
        paragraphs.clone(),
        notebook_id,
        app_state.db_pool.clone(),
    )
    .await
    .into_iter()
    .any(|z| z.is_none());

    if paragraphs_doesnt_exist {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Some paragraphs doesnt exist or payload is malformed".into_response(),
        );
    }

    let result = sqlx::query(super::db::UPDATE_NOTEBOOK_PARAGRAPHS)
        .bind(paragraphs)
        .bind(notebook_id)
        .execute(&app_state.db_pool)
        .await
        .context("Update notebook");

    if result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldnt insert paragraphs to DB".into_response(),
        );
    }
    let notebook_full: Result<super::model::NotebookFull> =
        sqlx::query_as(super::db::GET_NOTEBOOK_BY_ID)
            .bind(notebook_id)
            .fetch_one(&app_state.db_pool)
            .await
            .context("Fetching inserted notebook");

    match notebook_full {
        Ok(n) => (StatusCode::OK, Json(n).into_response()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "paragraphs were added, but couldnt retrive notebook afterwards".into_response(),
        ),
    }
}

pub async fn delete_notebook_with_id(
    notebook_id: String,
    app_state: Arc<app_state::AppState>,
) -> impl IntoResponse {
    let now = utils::get_sys_time_in_secs();
    let result = sqlx::query(super::db::DELETE_NOTEBOOK_BY_ID)
        .bind(now)
        .bind(notebook_id)
        .execute(&app_state.db_pool)
        .await
        .context("Error in deleting notebook");
    match result {
        Ok(res) => {
            println!("{:?}", res);
            if res.rows_affected() == 0 {
                (
                    StatusCode::NOT_FOUND,
                    "NotebookID not found".into_response(),
                )
            } else {
                (StatusCode::OK, "success".into_response())
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().into_response(),
        ),
    }
}
