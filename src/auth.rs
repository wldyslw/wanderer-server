use crate::config::{AppState, TOKEN_PREFIX};
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthClaims {
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
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthClaims, Self::Error> {
        let state: State<AppState> = request.guard()?;
        if let Some(auth) = extract_auth_claims(request, &DecodingKey::from_secret(&state.secret)) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
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
