use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{app_state, models::secret, utils};

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
        .execute(&mut *tx)
        .await;
    //.context("Error in inserting notebook");

    if insert_notebook_result.is_err() {
        let _ = tx.rollback().await;
        println!("{:?}", insert_notebook_result);
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
