use std::fmt::{Debug};
// use macro_rules_attribute::derive_alias;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

// #[derive(Debug)]
// struct MyError {
//     err: anyhow::Error,
// }
//
// impl Display for MyError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl actix_web::error::ResponseError for MyError {
//
// }
// impl From<anyhow::Error> for MyError {
//     fn from(err: anyhow::Error) -> MyError {
//         MyError { err }
//     }
// }

// impl From<std::io::Error> for anyhow::Error {
//     fn from(err: anyhow::Error) -> MyError {
//         MyError { err }
//     }
// }

// derive_alias! {
//     #[derive(Data!)] = #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)];
//     #[derive(SqlData!)] = #[derive(Data!, FromRow)];
// }

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, FromRow)]
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