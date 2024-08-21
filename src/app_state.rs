use anyhow::Context;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db::DB_URL;

#[derive(Debug)]
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
}

impl AppState {
    pub async fn new() -> Self {
        let db_pool = SqlitePool::connect(DB_URL)
            .await
            .context("Crete DB_pool for AppState")
            .unwrap();
        AppState { db_pool }
    }
}
