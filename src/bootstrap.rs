use crate::db;
use crate::models::firecracker;

use anyhow::{self, Context, Ok, Result};
use tokio::spawn;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let handle = spawn(firecracker::init_vm());
    db::init().await.context("Create DB").unwrap();
    db::create_tables().await.context("Create tables").unwrap();
    let kernal_create_out = handle.await.unwrap();
    println!("Result of creating kernal: {:?}", kernal_create_out);
    Ok(())
}
