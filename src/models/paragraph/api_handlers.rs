use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse};

use crate::{app_state, models::notebook};

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

    if notebook_full.is_err() || notebook_full.unwrap().is_none() {
        return (
            StatusCode::NOT_FOUND,
            format!("Notebook with ID: {} not found", notebook_id).into_response(),
        );
    }

    /*
        let paragraphs: Result<Vec<Option<ParagraphFull>> = sqlx::query_as(super::db::GET_PARAGRAPHS_BY_NOTEBOOK_ID)
            .bind(notebook_id)
            .fetch_all(&db_pool)
            .await
            .context("Error fetching paragraphs");
    */
    (StatusCode::OK, "Sucess".into_response())
    /*
    match paragraphs[0] {
        Ok(Some(r)) => (StatusCode::OK, Json(r).into_response()),
        Ok(None) => (StatusCode::NOT_FOUND, "Notebook not found".into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string().into_response(),
        ),
    }
    */
}

pub async fn create_paragraph() -> impl IntoResponse {
    (StatusCode::OK, "sucess".into_response())
}

pub async fn get_paragraph_by_id() -> impl IntoResponse {
    (StatusCode::OK, "sucess".into_response())
}

pub async fn update_paragraph() -> impl IntoResponse {
    (StatusCode::OK, "sucess".into_response())
}

pub async fn delete_paragraph() -> impl IntoResponse {
    (StatusCode::OK, "success".into_response())
}
