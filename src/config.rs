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
                .expect("No SECRET_KEY environment variable found")
                .into_bytes();

            Ok(rocket.manage(AppState { secret }))
        })
    }
}

pub fn get_config() -> Config {
    let environment = Environment::active().expect("No environment found");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file.");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(db_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
