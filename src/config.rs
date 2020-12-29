use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

pub const DATETIME_FORMAT_ARTICLE: &'static str = "%d %B %Y, %H:%M";

pub fn get_config() -> Config {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file.");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(db_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(Environment::Production)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
