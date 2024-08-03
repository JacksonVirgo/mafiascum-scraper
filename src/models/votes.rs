use crate::utils::app_state::AppState;
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{self, postgres::PgQueryResult};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Vote {
    pub id: i32,
    pub author: String,
    pub target: String,
    pub post_number: i32,
    pub target_correction: Option<String>,

    // FKs
    pub thread_id: String,
}

pub struct NewVote {
    pub author: String,
    pub target: String,
    pub post_number: i32,
    pub target_correction: Option<String>,
    pub thread_id: String,
}

pub enum VoteQuery {
    Thread(String),
    Player(i32),
}

pub async fn get_vote(state: &Data<AppState>, id: i32) -> Option<Vote> {
    let db = &state.db;
    match sqlx::query_as!(
        Vote,
        r#"SELECT id, author, target, post_number, target_correction, thread_id FROM votes WHERE id = $1"#,
        id
    )
    .fetch_one(db)
    .await
    {
        Ok(vote) => Some(vote),
        _ => None,
    }
}

pub async fn get_votes(state: &Data<AppState>, thread_id: &str) -> Option<Vec<Vote>> {
    let db = &state.db;
    match sqlx::query_as!(
            Vote,
            r#"SELECT id, author, target, post_number, target_correction, thread_id FROM votes WHERE thread_id = $1"#,
            thread_id
        )
        .fetch_all(db)
        .await
        {
            Ok(votes) => Some(votes),
            _ => None,
        }
}

pub async fn create_vote(state: &Data<AppState>, vote: NewVote) -> Option<PgQueryResult> {
    let db = &state.db;
    match sqlx::query!(
        "INSERT INTO votes (author, target, post_number, target_correction, thread_id) VALUES ($1, $2, $3, $4, $5)",
        vote.author,
        vote.target,
        vote.post_number,
        vote.target_correction,
        vote.thread_id,
    )
    .execute(db)
    .await
    {
        Ok(query) => Some(query),
        _ => None,
    }
}
