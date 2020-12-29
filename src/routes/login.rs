use chrono::{Duration, Utc};
use rocket::State;
use rocket_contrib::json::JsonValue;

use crate::{
    auth::AuthClaims,
    config::{AppState, TOKEN_EXP},
};

#[get("/login")]
pub fn login(state: State<AppState>) -> JsonValue {
    let token = AuthClaims {
        exp: (Utc::now() + Duration::days(TOKEN_EXP)).timestamp(),
        iat: Utc::now().timestamp(),
    }
    .create_token(&state.secret);
    json!({ "token": token })
}
