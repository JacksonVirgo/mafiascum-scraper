use crate::AppState;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{self, postgres::PgQueryResult};
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
    #[strum(serialize = "SelfAlignedKilling")]
    SelfAlignedKilling,
    #[strum(serialize = "SelfAlignedOther")]
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
    pub aliases: Vec<String>,
    pub replacements: Vec<String>,

    // FKs
    pub thread_id: Option<String>,
}

pub struct UpdatePlayer {
    pub id: i32,
    pub name: String,
    pub role: Option<String>,
    pub aliases: Vec<String>,
    pub replacements: Vec<String>,
    pub alignment: Option<PlayerAlignment>,
}

pub async fn get_players(app_state: &Data<AppState>, thread_id: &str) -> Option<Vec<Player>> {
    let db = &app_state.db;
    match sqlx::query_as!(
        Player,
        r#"SELECT id, name, alignment as "alignment: PlayerAlignment", role, aliases, replacements, created_at, thread_id FROM players WHERE thread_id = $1"#,
        thread_id
    )
    .fetch_all(db)
    .await
    {
        Ok(thread) => Some(thread),
        _ => None,
    }
}

pub async fn get_player(app_state: &Data<AppState>, id: i32) -> Option<Player> {
    let db = &app_state.db;
    match sqlx::query_as!(
        Player,
        r#"SELECT id, name, alignment as "alignment: PlayerAlignment", role, aliases, replacements, created_at, thread_id FROM players WHERE id = $1"#,
        id
    )
    .fetch_one(db)
    .await
    {
        Ok(player) => Some(player),
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

pub async fn update_player(
    app_state: &Data<AppState>,
    player: UpdatePlayer,
) -> Option<PgQueryResult> {
    let db = &app_state.db;

    let role = match player.role {
        Some(r) => {
            if r.trim().is_empty() {
                None
            } else {
                Some(r)
            }
        }
        None => None,
    };

    match sqlx::query!(
        r#"UPDATE players SET name = $1, alignment = ($2::text)::alignment, role = $3, aliases = $4, replacements = $5 WHERE id = $6"#,
        player.name,
        player.alignment.map(|a| a.to_string()), // Convert alignment to string
        role,
        &player.aliases,
        &player.replacements,
        player.id
    )
    .execute(db)
    .await
    {
        Ok(player) => Some(player),
        _ => None,
    }
}
