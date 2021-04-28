use chrono::Duration;
use rocket::http::{Cookie, Cookies, SameSite};

use crate::constants::{
    ACCESS_TOKEN_EXP, API_V1_BASE_PATH, IS_AUTHORIZED_COOKIE_NAME, SESSION_TOKEN_COOKIE_NAME,
};

use super::models::Session;

pub fn set_auth_cookie(cookies: &mut Cookies, session: &Session) {
    let session_token = Cookie::build(SESSION_TOKEN_COOKIE_NAME, session.id.clone())
        .path(API_V1_BASE_PATH)
        .http_only(true)
        .max_age(Duration::seconds(ACCESS_TOKEN_EXP))
        .same_site(SameSite::Strict)
        .finish();

    let is_authorized = Cookie::build(IS_AUTHORIZED_COOKIE_NAME, "true")
        .path(API_V1_BASE_PATH)
        .max_age(Duration::seconds(ACCESS_TOKEN_EXP))
        .same_site(SameSite::Strict)
        .finish();

    cookies.add(session_token);
    cookies.add(is_authorized);
}

pub fn remove_auth_cookie(cookies: &mut Cookies) {
    cookies.remove(Cookie::named(SESSION_TOKEN_COOKIE_NAME));
    cookies.remove(Cookie::named(IS_AUTHORIZED_COOKIE_NAME));
}

pub fn get_session_key(session_id: &str) -> String {
    format!("session:{}", session_id)
}
