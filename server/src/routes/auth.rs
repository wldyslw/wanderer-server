use rocket::http::Cookies;
use rocket_contrib::json::Json;

use crate::{
    auth::{
        drop_session, remove_auth_cookie, set_auth_cookie, store_session, Auth, RedisConnection,
        Session,
    },
    db::{self, DBConnection},
    models::{
        user::{UserGet, UserLoginData},
        ErrorMessage,
    },
};

#[post("/sign-in", format = "json", data = "<login_data>")]
pub fn sign_in(
    login_data: Json<UserLoginData>,
    connection: DBConnection,
    redis_connection: RedisConnection,
    mut cookies: Cookies,
) -> Result<UserGet, ErrorMessage> {
    let user_login_data = login_data.into_inner();
    let result = db::users::login(
        &connection,
        &user_login_data.username,
        &user_login_data.password,
    );
    result.map(|user| {
        let session = Session::new(user.id);
        store_session(&redis_connection, &session);
        set_auth_cookie(&mut cookies, &session);
        user
    })
}

#[post("/sign-out")]
pub fn sign_out(
    redis_connection: RedisConnection,
    mut cookies: Cookies,
    auth: Auth,
) -> Result<(), ErrorMessage> {
    remove_auth_cookie(&mut cookies);
    drop_session(&redis_connection, &auth?.id);
    Ok(())
}
