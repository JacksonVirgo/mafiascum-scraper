use crate::AppState;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, postgres::PgQueryResult, FromRow};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, EnumString, Display, EnumIter)]
pub enum PlayerAlignment {
    #[strum(serialize = "Town")]
    Town,
    #[strum(serialize = "Mafia")]
    Mafia,
    #[strum(serialize = "Werewolf")]
    Werewolf,
    #[strum(serialize = "Cult")]
    Cult,
    #[strum(serialize = "Self-Aligned (Killing)")]
    SelfAlignedKilling,
    #[strum(serialize = "Self-Aligned (Other)")]
    SelfAlignedOther,
    #[strum(serialize = "Unknown")]
    Unknown,
}

impl PlayerAlignment {
    pub fn to_vec() -> Vec<String> {
        PlayerAlignment::iter().map(|q| q.to_string()).collect()
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub role: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub alignment: Option<PlayerAlignment>,

    // FKs
    pub thread_id: Option<String>,
}

pub async fn get_players(app_state: &Data<AppState>, thread_id: &str) -> Option<Vec<Player>> {
    let db = &app_state.db;
    match sqlx::query_as!(
        Player,
        r#"SELECT id, name, alignment as "alignment: PlayerAlignment", role, created_at, thread_id FROM players WHERE thread_id = $1"#,
        thread_id
    )
    .fetch_all(db)
    .await
    {
        Ok(thread) => Some(thread),
        _ => None,
    }
}

pub async fn create_player(
    app_state: &Data<AppState>,
    thread_id: &str,
    name: &str,
) -> Option<PgQueryResult> {
    let db = &app_state.db;
    match sqlx::query!(
        "INSERT INTO players (name, thread_id) VALUES ($1, $2)",
        name,
        thread_id
    )
    .execute(db)
    .await
    {
        Ok(player) => Some(player),
        _ => None,
    }
}
