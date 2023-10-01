use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct CreateMessage {
    pub create_user_id: Uuid,
    pub message: String,
    pub initial_recipients: Option<Vec<Uuid>>,
    pub game_id: Option<Uuid>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub create_user_id: Uuid,
    pub message: String,
    pub game_id: Option<Uuid>,
    pub message_create_ts: DateTime<Utc>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct MessageBetweenUsers {
    pub user_id1: Uuid,
    pub user_id2: Uuid,
}