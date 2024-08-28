use anyhow::{Context, Ok, Result};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::models::{notebook, paragraph, secret};

pub const DB_URL: &str = "sqlite://data/notebook.db";

pub async fn init() -> Result<()> {
    println!("Setting up databse");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating databse {}", DB_URL);
        Sqlite::create_database(DB_URL)
            .await
            .context("Create database")
            .unwrap();
        println!("Created databse");
    } else {
        println!("Databse exists");
    }
    Ok(())
}

// to do - convert to migrations
pub async fn create_tables() -> Result<()> {
    println!("Setting up tables");
    let db = SqlitePool::connect(DB_URL)
        .await
        .context("Connect to DB")
        .unwrap();
    let _create_notebooks = sqlx::query(notebook::db::CREATE_TABLE_QUERY)
        .execute(&db)
        .await
        .context("Create TABLE notebooks")
        .unwrap();

    /*
    let _create_notebook_versions = sqlx::query(notebook_version::db::CREATE_TABLE_QUERY)
        .execute(&db)
        .await
        .context("Create TABLE notebook_versions")
        .unwrap();
    */

    let _create_paragraphs = sqlx::query(paragraph::db::CREATE_TABLE_QUERY)
        .execute(&db)
        .await
        .context("Create TABLE paragraphs")
        .unwrap();

    let _create_secrets = sqlx::query(secret::db::CREATE_TABLE_QUERY)
        .execute(&db)
        .await
        .context("Create TABLE secrets")
        .unwrap();

    Ok(())
}
