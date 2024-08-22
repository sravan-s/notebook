use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct NotebookSummary {
    id: i64,
    name: String,
    description: String,
    updated_at: i64,
}

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct NotebookFull {
    id: i64,
    name: String,
    description: String,
    created_at: i64,
    updated_at: i64,
    archived_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotebookPayload {
    pub name: String,
    pub description: String,
    pub dependencies: String,
    pub secrets: String,
}
