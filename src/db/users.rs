use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use std::fmt;

use crate::models::user::{User, UserNew};
use crate::schema::users;

pub enum LoginError {
    InvalidUsername,
    InvalidPassword,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            &LoginError::InvalidUsername => "Invalid Username",
            &LoginError::InvalidPassword => "Invalid Password",
        };
        write!(f, "{}", message)
    }
}

pub fn create(c: &PgConnection, username: String, password: String) -> Result<User, Error> {
    let hash = scrypt_simple(
        &password,
        &ScryptParams::new(14, 8, 1).expect("Invalid hashing params"),
    )
    .expect("Hashing error");

    diesel::insert_into(users::table)
        .values(UserNew::new(&username, &hash))
        .get_result::<User>(c)
}

pub fn login(c: &PgConnection, username: &str, password: &str) -> Result<User, LoginError> {
    let result = users::table
        .filter(users::username.eq(username))
        .get_result::<User>(c)
        .ok();

    if let Some(user) = result {
        let password_matches = scrypt_check(password, &user.password_hash).ok();
        match password_matches {
            Some(_) => Ok(user),
            None => Err(LoginError::InvalidPassword),
        }
    } else {
        Err(LoginError::InvalidUsername)
    }
}
