use log::{error, info, trace, warn};
use pwhash::bcrypt;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, update};
use thiserror::Error;

pub mod insertable;
pub mod models;
pub mod schema;

use crate::http_api::AppointmentSuggestion;
use insertable::*;
use models::*;

use schema::appointments::dsl as appointment_dsl;
use schema::books::dsl as booking_dsl;
use schema::courses::dsl as course_dsl;
use schema::customers::dsl as customer_dsl;
use schema::persons::dsl as person_dsl;
use schema::providers::dsl as provider_dsl;

#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);

/// Returns the id if user is verified
pub async fn verify_user(
    conn: &DBConn,
    user_email: &str,
    password: &str,
) -> Result<i32, DatabaseError> {
    let user_email = user_email.to_owned();
    let password = password.to_owned();

    conn.run(move |c| {
        match person_dsl::persons
            .filter(person_dsl::email.eq(&user_email))
            .inner_join(provider_dsl::providers)
            .select((provider_dsl::password_hash, provider_dsl::person_id))
            .first::<(Option<String>, i32)>(c)
            .map_err(diesel_to_no_entry)?
        {
            (Some(pw_hash), person_id) => {
                if bcrypt::verify(password, &pw_hash) {
                    Ok(person_id)
                } else {
                    let error = DatabaseError::PasswordMatch;
                    info!(
                        "wrong password for email {}: {}",
                        user_email,
                        DatabaseError::PasswordMatch
                    );
                    Err(error)
                }
            }
            _ => {
                let error = DatabaseError::NoEntry;
                warn!("no password for email {}: {}", user_email, error);
                Err(error)
            }
        }
    })
    .await
}

pub async fn set_password(
    conn: &DBConn,
    user_id: i32,
    password: &str,
) -> Result<(), DatabaseError> {
    let hash: String = bcrypt::hash(password).unwrap();

    conn.run(move |c| {
        update(provider_dsl::providers.find(user_id))
            .set(provider_dsl::password_hash.eq(hash))
            .execute(c)?;
        info!("Set password for user {}", user_id);

        Ok(())
    })
    .await
}

pub async fn is_user_admin(conn: DBConn, user_id: i32) -> Result<bool, DatabaseError> {
    conn.run(move |c| {
        let result: bool = provider_dsl::providers
            .find(user_id)
            .select(provider_dsl::is_admin)
            .first::<i32>(c)
            .map_err(diesel_to_no_entry)?
            != 0;
        Ok(result)
    })
    .await
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

    #[error("Update invalid, unallowed changes or inconsistent data")]
    InvalidChange,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DatabaseError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let status = match self {
            DatabaseError::NoEntry => Status::NotFound,
            DatabaseError::InvalidChange => Status::Forbidden,
            _ => Status::InternalServerError,
        };
        status.respond_to(req)
    }
}

/// This function maps Diesels not found error to our NoEntry error for easier matching
pub fn diesel_to_no_entry(err: diesel::result::Error) -> DatabaseError {
    match err {
        diesel::result::Error::NotFound => DatabaseError::NoEntry,
        _ => err.into(),
    }
}

/// returns the course info for frontend if successful
pub async fn get_booking_info(
    conn: &DBConn,
    booking_url: &str,
) -> Result<BookingInfo, DatabaseError> {
    let booking_url = booking_url.to_owned();

    conn.run(move |c| {
        c.transaction(|c| {
            let mut result: Vec<BookingInfo> = booking_dsl::books
                .filter(booking_dsl::url.eq(booking_url))
                .inner_join(customer_dsl::customers.inner_join(person_dsl::persons))
                .select((
                    person_dsl::firstname,
                    person_dsl::lastname,
                    person_dsl::email,
                    person_dsl::phone,
                    customer_dsl::organisation,
                    customer_dsl::class,
                    booking_dsl::course_id,
                ))
                .load::<BookingInfo>(c)?;

            match result.len() {
                0 => Err(DatabaseError::NoEntry),
                1 => Ok(result.remove(0)),
                _ => Err(DatabaseError::Ambiguous),
            }
        })
    })
    .await
}

