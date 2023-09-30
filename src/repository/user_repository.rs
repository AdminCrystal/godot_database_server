use std::sync::Arc;
use sqlx::{Pool, Postgres, Transaction};
use anyhow::Result;
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::types::Uuid;
use crate::models::user_structs::{FriendRequest, FriendRequestAction, User, UserCreateRequest};

pub async fn get_first_ten_users(pool: Arc<Pool<Postgres>>) -> Result<Vec<User>> {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS fetch first 10 rows only;")
        .fetch_all(&*pool)
        .await?;

    return Ok(values);
}

pub async fn get_user_id_from_username(pool: Arc<Pool<Postgres>>, user_create_request: &UserCreateRequest) -> Result<Option<Uuid>> {
    let user_id: Option<Uuid> = sqlx::query_scalar("SELECT user_id FROM USERS where username_distinct = $1;")
        .bind(&user_create_request.username.to_lowercase())
        .fetch_optional(&*pool)
        .await?;

    return Ok(user_id);
}

pub async fn get_specific_users(pool: Arc<Pool<Postgres>>, user_ids: &Vec<Uuid>) -> Result<Vec<User>> {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS where user_id = ANY ($1);")
        .bind(user_ids)
        .fetch_all(&*pool)
        .await?;

    return Ok(values);
}

pub async fn delete_friend_request(txn: &mut Transaction<'_, Postgres>, friend_request_action: &FriendRequestAction) -> Result<()>  {
    sqlx::query("DELETE FROM friend_requests where create_user_id = $1 and recipient_id = $2;")
        .bind(friend_request_action.user_id)
        .bind(friend_request_action.friend_id)
        .execute(&mut **txn)
        .await?;

    sqlx::query("DELETE FROM friend_requests where create_user_id = $1 and recipient_id = $2;")
        .bind(friend_request_action.friend_id)
        .bind(friend_request_action.user_id)
        .execute(&mut **txn)
        .await?;

    return Ok(());
}

pub async fn add_friend(txn: &mut Transaction<'_, Postgres>, friend_request_action: &FriendRequestAction) -> Result<()>  {
    sqlx::query("insert into friends(user_id, friend_id) values ($1, $2);")
        .bind(friend_request_action.friend_id)
        .bind(friend_request_action.user_id)
        .execute(&mut **txn)
        .await?;

    sqlx::query("insert into friends(user_id, friend_id) values ($1, $2);")
        .bind(friend_request_action.user_id)
        .bind(friend_request_action.friend_id)
        .execute(&mut **txn)
        .await?;

    return Ok(());
}

pub async fn create_user(txn: &mut Transaction<'_, Postgres>, new_user: &UserCreateRequest) -> Result<Uuid> {


    let uuid = Uuid::new_v4();
    sqlx::query(
        "
        insert into users(user_id, username, username_distinct)
        values ($1, $2, $3);
        ")
        .bind(&uuid)
        .bind(&new_user.username)
        .bind(&new_user.username.to_lowercase())
        .execute(&mut **txn)
        .await?;

    return Ok(uuid);
}

pub async fn delete_user(txn: &mut Transaction<'_, Postgres>, new_user: &UserCreateRequest) -> Result<()> {
    sqlx::query(
        "
        delete from users
        where username_distinct = $1;
        ")
        .bind(&new_user.username.to_lowercase())
        .execute(&mut **txn)
        .await?;

    return Ok(());
}

pub async fn send_friend_request(txn: &mut Transaction<'_, Postgres>, friend_request: &FriendRequest) -> Result<()> {

    sqlx::query(
        "
        insert into friend_requests(create_user_id, recipient_id)
        values ($1, $2);
        ")
        .bind(&friend_request.create_user_id)
        .bind(&friend_request.recipient_id)
        .execute(&mut **txn)
        .await?;

    return Ok(());
}

pub async fn get_incoming_friend_requests(pool: Arc<Pool<Postgres>>, user_id: &Uuid) -> Result<Vec<User>> {

    let incoming_friend_requests: Vec<User> = sqlx::query_as(
        "
        select u.user_id, u.username from friend_requests f join users u on u.user_id = f.create_user_id where f.recipient_id = $1
        ")
        .bind(user_id)
        .fetch_all(&*pool)
        .await?;

    return Ok(incoming_friend_requests);
}

pub async fn get_outgoing_friend_requests(pool: Arc<Pool<Postgres>>, user_id: &Uuid) -> Result<Vec<User>> {

    let outgoing_friend_requests: Vec<User> = sqlx::query_as(
        "
        select u.user_id, u.username from friend_requests f join users u on u.user_id = f.recipient_id where create_user_id = $1
        ")
        .bind(user_id)
        .fetch_all(&*pool)
        .await?;

    return Ok(outgoing_friend_requests);
}