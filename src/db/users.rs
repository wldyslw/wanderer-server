use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use scrypt::{scrypt_simple, ScryptParams};

use crate::models::user::{User, UserNew};
use crate::schema::users;

pub fn create(c: &PgConnection, username: String, password: String) -> Result<User, Error> {
    let hash = scrypt_simple(
        &password,
        &ScryptParams::new(14, 8, 1).expect("Invalid hashing params"),
    )
    .expect("Hashing error");

    diesel::insert_into(users::table)
        .values(UserNew::new(&username, &hash))
        .get_result::<User>(c)
}
