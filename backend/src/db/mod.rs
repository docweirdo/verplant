use diesel::{prelude::*, update, insert_into};
use log::info;
use pwhash::bcrypt;
use thiserror::Error;

pub mod models;
pub mod schema;
pub mod insertable;

use models::*;
use insertable::*;
use schema::appointments::dsl::*;
use schema::books::dsl::*;
use schema::courses::dsl::*;
use schema::persons::dsl::*;
use schema::providers::dsl::*;
use crate::http_api::AppointmentSuggestion;

#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);

/// Returns the id if user is verified
pub fn verify_user(conn: &DBConn, user_email: &str, password: &str) -> Result<i32, DatabaseError> {
    // Maybe change Query to one with inner join
    let user: Person = persons
        .filter(email.eq(user_email))
        .first::<Person>(&conn.0)
        .map_err(diesel_to_no_entry)?;

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
pub fn diesel_to_no_entry(err: diesel::result::Error) -> DatabaseError {
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

/// loads the appointments and the customer associated with the booking_url
pub fn get_booking_appointments_from_url(
    conn: &DBConn,
    booking_url: &str,
) -> Result<(Vec<Appointment>, i32), DatabaseError> {
    
    let customer: i32 = match books
        .filter(url.eq(booking_url))
        .select(customer_id)
        .load::<i32>(&conn.0) {
            Ok(v) if v.len() == 0 => return Err(DatabaseError::NoEntry),
            Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
            Ok(mut v) => v.remove(0),
            Err(e) => return Err(e.into())
        };

    let appointments_vec: Vec<Appointment> = books
        .filter(url.eq(booking_url))
        .inner_join(appointments.on(schema::appointments::books_id.eq(schema::books::id)))
        .select((
            schema::appointments::id,
            schema::appointments::date,
            schema::appointments::start_time,
            schema::appointments::end_time,
            schema::appointments::state,
            schema::appointments::proposer_id,
        ))
        .load::<Appointment>(&conn.0)
        .map_err(diesel_to_no_entry)?;

    Ok((appointments_vec, customer))
}


pub fn add_appointments(conn: &DBConn, booking_url : &str, mut appointment_list : Vec<AppointmentSuggestion>, provider: Option<i32>) -> Result<(), DatabaseError>{

    conn.transaction(move || {

        let (customer, booking_id): (i32, i32) = match books
        .filter(url.eq(booking_url))
        .select((customer_id, schema::books::id))
        .load::<(i32, i32)>(&conn.0) {
            Ok(v) if v.len() == 0 => return Err(DatabaseError::NoEntry),
            Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
            Ok(mut v) => v.remove(0),
            Err(e) => return Err(e.into())
        };

        let room : Option<i32> = match books.inner_join(courses.on(schema::courses::id.eq(course_id))).select(default_room_id).load(&conn.0) {
            Ok(v) if v.len() == 0 => return Err(DatabaseError::NoEntry),
            Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
            Ok(mut v) => v.remove(0),
            Err(e) => return Err(e.into())
        };

        let appt = appointment_list.drain(..).map(|sug| {
            NewAppointment {
                date: sug.date,
                start_time: sug.starttime,
                end_time: sug.endtime,
                state: String::from("SUGGESTED"),
                proposer_id: provider.unwrap_or(customer),
                books_id: booking_id,
                room_id: room
            }
        });

        for a in appt {
            insert_into(appointments).values(a).execute(&conn.0).map_err(diesel_to_no_entry)?;
        }
        Ok(())
    })

}
