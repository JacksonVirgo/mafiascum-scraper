use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[get("/test")]
async fn test(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}
