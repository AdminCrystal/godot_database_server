use std::net::IpAddr;
use std::sync::Arc;
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;
use crate::models::game_structs::{CreateGameRequest, JoinGameRequest};
use anyhow::Result;


pub async fn create_game(txn: &mut Transaction<'_, Postgres>, create_game_request: &CreateGameRequest) -> Result<Uuid> {

    let game_id = Uuid::new_v4();

    sqlx::query(
        "
        insert into games(game_id, create_user_id, game_ip, is_active, game_name)
        values ($1, $2, $3, $4, $5);
        ")
        .bind(&game_id)
        .bind(&create_game_request.create_user_id)
        .bind(&create_game_request.game_ip.to_string())
        .bind(true)
        .bind(&create_game_request.game_name)
        .execute(&mut **txn)
        .await?;

    return Ok(game_id);
}

pub async fn join_game(txn: &mut Transaction<'_, Postgres>, join_game_request: &JoinGameRequest) -> Result<()> {
    sqlx::query(
        "
        insert into game_participants(game_id, user_id, user_ip)
        values ($1, $2);
        ")
        .bind(&join_game_request.game_id) // purposefully flipped
        .bind(&join_game_request.user_id)
        .execute(&mut **txn)
        .await?;

    return Ok(());
}

pub async fn get_game_ip(pool: Arc<Pool<Postgres>>, join_game_request: &JoinGameRequest) -> Result<IpAddr> {
    let game_ip: IpAddr = sqlx::query_scalar(
        "
        select game_ip from games where game_id = $1
        ")
        .bind(&join_game_request.game_id)
        .fetch_one(&*pool)
        .await?;

    return Ok(game_ip);
}