#[macro_use]
extern crate macro_rules_attribute;

mod structs;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use crate::structs::User;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use std::env;
use config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = get_config();

    let pool = create_postgres_pool(&config).await;

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

async fn create_postgres_pool(configs: &Config) -> Pool<Postgres> {
    let username: String = configs.get("postgres_username").unwrap();
    let password: String = configs.get("postgres_password").unwrap();
    let port: u16 = configs.get("postgres_port").unwrap();
    let host: String = configs.get("postgres_host").unwrap();

    let conn = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .password(&password);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(conn)
        .await.unwrap();

    return pool;
}

fn get_config() -> Config {
    let env = &env::var("env").unwrap();
    let secrets_path = &env::var("secrets.folder.path").unwrap();


    let configs = Config::builder()
        .add_source(config::File::with_name(&format!("{}/postgres_{}.json", secrets_path, env)))
        .build().unwrap();

    return configs;
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}


#[post("/echo")]
async fn echo(req_body: String, pool: web::Data<Pool<Postgres>>, configs: web::Data<Config>) -> impl Responder {
    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool.into_inner())
        .await.unwrap();

    println!("{:?}", values);

    return HttpResponse::Ok().body(req_body)
}


#[post("/json")]
async fn json(pool: web::Data<Pool<Postgres>>, user: web::Json<User>) -> impl Responder {
    // into_inner returns all of the objects data
    let x = user.into_inner();

    let values: Vec<User> = sqlx::query_as("SELECT * FROM USERS")
        .fetch_all(&*pool.into_inner())
        .await.unwrap();

    println!("{:?}", values);

    HttpResponse::Ok().json(x)
}