/// loads the appointments and the customer associated with the booking_url
pub async fn get_booking_appointments_by_url(
    conn: &DBConn,
    booking_url: &str,
) -> Result<(Vec<Appointment>, i32), DatabaseError> {
    let booking_url = booking_url.to_owned();

    conn.run(|c| {
        c.transaction(|c| {
            let customer: i32 = match booking_dsl::books
                .filter(booking_dsl::url.eq(&booking_url))
                .select(booking_dsl::customer_id)
                .load::<i32>(c)
            {
                Ok(v) if v.is_empty() => return Err(DatabaseError::NoEntry),
                Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
                Ok(mut v) => v.remove(0),
                Err(e) => return Err(e.into()),
            };

            let appointments_vec: Vec<Appointment> = booking_dsl::books
                .filter(booking_dsl::url.eq(booking_url))
                .inner_join(appointment_dsl::appointments)
                .select(Appointment::as_select())
                .load::<Appointment>(c)
                .map_err(diesel_to_no_entry)?;

            Ok((appointments_vec, customer))
        })
    })
    .await
}

/// returns (customer, booking id) on success
fn get_booking_details(
    conn: &mut SqliteConnection,
    booking_ref: BookingReference,
) -> Result<(i32, i32), DatabaseError> {
    let mut query = booking_dsl::books.into_boxed();

    query = match booking_ref {
        BookingReference::BookingId(booking_id) => query.filter(booking_dsl::id.eq(booking_id)),
        BookingReference::BookingUrl(booking_url) => query.filter(booking_dsl::url.eq(booking_url)),
    };

    let (customer, booking_id): (i32, i32) = match query
        .select((booking_dsl::customer_id, booking_dsl::id))
        .load::<(i32, i32)>(conn)
    {
        Ok(v) if v.is_empty() => return Err(DatabaseError::NoEntry),
        Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
        Ok(mut v) => v.remove(0),
        Err(e) => return Err(e.into()),
    };

    Ok((customer, booking_id))
}

fn get_default_room_id_by_booking_id(
    conn: &mut SqliteConnection,
    booking_id: i32,
) -> Result<i32, DatabaseError> {
    let room: i32 = match booking_dsl::books
        .inner_join(course_dsl::courses)
        .filter(booking_dsl::id.eq(booking_id))
        .select(course_dsl::default_room_id)
        .load(conn)
    {
        Ok(v) if v.is_empty() => return Err(DatabaseError::NoEntry),
        Ok(v) if v.len() > 1 => return Err(DatabaseError::Ambiguous),
        Ok(mut v) => v.remove(0),
        Err(e) => return Err(e.into()),
    };

    Ok(room)
}

pub async fn add_appointments_by_customer(
    conn: &DBConn,
    booking_url: &str,
    mut appointment_list: Vec<AppointmentSuggestion>,
) -> Result<(), DatabaseError> {
    let booking_url = booking_url.to_owned();

    conn.run(move |c| {
        c.transaction(|c| {
            let (customer, booking_id) =
                get_booking_details(c, BookingReference::BookingUrl(booking_url))?;
            let room = get_default_room_id_by_booking_id(c, booking_id)?;
            let appt = appointment_list.drain(..).map(|sug| NewAppointment {
                start_time: sug.start_time,
                end_time: sug.end_time,
                state: String::from("SUGGESTED"),
                proposer_id: customer,
                books_id: booking_id,
                room_id: room,
            });

            for a in appt {
                insert_into(appointment_dsl::appointments)
                    .values(a)
                    .execute(c)
                    .map_err(diesel_to_no_entry)?;
            }
            Ok(())
        })
    })
    .await
}

pub async fn add_appointments_by_provider(
    conn: &DBConn,
    booking_id: i32,
    mut appointment_list: Vec<AppointmentSuggestion>,
    provider: i32,
) -> Result<(), DatabaseError> {
    use schema::appointments::dsl::*;

    conn.run(move |c| {
        c.transaction(move |c| {
            let room = get_default_room_id_by_booking_id(c, booking_id)?;

            let appt = appointment_list.drain(..).map(|sug| NewAppointment {
                start_time: sug.start_time,
                end_time: sug.end_time,
                state: String::from("SUGGESTED"),
                proposer_id: provider,
                books_id: booking_id,
                room_id: room,
            });

            for a in appt {
                insert_into(appointments)
                    .values(a)
                    .execute(c)
                    .map_err(diesel_to_no_entry)?;
            }
            Ok(())
        })
    })
    .await
}

fn other_appointment_fields_changed(old: &Appointment, new: &Appointment) -> bool {
    !(old.start_time == new.start_time
        && old.end_time == new.end_time
        && old.proposer_id == new.proposer_id
        && old.books_id == new.books_id)
}

