use log::{error, info, warn};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, patch, post, routes, Build, Rocket};

use crate::db;
use crate::db::models::Appointment;
use crate::db::{BookingReference, DBConn, DatabaseError};
use crate::http_api::auth::ProviderGuard;
use crate::http_api::AppointmentSuggestion;

pub fn mount_endpoints(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/api",
        routes![
            add_appointments_by_id,
            update_appointments_by_id,
            withdraw_appointments
        ],
    )
}

#[post("/bookings/id/<booking_id>", data = "<new_appointments>")]
pub async fn add_appointments_by_id(
    provider: ProviderGuard,
    conn: DBConn,
    booking_id: i32,
    new_appointments: Json<Vec<AppointmentSuggestion>>,
) -> Result<(), Status> {
    info!(target: "POST /bookings/url/<booking_url>", "booking id {} appointments suggested", booking_id);

    match db::add_appointments_by_provider(
        &conn,
        booking_id,
        new_appointments.0,
        provider.person_id,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "POST /bookings/url/<booking_url>", "database error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::Ambiguous) => {
            error!(target: "POST /bookings/url/<booking_url>", "multiple entries for id {}: {}", booking_id, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "POST /bookings/url/<booking_url>", "no entries for id {}: {}", booking_id, DatabaseError::NoEntry);
            Err(Status::NotFound)
        }
        Err(e) => {
            warn!(target: "POST /bookings/<booking_url>", "undefined error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
    }
}

#[patch("/bookings/id/<booking_id>", data = "<updated_appointments>")]
pub async fn update_appointments_by_id(
    provider_guard: ProviderGuard,
    conn: DBConn,
    booking_id: i32,
    updated_appointments: Json<Vec<Appointment>>,
) -> Result<(), Status> {
    info!(target: "PATCH /bookings/id/<booking_id>", "booking id {} appointments suggested", booking_id);

    match db::update_appointments(
        &conn,
        BookingReference::BookingId(booking_id),
        updated_appointments.0,
        Some(provider_guard.person_id),
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "PATCH /bookings/id/<booking_id>", "database error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::Ambiguous) => {
            error!(target: "PATCH /bookings/id/<booking_id>", "multiple entries for id {}: {}", booking_id, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "PATCH /bookings/id/<booking_id>", "no entries for id {}: {}", booking_id, DatabaseError::NoEntry);
            Err(Status::NotFound)
        }
        Err(DatabaseError::InvalidChange) => {
            warn!(target: "PATCH /bookings/id/<booking_id>", "requested invalid change for id {}: {}", booking_id, DatabaseError::InvalidChange);
            Err(Status::Forbidden)
        }
        Err(e) => {
            warn!(target: "PATCH /bookings/id/<booking_id>", "undefined error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/bookings/id/<booking_id>", data = "<withdrawn_appointments>")]
pub async fn withdraw_appointments(
    provider_guard: ProviderGuard,
    conn: DBConn,
    booking_id: i32,
    withdrawn_appointments: Json<Vec<i32>>,
) -> Result<(), Status> {
    info!(target: "DELETE /bookings/id/<booking_id>", "booking id {} appointments withdrawal suggested", booking_id);

    match db::withdraw_appointments(
        &conn,
        BookingReference::BookingId(booking_id),
        withdrawn_appointments.0,
        Some(provider_guard.person_id),
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "DELETE /bookings/id/<booking_id>", "database error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::Ambiguous) => {
            error!(target: "DELETE /bookings/id/<booking_id>", "multiple entries for id {}: {}", booking_id, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "DELETE /bookings/id/<booking_id>", "no entries for id {}: {}", booking_id, DatabaseError::NoEntry);
            Err(Status::NotFound)
        }
        Err(DatabaseError::InvalidChange) => {
            warn!(target: "DELETE /bookings/id/<booking_id>", "requested invalid withdrawal for id {}: {}", booking_id, DatabaseError::InvalidChange);
            Err(Status::Forbidden)
        }
        Err(e) => {
            warn!(target: "DELETE /bookings/id/<booking_id>", "undefined error for id {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        }
    }
}
