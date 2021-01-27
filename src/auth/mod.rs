use crate::{
    config::AppState,
    constants::{ACCESS_TOKEN_EXP, API_V1_BASE_PATH, AUTH_COOKIE_NAME},
};
use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use jwt::{errors::Error as JWTError, errors::ErrorKind as JWTErrorKind, DecodingKey, EncodingKey};
use rocket::http::{Cookie, Cookies, SameSite, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthError {
    InvalidSecret,
    MissingToken,
    InvalidToken(JWTErrorKind),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthClaims {
    pub id: i32, // user id
    pub iat: i64,
    pub exp: i64,
}

impl AuthClaims {
    pub fn new(user_id: i32) -> AuthClaims {
        AuthClaims {
            id: user_id,
            exp: (Utc::now() + Duration::minutes(ACCESS_TOKEN_EXP)).timestamp(),
            iat: Utc::now().timestamp(),
        }
    }

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
            let mut cookies = request.cookies();
            match extract_auth_claims(&cookies, &DecodingKey::from_secret(&state.secret)) {
                Ok(auth) => {
                    let token = AuthClaims::new(auth.id).create_token(&state.secret);
                    cookies.add(create_auth_cookie(token));
                    Outcome::Success(auth)
                }
                Err(e) => Outcome::Failure((Status::Forbidden, e)),
            }
        } else {
            Outcome::Failure((Status::InternalServerError, AuthError::InvalidSecret))
        }
    }
}

fn extract_auth_claims(cookies: &Cookies, secret: &DecodingKey) -> Result<AuthClaims, AuthError> {
    let cookie = cookies
        .get(AUTH_COOKIE_NAME)
        .ok_or(AuthError::MissingToken)?;

    decode_token(&cookie.value(), secret).map_err(|e| AuthError::InvalidToken(e.into_kind()))
}

fn decode_token(token: &str, secret: &DecodingKey) -> Result<AuthClaims, JWTError> {
    use jwt::{Algorithm, Validation};

    jwt::decode(token, secret, &Validation::new(Algorithm::HS256))
        .map(|token_data| token_data.claims)
}

pub fn create_auth_cookie(token: String) -> Cookie<'static> {
    Cookie::build(AUTH_COOKIE_NAME, token)
        .path(API_V1_BASE_PATH)
        .http_only(true)
        .max_age(Duration::seconds(ACCESS_TOKEN_EXP))
        .same_site(SameSite::Strict)
        .finish()
}
