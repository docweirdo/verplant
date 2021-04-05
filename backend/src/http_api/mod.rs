use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use log::{info, warn};
pub mod auth;

use crate::db::{DBConn, models, schema};
use schema::courses::dsl::*;
use auth::ProviderGuard;
use models::Appointment;

#[get("/courses")]
pub fn get_courses(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses.load::<models::Course>(&conn.0).expect("Error loading courses");
    Json(results)
}


#[get("/provider/<provider_id>/appointments")]
pub fn get_provider_appointments(provider: ProviderGuard, provider_id: i32) -> String {
    info!(target: "/provider/<provider_id>/appointments", "provider number {} appointments requested", provider_id);
    return format!("{}", provider.is_admin);
}

/// Sends back info corresponding to booking_url. At the moment only the name of the course.
#[get("/bookings/<booking_url>/info")]
pub fn get_booking_info(provider: Option<ProviderGuard>, booking_url : String) -> Result<JsonValue, Status>{
    info!(target: "/bookings/<booking_url>/info", "booking url {} info requested", booking_url);

    match db::get_booking_info(booking_url) {
        Ok(course_name) => {
            Ok(json!({
                "selectedCourse": course_name
            }))
        },
        Err(e) => {
            warn!(target: "/bookings/<booking_url>/info", "get_booking_info database-error for url: {}", booking_url);
            return Err(Status::NotFound)
        },
        Err() => {

        }
    }
}   

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
        if appointment.person_id == provider.person_id && status_suggested.eq(&appointment.status){
            appointment.status = String::from("PENDING");
        }
    }

    Ok(Json(appointments))
}

