use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use crate::models::error_message::DevMessage;
use crate::models::game_structs::{BaseGame, CreateGameRequest, JoinGameRequest, PublicGameRequest};
use crate::repository::game_repository;
use anyhow::Result;


pub async fn create_game(pool: Arc<Pool<Postgres>>, create_game_request: &CreateGameRequest) -> Result<impl Responder> {
    let mut txn = pool.begin().await?;
    let game_id = game_repository::create_game(&mut txn, create_game_request).await?;

    txn.commit().await?;

    let base_game = BaseGame {
        game_id,
    };
    return Ok(HttpResponse::Ok().json(base_game));
}

pub async fn join_game(pool: Arc<Pool<Postgres>>, join_game_request: &JoinGameRequest) -> Result<impl Responder> {
    let mut txn = pool.begin().await?;

    game_repository::join_game(&mut txn, join_game_request).await?;
    txn.commit().await?;

    let game_ip = game_repository::get_game_ip(pool.clone(), &join_game_request).await?;

    return Ok(HttpResponse::Ok().body(game_ip.to_string()));

}

pub async fn get_public_games(pool: Arc<Pool<Postgres>>) -> Result<impl Responder> {
    let games = game_repository::get_public_games(pool.clone()).await?;

    return Ok(HttpResponse::Ok().json(games));
}