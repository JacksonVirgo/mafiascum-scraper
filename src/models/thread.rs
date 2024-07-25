use crate::{scraping::scraper::PageData, AppState};
use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Thread {
    id: i32,
    thread_id: String,
    pub title: Option<String>,
    pub queue: Option<String>,
    pub queue_index: Option<i32>,
    created_at: Option<NaiveDateTime>,
}

pub struct ThreadUpdate {
    pub title: String,
    pub queue: String,
    pub queue_index: i32,
}

pub async fn get_thread(app_state: &Data<AppState>, thread_id: &str) -> Option<Thread> {
    let db = &app_state.db;
    match sqlx::query_as!(
        Thread,
        "SELECT * FROM threads WHERE thread_id = $1",
        thread_id
    )
    .fetch_optional(db)
    .await
    {
        Ok(Some(thread)) => Some(thread),
        _ => None,
    }
}

pub async fn create_thread(app_state: &Data<AppState>, thread_id: &str) -> Option<Thread> {
    let db = &app_state.db;
    match sqlx::query_as!(
        Thread,
        "INSERT INTO threads (thread_id) VALUES ($1) RETURNING *",
        thread_id
    )
    .fetch_one(db)
    .await
    {
        Ok(thread) => Some(thread),
        _ => None,
    }
}

pub async fn update_thread(
    app_state: &Data<AppState>,
    thread_id: &str,
    data: ThreadUpdate,
) -> Result<(), sqlx::Error> {
    let db = &app_state.db;

    match sqlx::query!(
        "UPDATE threads SET title = $1, queue = $2, queue_index = $3 WHERE thread_id = $4",
        data.title,
        data.queue,
        data.queue_index,
        thread_id
    )
    .execute(db)
    .await
    {
        Ok(_) => {
            println!("Updated thread {}", thread_id);
            Ok(())
        }
        Err(err) => {
            println!("Failed to update thread {}: {}", thread_id, err);
            Err(err)
        }
    }
}
