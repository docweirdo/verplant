use pwhash::bcrypt;
use std::result::Result;

pub mod models;
pub mod schema;

use schema::providers::dsl::*;
use schema::persons::dsl::{persons, id};

#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);




pub fn verify_user(conn: DBConn, email: String, password: String) -> Result<(), ()> {


    let pw_hash = bcrypt::hash(password).unwrap();

    allow_tables_to_appear_in_same_query!(persons, providers);

    match persons.inner_join(providers.on(person_id.like(id)).select(password_hash)).load(&conn.0) {
        [pw_hash] => return Ok(None),
        _ => return Err(None)
    }

}