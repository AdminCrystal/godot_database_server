use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::repository::user_repository;
use anyhow::Result;
use sqlx::postgres::any::AnyConnectionBackend;
use uuid::Uuid;
use crate::models::error_message::DevMessage;
use crate::models::user_structs::{FriendRequest, FriendRequestAction, User, UserCreateRequest};


pub async fn get_specific_users(pool: Arc<Pool<Postgres>>, user_ids: &Vec<Uuid>) -> Result<Vec<User>> {
    let users = user_repository::get_specific_users(pool, &user_ids).await?;

    return Ok(users);
}

pub async fn friend_request_action(pool: Arc<Pool<Postgres>>, friend_request: &FriendRequestAction) -> Result<DevMessage> {
    let mut txn = pool.begin().await?;
    user_repository::delete_friend_request(&mut txn, friend_request).await?;

    if friend_request.accepted_request {
        user_repository::add_friend(&mut txn, friend_request).await?;
    }

    txn.commit().await?;

    return Ok(DevMessage {
        message: "Successfully added friend".to_string(),
    });
}

pub async fn get_first_ten_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let users = user_repository::get_first_ten_users(pool).await?;

    return Ok(users);
}

pub async fn create_user(pool: Arc<Pool<Postgres>>, new_user: &UserCreateRequest) -> Result<impl Responder> {
    let mut txn = pool.begin().await?;
    let user = user_repository::create_user(&mut txn, new_user).await;

    txn.commit().await?;

    match user {
        Ok(_) => {
            let dev_message = DevMessage {
                message: "User created successfully".to_string()
            };

            return Ok(HttpResponse::Ok().json(dev_message));
        },
        Err(_) => {
            let dev_message = DevMessage {
                message: "User already exists".to_string()
            };

            return Ok(HttpResponse::BadRequest().json(dev_message));
        }
    }
}

pub async fn send_friend_request(pool: Arc<Pool<Postgres>>, friend_request: &FriendRequest) -> Result<impl Responder> {
    let mut txn = pool.begin().await?;
    let user = user_repository::send_friend_request(&mut txn, friend_request).await;
    txn.commit().await?;

    match user {
        Ok(_) => {
            let dev_message = DevMessage {
                message: "Friend request created successfully".to_string()
            };

            return Ok(HttpResponse::Ok().json(dev_message));
        },
        Err(_) => {
            let dev_message = DevMessage {
                message: "Already have a friend request sent out to that user".to_string()
            };

            return Ok(HttpResponse::BadRequest().json(dev_message));
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

    let mut txn = pool.begin().await.unwrap();

    user_repository::delete_user(&mut txn, &user1).await.expect("Failed to delete from users table");
    user_repository::delete_user(&mut txn, &user2).await.expect("Failed to delete from users table");
    user_repository::delete_user(&mut txn, &user3).await.expect("Failed to delete from users table");

    let id1 = user_repository::create_user(&mut txn, &user1).await.expect("Failed to add users1 to users table");
    let id2 = user_repository::create_user(&mut txn, &user2).await.expect("Failed to add users2 to users table");
    let id3 = user_repository::create_user(&mut txn, &user3).await.expect("Failed to add users3 to users table");

    txn.commit().await.unwrap();

    let id_vec = vec![id1, id2, id3];
    let users = user_repository::get_specific_users(pool.clone(), &id_vec).await.unwrap();

    assert_eq!(3, users.len());

    let friend_request1 = FriendRequest {
        user_id: id1.clone(),
        friend_id: id2.clone(),
    };

    let friend_request2 = FriendRequest {
        user_id: id1.clone(),
        friend_id: id3.clone(),
    };
    send_friend_request(pool.clone(), &friend_request1).await.unwrap();
    send_friend_request(pool.clone(), &friend_request2).await.unwrap();
    send_friend_request(pool.clone(), &friend_request1).await.unwrap();

    let outgoing1 = user_repository::get_outgoing_friend_requests(pool.clone(), &id1).await.expect("Unable to get outgoing friend requests");
    let outgoing2 = user_repository::get_outgoing_friend_requests(pool.clone(), &id2).await.expect("Unable to get outgoing friend requests");

    let incoming = user_repository::get_incoming_friend_requests(pool.clone(), &id2).await.expect("Unable to get incoming friend requests");

    assert_eq!(2, outgoing1.len());
    assert_eq!(0, outgoing2.len());
    assert_eq!(1, incoming.len());

    let friend_request_accept = FriendRequestAction {
        user_id: id2.clone(),
        friend_id: id1.clone(),
        accepted_request: true,
    };

    let friend_request_reject = FriendRequestAction {
        user_id: id3.clone(),
        friend_id: id1.clone(),
        accepted_request: false,
    };

    friend_request_action(pool.clone(), &friend_request_accept).await.expect("Unable to accept friend request");
    friend_request_action(pool.clone(), &friend_request_reject).await.expect("Unable to reject friend request");

    let outgoing1 = user_repository::get_outgoing_friend_requests(pool.clone(), &id1).await.expect("Unable to get outgoing friend requests");
    let outgoing2 = user_repository::get_outgoing_friend_requests(pool.clone(), &id3).await.expect("Unable to get outgoing friend requests");

    let incoming = user_repository::get_incoming_friend_requests(pool.clone(), &id2).await.expect("Unable to get incoming friend requests");

    assert_eq!(0, outgoing1.len());
    assert_eq!(0, outgoing2.len());
    assert_eq!(0, incoming.len());

    println!("{users:?}");
    return;
}