use chrono::Duration;
use rocket::http::{Cookie, Cookies, SameSite};

use crate::constants::{ACCESS_TOKEN_EXP, API_V1_BASE_PATH, AUTH_COOKIE_NAME};

use super::models::Session;

pub fn set_auth_cookie(cookies: &mut Cookies, session: &Session) {
    let cookie = Cookie::build(AUTH_COOKIE_NAME, session.id.clone())
        .path(API_V1_BASE_PATH)
        .http_only(true)
        .max_age(Duration::seconds(ACCESS_TOKEN_EXP))
        .same_site(SameSite::Strict)
        .finish();

    cookies.add(cookie);
}

pub fn get_session_key(session_id: &str) -> String {
    format!("session:{}", session_id)
}
