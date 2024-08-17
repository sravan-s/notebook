use crate::db;

use anyhow::{self, Context, Ok, Result};

pub async fn run() -> Result<()> {
    db::init().await.context("Couldnt create DB").unwrap();
    db::create_tables()
        .await
        .context("Couldnt create tables")
        .unwrap();
    Ok(())
}