/// Enum used to pass one of the two types referencing a Booking Process.  
/// Used when a Database Function is built Provider/Customer agnostic.
#[derive(Clone)]
pub enum BookingReference {
    BookingId(i32),
    BookingUrl(String),
}

/// Cycles through the list of provided appointments that are to be updated,    
/// checks each for existence, validity of updated values and authorization    
/// for updating the specific fields. If all checks out, all changes get written    
/// to the database. If not, the transaction gets canceled and a [DatabaseError] is returned.
pub async fn update_appointments(
    conn: &DBConn,
    booking_ref: BookingReference,
    appointment_list: Vec<Appointment>,
    provider: Option<i32>,
) -> Result<(), DatabaseError> {
    conn.run(move |c|{
        c.transaction(|c| {

            let (customer, _) = get_booking_details(c, booking_ref)?;

            trace!(target: "db::update_appointments", "found customer {}", customer);

            for updated_appointment in appointment_list {

                trace!(target: "db::update_appointments", "searching for appointment {}", updated_appointment.id);

                let old_appointment : Appointment = appointment_dsl::appointments.find(updated_appointment.id).first::<Appointment>(c).map_err(diesel_to_no_entry)?;

                trace!(target: "db::update_appointments", "found appointment {}", updated_appointment.id);

                // only allow room_id or state to change
                if other_appointment_fields_changed(&old_appointment, &updated_appointment) {
                    return Err(DatabaseError::InvalidChange);
                }

                // only allow update of state if current state is "SUGGESTED"
                if old_appointment.state != "SUGGESTED" {
                    return Err(DatabaseError::InvalidChange);
                }

                // return Error if room was changed and request is not made by a provider
                // otherwise check if ID of proposer is different than requesting person ID
                if let Some(provider_id) = provider {
                    if provider_id == old_appointment.proposer_id {
                        return Err(DatabaseError::InvalidChange);
                    }
                } else if (updated_appointment.room_id != old_appointment.room_id) || (customer == old_appointment.proposer_id){
                    return Err(DatabaseError::InvalidChange);}

                // check if state is changed to one of the two valid states following "SUGGESTED"
                if updated_appointment.state != "REJECTED" && updated_appointment.state != "APPROVED"{
                    return Err(DatabaseError::InvalidChange);
                }

                diesel::update(&old_appointment).set(&updated_appointment).execute(c)?;

                trace!(target: "db::update_appointments", "updated appointment {}", updated_appointment.id);
            }


            Ok(())
        })
    }).await
}

pub async fn withdraw_appointments(
    conn: &DBConn,
    booking_ref: BookingReference,
    withdrawn_appointments: Vec<i32>,
    provider: Option<i32>,
) -> Result<(), DatabaseError> {
    use schema::appointments::dsl::*;

    conn.run(move |c|{
        c.transaction(|c| {

            let (customer, _) = get_booking_details(c, booking_ref)?;

            let person = provider.unwrap_or(customer);

            trace!(target: "db::withdraw_appointments", "found customer {}", customer);

            for withdrawn_appointment in withdrawn_appointments {
                trace!(target: "db::withdraw_appointments", "searching for appointment {}", withdrawn_appointment);

                let old_appointment: Appointment = match appointments
                    .find(withdrawn_appointment)
                    .first::<Appointment>(c)
                    .map_err(diesel_to_no_entry)
                {
                    Ok(a) => a,
                    Err(e) => return Err(e),
                };

                trace!(target: "db::withdraw_appointments", "found appointment {}", withdrawn_appointment);

                // only allow withdrawal if current state is "SUGGESTED"
                if old_appointment.state != "SUGGESTED" {
                    return Err(DatabaseError::InvalidChange);
                }

                if old_appointment.proposer_id != person {
                    return Err(DatabaseError::InvalidChange);
                }

                match diesel::delete(appointments.filter(id.eq(withdrawn_appointment))).execute(c)
                {
                    Ok(1) => {}
                    Ok(0) => {
                        error!(target: "db::withdraw_appointments", "appointment {} deletion req. checked out but row not found", withdrawn_appointment);
                        return Err(DatabaseError::NoEntry);
                    }
                    Ok(_) => {
                        error!(target: "db::withdraw_appointments", "appointment {} deletion req. checked out but multiple rows", withdrawn_appointment);
                        return Err(DatabaseError::Ambiguous);
                    }
                    Err(e) => return Err(e.into()),
                }
            }

            Ok(())
        })
    }).await
}
