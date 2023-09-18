use std::sync::Arc;
use sqlx::{Pool, Postgres};
use anyhow::Result;
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::types::Uuid;
use crate::models::user::{User, UserCreateRequest};

pub async fn get_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool)
        .await?;

    return Ok(values);
}

pub async fn create_user(pool: Arc<Pool<Postgres>>, new_user: &UserCreateRequest) -> Result<()> {

    let uuid = Uuid::new_v4();
    sqlx::query(
        "
        insert into users(user_id, username, username_distinct)
        values ($1, $2, $3);
        ")
        .bind(&uuid)
        .bind(&new_user.username)
        .bind(&new_user.username.to_lowercase())
        .execute(&*pool)
        .await?;

    return Ok(());
}