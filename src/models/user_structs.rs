use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;
use sqlx::types::chrono::{DateTime, Utc };
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    #[sqlx(default)]
    pub username: Option<String>,
    #[sqlx(default)]
    pub username_distinct: Option<String>,
    #[sqlx(default)]
    pub user_create_ts: Option<DateTime<Utc>>
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreateRequest {
    pub username: String
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct FriendRequest {
    pub create_user_id: Uuid,
    pub recipient_id: Uuid,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct BaseUser {
    pub user_id: Uuid,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct FriendRequestAction {
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub accepted_request: bool,
}