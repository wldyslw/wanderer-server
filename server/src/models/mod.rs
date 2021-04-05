use rocket::{
    http::Status,
    response::{self, Responder},
};
use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::io;

pub mod article;
pub mod user;
pub mod util;

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    code: i32,
    message: String,
    description: String,
}

impl ErrorMessage {
    pub fn new(code: i32, message: String, description: String) -> Self {
        ErrorMessage {
            code,
            message,
            description,
        }
    }
}

impl From<io::Error> for ErrorMessage {
    fn from(err: io::Error) -> Self {
        ErrorMessage::new(0, err.to_string(), "".to_string())
    }
}

impl<'a> Responder<'a> for ErrorMessage {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Json(self).respond_to(req)
    }
}

pub struct ContentLength(u64);

impl ContentLength {
    pub fn fits(self, bytes: u64) -> bool {
        self.0 <= bytes
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ContentLength {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let length_str = request.headers().get_one("Content-Length");
        if let Some(len) = length_str {
            let result = u64::from_str_radix(len, 10);
            match result {
                Ok(length) => Outcome::Success(ContentLength(length)),
                Err(_) => Outcome::Failure((Status::LengthRequired, ())),
            }
        } else {
            Outcome::Failure((Status::LengthRequired, ()))
        }
    }
}
