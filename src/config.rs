use rocket::{
    config::{Config, Environment, Value},
    fairing::AdHoc,
};
use std::collections::HashMap;
use std::env;

pub struct AppState {
    pub secret: Vec<u8>,
}

impl AppState {
    pub fn secret_retriever() -> AdHoc {
        AdHoc::on_attach("Secret retriever", |rocket| {
            let secret = env::var("SECRET_KEY")
                .expect("SECRET_KEY environment variable must be set.")
                .into_bytes();

            Ok(rocket.manage(AppState { secret }))
        })
    }
}

pub fn get_config() -> Config {
    let environment = Environment::active().expect("No environment found");

    let postgres_url =
        env::var("POSTGRES_URL").expect("POSTGRES_URL environment variable must be set.");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable must be set.");

    let mut postgres_config = HashMap::new();
    postgres_config.insert("url", Value::from(postgres_url));

    let mut redis_config = HashMap::new();
    redis_config.insert("url", Value::from(redis_url));

    let mut databases = HashMap::new();
    databases.insert("diesel_postgres_pool", Value::from(postgres_config));
    databases.insert("redis_pool", Value::from(redis_config));

    Config::build(environment)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
