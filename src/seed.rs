#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate scrypt;
extern crate wanderer;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use wanderer::db::users::create as create_user;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL variable must be provided");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_admin_user(c: &PgConnection) {
    let username = env::var("ADMIN_USERNAME").unwrap_or("admin".to_string());
    let password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD variable must be provided");

    create_user(c, username, password).expect("Error creating admin user");
}

pub fn main() {
    dotenv().ok();
    let connection = establish_connection();
    create_admin_user(&connection);
}
