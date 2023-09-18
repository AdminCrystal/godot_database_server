use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::structs::User;
use anyhow::Result;

pub async fn get_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool)
        .await?;

    return Ok(values);
}