#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

use rocket_cors::Cors;

pub mod config;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("CORS fairing cannot be created")
}

fn main() {
    let config = config::get_config();
    rocket::custom(config)
        .mount(
            "/api",
            routes![
                routes::articles::articles_get,
                routes::articles::article_get
            ],
        )
        .attach(db::DBConnection::fairing())
        .attach(cors_fairing())
        .register(catchers![routes::not_found::not_found])
        .launch();
}
