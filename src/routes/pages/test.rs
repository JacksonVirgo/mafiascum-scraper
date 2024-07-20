use crate::AppState;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[get("/test")]
async fn test(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

#[get("/test/{id}")]
async fn test_id(state: Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user.created_at),
        Err(_) => HttpResponse::NotFound().json("No user found"),
    }
}
