use std::net::IpAddr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct CreateGameRequest {
    pub create_user_id: Uuid,
    pub game_ip: IpAddr,
    pub game_name: String,
    pub is_public: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct JoinGameRequest {
    pub game_id: Uuid,
    pub user_id: Uuid,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct Game {
    pub game_id: Uuid,
    pub create_user_id: Uuid,
    pub game_ip: IpAddr,
    pub game_name: String,
    pub is_public: bool,
    pub created_ts: DateTime<Utc>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct PublicGameRequest {
    pub create_user_id: Option<Uuid>,
    pub game_name: Option<String>,
}