use crate::AppState;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Thread {
    id: i32,
    thread_id: String,
    title: Option<String>,
    queue: Option<String>,
    queue_index: Option<i32>,
    created_at: Option<NaiveDateTime>,
}

pub async fn get_thread(app_state: Data<AppState>, thread_id: String) -> Option<Thread> {
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

// pub async fn create_thread(app_state: Data<AppState>, thread_id: String) -> Option<Thread> {
//     let db = &app_state.db;
//     match sqlx::query_as!(
//         Thread,
//         "INSERT INTO threads (thread_id) VALUES ($1) RETURNING *",
//         thread_id
//     )
//     .fetch_one(db)
//     .await
//     {
//         Ok(thread) => Some(thread),
//         _ => None,
//     }
// }
