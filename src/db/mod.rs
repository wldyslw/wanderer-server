use diesel::result::Error;
use rocket_contrib::databases::diesel;

use crate::models::ErrorMessage;

pub mod articles;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct DBConnection(diesel::PgConnection);

impl From<Error> for ErrorMessage {
    fn from(err: Error) -> ErrorMessage {
        ErrorMessage::new(i32::default(), err.to_string(), err.to_string())
    }
}
