#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

use dotenv::dotenv;
use rocket_cors::Cors;

pub mod auth;
pub mod config;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("CORS fairing cannot be created")
}

pub fn run() {
    dotenv().ok();
    let config = config::get_config();
    rocket::custom(config)
        .mount(
            "/api/v1",
            routes![
                routes::articles::articles_get,
                routes::articles::article_get,
                routes::articles::article_create,
                routes::articles::article_update,
                routes::login::login,
            ],
        )
        .attach(config::AppState::secret_retriever())
        .attach(db::DBConnection::fairing())
        .attach(cors_fairing())
        .register(catchers![routes::catchers::not_found])
        .launch();
}
