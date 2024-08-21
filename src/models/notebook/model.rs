use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct NotebookSummary {
    id: u64,
    name: String,
    description: String,
    created_at: String,
}
