#[macro_use]
extern crate macro_rules_attribute;

mod configs;
mod repository;
mod service;
mod models;

use configs::{configurations};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use config::Config;
use uuid::Uuid;
use crate::models::error_message::DevMessage;
use crate::models::user_objects::{UserCreateRequest, User, FriendRequest, FriendRequestAction};
use crate::service::user_service;


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
    let response = user_service::create_user(pool.into_inner(), &user.into_inner()).await;

    return response;
}

#[post("/users/send_friend_request")]
async fn send_friend_request(pool: web::Data<Pool<Postgres>>, user: web::Json<FriendRequest>) -> impl Responder {
    let response = user_service::send_friend_request(pool.into_inner(), &user.into_inner()).await;

    return response;
}

#[get("/users/run_tests")]
async fn run_tests(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    user_service::run_tests(pool.into_inner()).await;

    return HttpResponse::Ok();
}