use anyhow::{Context, Ok, Result};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://notebook.db";

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
    let _create_notebooks = sqlx::query(
        "
            CREATE TABLE IF NOT EXISTS notebooks (
                id INTEGER PRIMARY KEY NOT NULL,
                archived TINYINT,
                created_at INTEGER);
        ",
    )
    .execute(&db)
    .await
    .context("Create TABLE notebooks")
    .unwrap();

    let _create_notebook_versions = sqlx::query(
        "
            CREATE TABLE IF NOT EXISTS notebook_versions (
                id INTEGER PRIMARY KEY NOT NULL,
                notebook_id INTEGER NOT NULL,
                created_at INTEGER,
                updated_at INTEGER,
                dependencies TEXT,
                FOREIGN KEY (notebook_id) REFERENCES notebooks(id) ON DELETE CASCADE);
        ",
    )
    .execute(&db)
    .await
    .context("Create TABLE notebook_versions")
    .unwrap();

    let _create_paragraphs = sqlx::query(
        "
            CREATE TABLE IF NOT EXISTS paragraphs (
                id INTEGER PRIMARY KEY NOT NULL,
                notebook_version INTEGER NOT NULL,
                created_at INTEGER,
                updated_at INTEGER,
                status TINYINT,
                code TEXT,
                result TEXT,
                meta TEXT,
                FOREIGN KEY (notebook_version) REFERENCES notebook_versions(id) ON DELETE CASCADE);
        ",
    )
    .execute(&db)
    .await
    .context("Create TABLE paragraphs")
    .unwrap();

    let _create_secrets = sqlx::query(
        "
            CREATE TABLE IF NOT EXISTS secrets (
                id INTEGER PRIMARY KEY NOT NULL,
                notebook_id INTEGER NOT NULL,
                data BLOB,
                FOREIGN KEY (notebook_id) REFERENCES notebooks(id) ON DELETE CASCADE);
        ",
    )
    .execute(&db)
    .await
    .context("Create TABLE secrets")
    .unwrap();

    Ok(())
}
