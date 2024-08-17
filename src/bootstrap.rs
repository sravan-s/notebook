use crate::db;

use anyhow::{self, Context, Ok, Result};

pub async fn run() -> Result<()> {
    db::init().await.context("Create DB").unwrap();
    db::create_tables().await.context("Create tables").unwrap();
    Ok(())
}
