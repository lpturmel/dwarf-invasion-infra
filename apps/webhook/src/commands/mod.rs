use crate::error::Result;
use axum::http::StatusCode;

use crate::discord::{DiscordPayload, DiscordResponse, InteractionResponse, ResponseType};
use serde::{Deserialize, Serialize};

pub mod addchar;
pub mod delchar;
pub mod queue_sims;
pub mod roll;
pub mod roster;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Class {
    Druid,
    Hunter,
    Mage,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Evoker,
    DeathKnight,
    DemonHunter,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RosterItem {
    pub class: String,
    pub spec: String,
    pub character_name: String,
    pub alt: bool,
    pub user_id: String,
}
pub async fn ping(_body: &DiscordPayload) -> Result<(DiscordResponse, StatusCode)> {
    let res = InteractionResponse::new(ResponseType::Pong, "pong");

    Ok((res, StatusCode::OK))
}
