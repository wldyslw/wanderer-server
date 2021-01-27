use crate::schema::users;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct UserGet {
    pub id: i32,
    pub username: String,
}

impl From<User> for UserGet {
    fn from(user: User) -> Self {
        UserGet {
            id: user.id,
            username: user.username.clone(),
        }
    }
}

impl<'a> Responder<'a> for UserGet {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Json(self).respond_to(req)
    }
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
