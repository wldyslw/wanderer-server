use rocket_contrib::databases::diesel;

pub mod articles;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct DBConnection(diesel::PgConnection);
