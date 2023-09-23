#[macro_use]
extern crate macro_rules_attribute;

mod configs;
mod repository;
mod service;
mod models;

use configs::{configurations};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::__private::kind::TraitKind;
use sqlx::{Pool, Postgres};
use config::Config;
use uuid::Uuid;
use crate::models::game_structs::{CreateGameRequest, JoinGameRequest, PublicGameRequest};
use crate::models::error_message::DevMessage;
use crate::models::user_structs::{UserCreateRequest, User, FriendRequest, FriendRequestAction};
use crate::service::{game_service, user_service};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = configurations::get_config();

    let pool = configurations::create_postgres_pool(&config).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(health)
            .service(get_first_ten_users)
            .service(get_users)
            .service(create_user)
            .service(send_friend_request)
            .service(run_tests)
            .service(friend_request_action)
            .service(create_game)
            .service(join_game)
            .service(get_public_games)

    })
    .bind("127.0.0.1:6083")?
    .run()
    .await

}

#[post("/users/friend_request_action")]
async fn friend_request_action(pool: web::Data<Pool<Postgres>>, friend_request_action: web::Json<FriendRequestAction>) -> impl Responder {
    let users = user_service::friend_request_action(pool.into_inner(), &friend_request_action.into_inner()).await.unwrap();

    HttpResponse::Ok().json(users)
}
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}

#[get("/users/get_first_ten_users")]
async fn get_first_ten_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let users = user_service::get_first_ten_users(pool.into_inner()).await.unwrap();
    HttpResponse::Ok().json(users)
}


#[post("/users/get_users")]
async fn get_users(pool: web::Data<Pool<Postgres>>, user_ids: web::Json<Vec<Uuid>>) -> impl Responder {
    let users = user_service::get_specific_users(pool.into_inner(), &user_ids.into_inner()).await.unwrap();

    HttpResponse::Ok().json(users)
}

#[post("/users/create_user")]
async fn create_user(pool: web::Data<Pool<Postgres>>, user: web::Json<UserCreateRequest>) -> impl Responder {
    let response = user_service::create_user(pool.into_inner(), &user.into_inner()).await.unwrap();

    return response;
}

#[post("/users/send_friend_request")]
async fn send_friend_request(pool: web::Data<Pool<Postgres>>, user: web::Json<FriendRequest>) -> impl Responder {
    let response = user_service::send_friend_request(pool.into_inner(), &user.into_inner()).await.unwrap();

    return response;
}

#[get("/users/run_tests")]
async fn run_tests(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    user_service::run_tests(pool.into_inner()).await;

    return HttpResponse::Ok();
}

#[post("/games/create_game")]
async fn create_game(pool: web::Data<Pool<Postgres>>, create_game_request: web::Json<CreateGameRequest>) -> impl Responder {
    let response = game_service::create_game(pool.into_inner(), &create_game_request.into_inner()).await.unwrap();

    return response;
}

#[post("/games/join_game")]
async fn join_game(pool: web::Data<Pool<Postgres>>, join_game_request: web::Json<JoinGameRequest>) -> impl Responder {
    let response = game_service::join_game(pool.into_inner(), &join_game_request.into_inner()).await.unwrap();

    return response;
}

#[get("/games/get_active_games")]
async fn get_public_games(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let response = game_service::get_public_games(pool.into_inner()).await;

    return response.unwrap();
}
