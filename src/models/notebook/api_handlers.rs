use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::app_state;

pub async fn get_notebooks(app_state: Arc<app_state::AppState>) -> impl IntoResponse {
    let results: Result<Vec<super::model::NotebookSummary>> =
        sqlx::query_as(super::db::GET_NOTEBOOKS_NON_ARCHIVED)
            .fetch_all(&app_state.db_pool)
            .await
            .context("Error in running query GET_NOTEBOOKS_NON_ARCHIVED");
    println!("{:?}", results);
    match results {
        Ok(r) => (StatusCode::OK, Json(r).into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(e.to_string()).into_response(),
        ),
    }
}
