use chrono::{Duration, Utc};
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use crate::{
    auth::AuthClaims,
    config::AppState,
    constants::TOKEN_EXP,
    db::{self, DBConnection},
    models::user::UserLoginData,
};

#[post("/login", format = "json", data = "<login_data>")]
pub fn login(
    login_data: Json<UserLoginData>,
    c: DBConnection,
    state: State<AppState>,
    mut cookies: Cookies,
) -> JsonValue {
    let user_login_data = login_data.into_inner();
    let result = db::users::login(&c, &user_login_data.username, &user_login_data.password);
    match result {
        Ok(user) => {
            let token = AuthClaims {
                id: user.id,
                exp: (Utc::now() + Duration::minutes(TOKEN_EXP)).timestamp(),
                iat: Utc::now().timestamp(),
            }
            .create_token(&state.secret);

            cookies.add(Cookie::new("refresh-token", "")); // TODO: provide refresh token here

            json!({ "token": token })
        }
        Err(error) => {
            json!({ "error": error.to_string() })
        }
    }
}
