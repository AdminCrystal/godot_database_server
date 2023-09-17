use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    // Required values
    pub username: String,
    pub user_id: i32,
    // pub password: String,

    // Makes the field not required and will put in default value
    // #[serde(default)]
    // pub x: String,
    //
    // // Makes the field optional, if not supplied it is null
    // pub y: Option<String>,
}