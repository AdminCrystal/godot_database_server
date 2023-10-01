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
use crate::models::message_structs::{CreateMessage, MessageBetweenUsers};
use crate::models::user_structs::{UserCreateRequest, User, FriendRequest, FriendRequestAction, BaseUser};
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
            .service(delete_user)
            .service(get_user_id_from_username)
            .service(get_incoming_friend_requests)
            .service(get_outgoing_friend_requests)
            .service(get_friends)
            .service(create_message)
            .service(get_messages_between_users)

    })
    .bind("127.0.0.1:6083")?
    .run()
    .await

}

#[post("/users/create_message")]
async fn create_message(pool: web::Data<Pool<Postgres>>, create_message: web::Json<CreateMessage>) -> impl Responder {
    let users = user_service::create_message(pool.into_inner(), &mut create_message.into_inner()).await.unwrap();
    HttpResponse::Ok().json(users)
}

#[post("/users/get_messages_between_users")]
async fn get_messages_between_users(pool: web::Data<Pool<Postgres>>, message_between_users: web::Json<MessageBetweenUsers>) -> impl Responder {
    let messages = user_service::get_messages_between_users(pool.into_inner(), &message_between_users.into_inner()).await;
    HttpResponse::Ok().json(messages.unwrap())
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

#[post("/users/get_user_id_from_username")]
async fn get_user_id_from_username(pool: web::Data<Pool<Postgres>>, user: web::Json<UserCreateRequest>) -> impl Responder {
    let users = user_service::get_user_id_from_username(pool.into_inner(), &user).await;
    HttpResponse::Ok().json(users.unwrap())
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

#[post("/users/delete_user")]
async fn delete_user(pool: web::Data<Pool<Postgres>>, user: web::Json<UserCreateRequest>) -> impl Responder {
    let response = user_service::delete_user(pool.into_inner(), &user.into_inner()).await.unwrap();

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

#[post("/users/get_incoming_friend_requests")]
async fn get_incoming_friend_requests(pool: web::Data<Pool<Postgres>>, user: web::Json<BaseUser>) -> impl Responder {

    let users = user_service::get_incoming_friend_requests(pool.into_inner(), &user.into_inner()).await;
    HttpResponse::Ok().json(users.unwrap())
}

#[post("/users/get_friends")]
async fn get_friends(pool: web::Data<Pool<Postgres>>, user: web::Json<BaseUser>) -> impl Responder {

    let users = user_service::get_friends(pool.into_inner(), &user.into_inner()).await;
    HttpResponse::Ok().json(users.unwrap())
}

#[post("/users/get_outgoing_friend_requests")]
async fn get_outgoing_friend_requests(pool: web::Data<Pool<Postgres>>, user: web::Json<BaseUser>) -> impl Responder {

    let users = user_service::get_outgoing_friend_requests(pool.into_inner(), &user.into_inner()).await;
    HttpResponse::Ok().json(users.unwrap())
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
