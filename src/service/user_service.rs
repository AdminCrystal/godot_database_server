use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::structs::User;

use crate::repository::user_repository;
use anyhow::Result;



pub async fn get_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let users = user_repository::get_users(pool).await?;

    return Ok(users);
}