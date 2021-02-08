#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_cors::Cors;

pub mod auth;
pub mod config;
pub mod constants;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use constants::API_V1_BASE_PATH;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("CORS fairing cannot be created")
}

pub fn run() {
    let config = config::get_config();
    rocket::custom(config)
        .mount(
            API_V1_BASE_PATH,
            routes![
                routes::articles::articles_get,
                routes::articles::article_get,
                routes::articles::article_create,
                routes::articles::article_update,
                routes::articles::article_archive,
                routes::auth::sign_in,
                routes::auth::sign_out,
            ],
        )
        .attach(db::DBConnection::fairing())
        .attach(auth::RedisConnection::fairing())
        .attach(cors_fairing())
        .register(catchers![routes::catchers::not_found])
        .launch();
}
