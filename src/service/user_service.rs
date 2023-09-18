use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::repository::user_repository;
use anyhow::Result;
use uuid::Uuid;
use crate::models::error_message::DevMessage;
use crate::models::user_objects::{FriendRequest, User, UserCreateRequest};


pub async fn get_specific_users(pool: Arc<Pool<Postgres>>, user_ids: &Vec<Uuid>) -> Result<Vec<User>> {
    let users = user_repository::get_specific_users(pool, &user_ids).await?;

    return Ok(users);
}

pub async fn get_first_ten_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let users = user_repository::get_first_ten_users(pool).await?;

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

pub async fn send_friend_request(pool: Arc<Pool<Postgres>>, friend_request: &FriendRequest) -> impl Responder {
    let user = user_repository::send_friend_request(pool, friend_request).await;

    match user {
        Ok(_) => {
            let dev_message = DevMessage {
                message: "Friend request created successfully".to_string()
            };

            HttpResponse::Ok().json(dev_message)
        },
        Err(_) => {
            let dev_message = DevMessage {
                message: "Already have a friend request sent out to that user".to_string()
            };

            return HttpResponse::BadRequest().json(dev_message);
        }
    }
}

pub async fn run_tests(pool: Arc<Pool<Postgres>>) {
    let user1 = UserCreateRequest {
        username: "Admincrystal".to_string(),
    };

    let user2 = UserCreateRequest {
        username: "Brent".to_string(),
    };

    let user3 = UserCreateRequest {
        username: "Hannah".to_string(),
    };

    user_repository::delete_user(pool.clone(), &user1).await.expect("Failed to delete from users table");
    user_repository::delete_user(pool.clone(), &user2).await.expect("Failed to delete from users table");
    user_repository::delete_user(pool.clone(), &user3).await.expect("Failed to delete from users table");

    let id1 = user_repository::create_user(pool.clone(), &user1).await.expect("Failed to add users1 to users table");
    let id2 = user_repository::create_user(pool.clone(), &user2).await.expect("Failed to add users2 to users table");
    let id3 = user_repository::create_user(pool.clone(), &user3).await.expect("Failed to add users3 to users table");

    let id_vec = vec![id1, id2, id3];
    let users = user_repository::get_specific_users(pool.clone(), &id_vec).await.unwrap();

    let friend_request1 = FriendRequest {
        user_id: id1.clone(),
        friend_id: id1.clone(),
    };

    let friend_request2 = FriendRequest {
        user_id: id1.clone(),
        friend_id: id2.clone(),
    };
    user_repository::send_friend_request(pool.clone(), &friend_request1).await.expect("Unable to send first friend request");
    user_repository::send_friend_request(pool.clone(), &friend_request2).await.expect("Unable to send second friend request");
    user_repository::send_friend_request(pool.clone(), &friend_request1).await.expect_err("Created duplicate friend request");

    println!("{users:?}");
    return;
}