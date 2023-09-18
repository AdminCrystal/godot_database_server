#[macro_use]
extern crate macro_rules_attribute;

mod structs;
mod configs;
mod repository;
mod service;

use configs::{configurations};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use crate::structs::User;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use std::env;
use config::Config;
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
            .service(echo)
            .service(json)

    })
    .bind("127.0.0.1:6083")?
    .run()
    .await

}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}


#[post("/echo")]
async fn echo(req_body: String, pool: web::Data<Pool<Postgres>>, configs: web::Data<Config>) -> impl Responder {
    return HttpResponse::Ok().body(req_body)
}


#[post("/json")]
async fn json(pool: web::Data<Pool<Postgres>>, user: web::Json<User>) -> impl Responder {
    let users = user_service::get_users(pool.into_inner()).await.unwrap();

    println!("{:?}", users);


    HttpResponse::Ok().json(users)
}