mod structs;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use crate::structs::User;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use dotenv::dotenv;
use std::env;
use config::Config;
use std::collections::HashMap;
use lazy_static::lazy_static;
use serde_json::Value;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::builder()
        .add_source(config::File::with_name("C:/Users/mrbre/CLionProjects/leetcode/tokio_test/src/application-dev.json"))
        .build()
        .expect("Couldn't grab config files"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let username: String = SETTINGS.read().unwrap().get("database_username").unwrap();
    // let username = env::var("DB_USERNAME").expect("Db username is missing");
    let conn = PgConnectOptions::new()
        .host("localhost")
        .port(5433)
        .username(&username)
        .password("postgres");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(conn)
        .await.expect("bad connection details");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
async fn echo(req_body: String, pool: web::Data<Pool<Postgres>>,) -> impl Responder {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool.into_inner())
        .await.unwrap();

    println!("{:?}", values);

    HttpResponse::Ok().body(req_body)
}


#[post("/json")]
async fn json(pool: web::Data<Pool<Postgres>>, user: web::Json<User>) -> impl Responder {
    // into_inner returns all of the objects data
    let x = user.into_inner();
    let y = 5;

    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool.into_inner())
        .await.unwrap();

    println!("{:?}", values);
    let x = 5;

    HttpResponse::Ok().json(x)
}