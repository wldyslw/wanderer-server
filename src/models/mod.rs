use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;
pub mod article;
pub mod user;

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    code: i32,
    message: String,
    description: String,
}

impl ErrorMessage {
    pub fn new(message: String, description: String) -> Self {
        ErrorMessage {
            code: Default::default(),
            message,
            description,
        }
    }
}

impl<'a> Responder<'a> for ErrorMessage {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Json(self).respond_to(req)
    }
}
