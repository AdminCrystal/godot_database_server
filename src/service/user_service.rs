use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::repository::user_repository;
use anyhow::Result;
use crate::models::error_message::DevMessage;
use crate::models::user::{User, UserCreateRequest};


pub async fn get_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let users = user_repository::get_users(pool).await?;

    return Ok(users);
}

pub async fn create_user(pool: Arc<Pool<Postgres>>, new_user: &UserCreateRequest) -> impl Responder {
    let user = user_repository::create_user(pool, new_user).await;

    match user {
        Ok(_) => {
            let dev_message = DevMessage {
                message: "User created successfully".to_string()
            };

            HttpResponse::Ok().json(dev_message)
        },
        Err(_) => {
            let dev_message = DevMessage {
                message: "User already exists".to_string()
            };

            return HttpResponse::BadRequest().json(dev_message);
        }
    }
}