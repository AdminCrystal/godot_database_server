use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    // Required values
    pub username: String,
    pub user_id: Uuid,
    pub username_distinct: String,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreateRequest {
    pub username: String
}