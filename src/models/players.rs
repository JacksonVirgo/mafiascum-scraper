use crate::AppState;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum PlayerAlignment {
    Town,
    Mafia,
    Werewolf,
    Cult,
    SelfAlignedKilling,
    SelfAlignedOther,
    Unknown,
}

// Convert to String
impl ToString for PlayerAlignment {
    fn to_string(&self) -> String {
        match self {
            PlayerAlignment::Town => "Town",
            PlayerAlignment::Mafia => "Mafia",
            PlayerAlignment::Werewolf => "Werewolf",
            PlayerAlignment::Cult => "Cult",
            PlayerAlignment::SelfAlignedKilling => "Self-Aligned (Killing)",
            PlayerAlignment::SelfAlignedOther => "Self-Aligned (Other)",
            PlayerAlignment::Unknown => "Unknown",
        }
        .to_string()
    }
}

// Convert from String
impl From<String> for PlayerAlignment {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Town" => PlayerAlignment::Town,
            "Mafia" => PlayerAlignment::Mafia,
            "Werewolf" => PlayerAlignment::Werewolf,
            "Cult" => PlayerAlignment::Cult,
            "Self-Aligned (Killing)" => PlayerAlignment::SelfAlignedKilling,
            "Self-Aligned (Other)" => PlayerAlignment::SelfAlignedOther,
            _ => PlayerAlignment::Unknown,
        }
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
