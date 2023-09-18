use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono;
use sqlx::types::chrono::{DateTime, TimeZone, Utc };
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub username_distinct: Option<String>,
    pub user_create_ts: Option<DateTime<Utc>>
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreateRequest {
    pub username: String
}