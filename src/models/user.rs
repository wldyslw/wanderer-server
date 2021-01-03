use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

impl<'a> UserNew<'a> {
    pub fn new(username: &'a String, password_hash: &'a String) -> UserNew<'a> {
        UserNew {
            username,
            password_hash,
        }
    }
}

#[derive(Deserialize)]
pub struct UserLoginData {
    pub username: String,
    pub password: String,
}
