use std::fmt;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use uuid::Uuid;

use crate::{constants::SESSION_TOKEN_COOKIE_NAME, models::ErrorMessage};

use super::{
    db::{drop_session, find_session, store_session, RedisConnection},
    utils::set_auth_cookie,
};

pub struct Session {
    pub id: String,
    pub user_id: i32,
}

impl Session {
    pub fn new(user_id: i32) -> Self {
        Session {
            id: Uuid::new_v4().to_simple().to_string(),
            user_id,
        }
    }

    pub fn existing(id: &str, user_id: i32) -> Self {
        Session {
            id: String::from(id),
            user_id,
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    InternalError,
    MissingSessionId,
    NonexistentOrExpiredSession,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            &AuthError::InternalError => "Internal server error: cannot get Redis connection",
            &AuthError::MissingSessionId => "Missing Session ID",
            &AuthError::NonexistentOrExpiredSession => "Session has expired or does not exist",
        };
        write!(f, "{}", message)
    }
}

impl From<AuthError> for ErrorMessage {
    fn from(error: AuthError) -> Self {
        ErrorMessage::new(Default::default(), error.to_string(), "".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let connection_outcome = request.guard::<RedisConnection>();
        if let Some(connection) = connection_outcome.succeeded() {
            let mut cookies = request.cookies();
            if let Some(cookie) = cookies.get(SESSION_TOKEN_COOKIE_NAME) {
                match find_session(&connection, cookie.value()) {
                    Ok(session) => {
                        drop_session(&connection, &session.id);
                        let new_session = Session::new(session.user_id);
                        store_session(&connection, &new_session);
                        set_auth_cookie(&mut cookies, &new_session);
                        Outcome::Success(new_session)
                    }
                    Err(e) => Outcome::Failure((Status::Forbidden, e)),
                }
            } else {
                Outcome::Failure((Status::Forbidden, AuthError::MissingSessionId))
            }
        } else {
            Outcome::Failure((Status::InternalServerError, AuthError::InternalError))
        }
    }
}

pub type Auth = Result<Session, AuthError>;
