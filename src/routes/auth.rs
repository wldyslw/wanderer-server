use rocket::http::Cookies;
use rocket_contrib::json::Json;

use crate::{
    auth::{set_auth_cookie, store as store_session, RedisConnection, Session},
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
    redis_connection: RedisConnection,
    mut cookies: Cookies,
) -> Result<UserGet, ErrorMessage> {
    let user_login_data = login_data.into_inner();
    let result = db::users::login(&c, &user_login_data.username, &user_login_data.password);
    result.map(|user| {
        let session = Session::new(user.id);
        store_session(&redis_connection, &session);
        set_auth_cookie(&mut cookies, &session);
        user
    })
}
