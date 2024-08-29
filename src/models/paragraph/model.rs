use anyhow::Context;
use futures::future;
use serde::Serialize;
use sqlx::{prelude::FromRow, Pool, Sqlite};
use tokio::task::JoinHandle;

use crate::utils::paragraphs_to_vec;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct ParagraphFull {
    id: i64,
    created_at: i64,
    updated_at: i64,
    status: i64,
    code: String,
    result: String,
    meta: String,
    pub notebook_id: i64,
}

pub async fn paragraphs_from_string(
    plain_str: String,
    notebook_id: i64,
    db_pool: Pool<Sqlite>,
) -> Vec<Option<ParagraphFull>> {
    let paragraphs_handle: Vec<JoinHandle<Option<ParagraphFull>>> = paragraphs_to_vec(&plain_str)
        .map(|pid| {
            let pid = pid.to_owned();
            let db_pool = db_pool.clone();
            tokio::spawn(async move {
                let paragraph = sqlx::query(super::db::GET_PARAGRAPH_BY_ID)
                    .bind(&pid)
                    .fetch_one(&db_pool)
                    .await
                    .context(format!("Fetch paragraphs with ID: {}", pid));
                let paragraph: Option<ParagraphFull> = match paragraph {
                    Ok(p) => {
                        let para = ParagraphFull::from_row(&p);
                        match para {
                            Ok(p) => Some(p),
                            Err(_) => {
                                println!("Couldnt convert row to paragraph: {:?}", para);
                                None
                            }
                        }
                    }
                    Err(_) => None,
                };

                match paragraph {
                    Some(p) => {
                        if p.notebook_id != notebook_id {
                            return None;
                        }
                        Some(p)
                    }
                    None => None,
                }
            })
        })
        .collect();

    // are these in order? need to check
    let results: Vec<Option<ParagraphFull>> = future::join_all(paragraphs_handle)
        .await
        .into_iter()
        .filter_map(|x| match x {
            Ok(x) => Some(x),
            _ => None,
        })
        .collect();

    results
}
