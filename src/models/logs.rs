use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

use crate::utils::app_state::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct LogEntry {
    pub id: i32,
    pub level: LogLevel,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[allow(dead_code)]
pub async fn get_log(app_state: Data<AppState>, id: i32) -> Option<LogEntry> {
    let db = &app_state.db;
    match sqlx::query_as!(
        LogEntry,
        r#"SELECT id, level as "level: LogLevel", message, created_at FROM logs WHERE id = $1"#,
        id
    )
    .fetch_optional(db)
    .await
    {
        Ok(Some(thread)) => Some(thread),
        _ => None,
    }
}

#[allow(dead_code)]
pub async fn log(app_state: Data<AppState>, level: LogLevel, message: String) {
    let db = &app_state.db;
    match sqlx::query!(
        r#"INSERT INTO logs (level, message) VALUES ($1, $2)"#,
        level as LogLevel,
        message
    )
    .execute(db)
    .await
    {
        Ok(_) => (),
        _ => (),
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
