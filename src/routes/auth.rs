use rocket::http::Cookies;
use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    auth::{create_auth_cookie, AuthClaims},
    config::AppState,
    db::{self, DBConnection},
    models::{
        user::{UserGet, UserLoginData},
        ErrorMessage,
    },
};

#[post("/auth", format = "json", data = "<login_data>")]
pub fn auth(
    login_data: Json<UserLoginData>,
    c: DBConnection,
    state: State<AppState>,
    mut cookies: Cookies,
) -> Result<UserGet, ErrorMessage> {
    let user_login_data = login_data.into_inner();
    let result = db::users::login(&c, &user_login_data.username, &user_login_data.password);
    result.map(|user| {
        let token = AuthClaims::new(user.id).create_token(&state.secret);
        cookies.add(create_auth_cookie(token));
        user
    })
}
