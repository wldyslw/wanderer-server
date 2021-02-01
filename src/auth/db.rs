use rocket_contrib::databases::{
    database,
    redis::{Commands, Connection},
};

use crate::constants::ACCESS_TOKEN_EXP;

use super::{
    models::{AuthError, Session},
    utils::get_session_key,
};

#[database("redis_pool")]
pub struct RedisConnection(Connection);

pub fn find(connection: &Connection, session_id: &str) -> Result<Session, AuthError> {
    connection
        .get::<&str, i32>(&get_session_key(session_id))
        .map(|user_id| Session::existing(session_id, user_id))
        .or(Err(AuthError::NonexistentOrExpiredSession))
}

pub fn drop(connection: &Connection, session_id: &str) {
    let _: Option<()> = connection.del(&get_session_key(session_id)).ok();
}

pub fn store(connection: &Connection, session: &Session) -> Option<()> {
    connection
        .set_ex(
            &get_session_key(&session.id),
            session.user_id,
            ACCESS_TOKEN_EXP as usize,
        )
        .ok()
}
