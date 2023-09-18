use std::env;
use config::Config;
use sqlx::{Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub fn get_config() -> Config {
    let env = &env::var("env").unwrap();
    let secrets_path = &env::var("secrets.folder.path").unwrap();


    let configs = Config::builder()
        .add_source(config::File::with_name(&format!("{secrets_path}/postgres_{env}.json")))
        .build().unwrap();

    return configs;
}

pub async fn create_postgres_pool(configs: &Config) -> Pool<Postgres> {
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
