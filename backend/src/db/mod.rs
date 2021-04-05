use diesel::{prelude::*, update};
use log::info;
use pwhash::bcrypt;
use rocket::Data;
use thiserror::Error;

pub mod models;
pub mod schema;

use models::*;
use schema::books::dsl::*;
use schema::courses::dsl::*;
use schema::persons::dsl::*;
use schema::providers::dsl::*;

#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);

/// Returns the id if user is verified
pub fn verify_user(conn: &DBConn, user_email: &str, password: &str) -> Result<i32, DatabaseError> {
    // Maybe change Query to one with inner join
    let user: Person = persons
        .filter(email.eq(user_email))
        .first::<Person>(&conn.0).map_err(diesel_to_no_entry)?;

    match Provider::belonging_to(&user)
        .select(password_hash)
        .first::<Option<String>>(&conn.0)?
    {
        Some(pw_hash) => {
            if bcrypt::verify(password, &pw_hash) {
                return Ok(user.id);
            } else {
                Err(DatabaseError::PasswordMatch)
            }
        }
        _ => Err(DatabaseError::NoEntry),
    }
}

pub fn set_password(conn: &DBConn, user_id: i32, password: &str) -> Result<(), DatabaseError> {
    let hash: String = bcrypt::hash(password).unwrap();

    update(providers.find(user_id))
        .set(password_hash.eq(hash))
        .execute(&conn.0)?;
    info!("Set password for user {}", user_id);

    Ok(())
}

pub fn is_user_admin(conn: DBConn, user_id: i32) -> Result<bool, DatabaseError> {
    let result: bool = providers
        .find(user_id)
        .select(is_admin)
        .first::<i32>(&conn.0)
        .map_err(diesel_to_no_entry)?
        != 0;
    Ok(result)
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    #[error("No entry found")]
    NoEntry,
    #[error("Passwords don't match")]
    PasswordMatch,
    #[error("Ambiguous result, too many entries")]
    Ambiguous,
}

/// This function maps Diesels not found error to our NoEntry error for easier matching
fn diesel_to_no_entry(err: diesel::result::Error) -> DatabaseError {
    match err {
        diesel::result::Error::NotFound => DatabaseError::NoEntry,
        _ => err.into(),
    }
}

/// returns the course name if successfull
pub fn get_booking_info(conn: &DBConn, booking_url: &str) -> Result<String, DatabaseError> {
    let mut result: Vec<String> = books
        .filter(url.eq(booking_url))
        .inner_join(courses.on(schema::courses::id.eq(course_id)))
        .select(schema::courses::name)
        .load::<String>(&conn.0)?;

    match result.len() {
        0 => return Err(DatabaseError::NoEntry),
        1 => return Ok(result.remove(0)),
        _ => return Err(DatabaseError::Ambiguous),
    }
}
