use crate::config::{AppState, TOKEN_PREFIX};
use chrono::prelude::*;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthError {
    InvalidSecret,
    InvalidToken,
    TokenExpired,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthClaims {
    pub id: i32, // user id
    pub iat: i64,
    pub exp: i64,
}

impl AuthClaims {
    pub fn create_token(&self, secret: &[u8]) -> String {
        use jwt::{encode, Algorithm, Header};
        let header = Header::new(Algorithm::HS256);
        encode(&header, self, &EncodingKey::from_secret(secret)).expect("JWT cannot be created")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthClaims {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let state_outcome = request.guard::<State<AppState>>();
        if let Some(state) = state_outcome.succeeded() {
            let claims = extract_auth_claims(request, &DecodingKey::from_secret(&state.secret));
            let timestamp = Utc::now().timestamp();
            match claims {
                Some(auth) if auth.exp > timestamp => Outcome::Success(auth),
                Some(_) => Outcome::Failure((Status::Forbidden, AuthError::TokenExpired)),
                _ => Outcome::Failure((Status::Forbidden, AuthError::InvalidToken)),
            }
        } else {
            Outcome::Failure((Status::InternalServerError, AuthError::InvalidSecret))
        }
    }
}

fn extract_auth_claims(request: &Request, secret: &DecodingKey) -> Option<AuthClaims> {
    request
        .headers()
        .get_one("authorization")
        .and_then(|header| {
            if header.starts_with(TOKEN_PREFIX) {
                Some(&header[TOKEN_PREFIX.len()..])
            } else {
                None
            }
        })
        .and_then(|token| decode_token(token, secret))
}

fn decode_token(token: &str, secret: &DecodingKey) -> Option<AuthClaims> {
    use jwt::{Algorithm, Validation};

    jwt::decode(token, secret, &Validation::new(Algorithm::HS256))
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}
