use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use log::{error, info, warn};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

pub mod auth;

use crate::db;
use auth::ProviderGuard;
use db::{models, schema, DBConn, DatabaseError};
use models::Appointment;
use schema::courses::dsl::*;

#[get("/courses")]
pub fn get_courses(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses
        .load::<models::Course>(&conn.0)
        .expect("Error loading courses");
    Json(results)
}

#[get("/provider/<provider_id>/appointments")]
pub fn get_provider_appointments(provider: ProviderGuard, provider_id: i32) -> String {
    info!(target: "/provider/<provider_id>/appointments", "provider number {} appointments requested", provider_id);
    return format!("{}", provider.is_admin);
}

/// Sends back info corresponding to booking_url. At the moment only the name of the course.
#[get("/bookings/<booking_url>/info")]
pub fn get_booking_info(
    conn: DBConn,
    booking_url: String,
) -> Result<JsonValue, Status> {
    info!(target: "/bookings/<booking_url>/info", "booking url {} info requested", booking_url);

    match db::get_booking_info(&conn, &booking_url) {
        Ok(course_name) => Ok(json!({ "selectedCourse": course_name })),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "/bookings/<booking_url>/info", "get_booking_info database error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::Ambiguous) => {
            error!(target: "/bookings/<booking_url>/info", "get_booking_info multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "/bookings/<booking_url>/info", "get_booking_info no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            return Err(Status::NotFound);
        }
        Err(e) => {
            error!(target: "/bookings/<booking_url>/info", "get_booking_info undefined error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        }
    }
}

/*
#[get("/bookings/<booking_url>/appointments")]
pub fn get_booking_appointments(provider: Option<ProviderGuard>, booking_url : String) -> Result<Json<Vec<Appointment>>, Status>{
    info!(target: "/bookings/<booking_url>/appointments", "booking url {} appointments requested", booking_url);

    let appointments : Vec<Appointment> = if let Some(provider_guard) = provider {
        db::get_booking_appointments_from_id(provider_guard.person_id)
    } else {
        db::get_booking_appointments_from_url(booking_url)
    };

    let status_suggested = "SUGGESTED";

    for appointment in appointments.iter_mut() {
        if appointment.proposer_id == provider_guard.person_id && status_suggested.eq(&appointment.status){
            appointment.status = String::from("PENDING");
        }
    }

    Ok(Json(appointments))
}
*/
