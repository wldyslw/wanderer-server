#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use server::db::users::create as create_user;
use std::env;

embed_migrations!("migrations");

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL variable must be provided");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_admin_user(c: &PgConnection) {
    let username = env::var("ADMIN_USERNAME").unwrap_or("Admin".to_string());
    let password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD variable must be provided");

    if let Err(e) = create_user(c, username, password) {
        match e {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                println!("Admin user already exists, skipping.")
            }
            _ => panic!(e),
        }
    } else {
        println!("Default user created.")
    }
}

fn main() {
    let connection = establish_connection();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).ok();

    create_admin_user(&connection);
}
