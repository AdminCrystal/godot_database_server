use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct CreateGameRequest {
    pub create_user_id: Uuid,
    pub game_ip: IpAddr,
    pub is_active: bool,
    pub game_name: String
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct JoinGameRequest {
    pub game_id: Uuid,
    pub user_id: Uuid
}