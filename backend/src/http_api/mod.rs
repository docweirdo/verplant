use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use log::{error, info, warn};
use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

pub mod auth;

use crate::db;
use auth::ProviderGuard;
use db::{models, schema, DBConn, DatabaseError};
use models::Appointment;
use schema::courses::dsl::*;


pub fn mount_endpoints(rocket: Rocket) -> Rocket {
    rocket.mount("/api/", routes![
        get_courses, 
        get_provider_appointments, 
        get_booking_info, 
        get_booking_appointments, 
        add_appointments])
}


#[get("/courses")]
pub fn get_courses(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses
        .load::<models::Course>(&conn.0)
        .expect("Error loading courses");
    Json(results)
}

// Todo: write actual function
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


#[get("/bookings/<booking_url>/appointments")]
pub fn get_booking_appointments(provider: Option<ProviderGuard>, booking_url : String, conn: DBConn) -> Result<Json<Vec<Appointment>>, Status>{
    info!(target: "/bookings/<booking_url>/appointments", "booking url {} appointments requested", booking_url);

    let (mut appointments, person_id) : (Vec<Appointment>, i32) = match db::get_booking_appointments_from_url(&conn, &booking_url){
        Ok(r) => r,
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "/bookings/<booking_url>/appointments", "get_booking_appointments_from_url database error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "/bookings/<booking_url>/appointments", "get_booking_appointments_from_url multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "/bookings/<booking_url>/appointments", "get_booking_appointments_from_url no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            return Err(Status::NotFound);
        },
        Err(e) => {
            warn!(target: "/bookings/<booking_url>/appointments", "get_booking_appointments_from_url undefined error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        },

    };

    let status_suggested = "SUGGESTED";
    let our_id: i32 = provider.map(|p| p.person_id).unwrap_or(person_id);

    for appointment in appointments.iter_mut() {
        if appointment.proposer_id == our_id && status_suggested.eq(&appointment.state){
            appointment.state = String::from("PENDING");
        }
    }

    Ok(Json(appointments))
}

#[derive(Deserialize, Debug)]
pub struct AppointmentSuggestion {
    pub date: String, 
    pub start_time: String,
    pub end_time: String,
    pub room_id: Option<i32>
}

#[post("/bookings/<booking_url>", data = "<new_appointments>")]
pub fn add_appointments(provider: Option<ProviderGuard>, conn: DBConn, booking_url : String, new_appointments: Json<Vec<AppointmentSuggestion>>) -> Result<(), Status> {
    
    info!(target: "POST /bookings/<booking_url>", "booking url {} appointments suggested", booking_url);

     match db::add_appointments(&conn, &booking_url, new_appointments.0, provider.map(|p| p.person_id)){
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "/bookings/<booking_url>", "add_appointments database error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "/bookings/<booking_url>", "add_appointments multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "/bookings/<booking_url>", "add_appointments no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            Err(Status::NotFound)
        },
        Err(e) => {
            warn!(target: "/bookings/<booking_url>", "add_appointments undefined error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },

    }
}

#[patch("/bookings/<booking_url>", data = "<updated_appointments>")]
pub fn update_appointments(provider: Option<ProviderGuard>, conn: DBConn, booking_url : String, updated_appointments: Json<Vec<Appointment>>) -> Result<(), Status> {
    
    info!(target: "PATCH /bookings/<booking_url>", "booking url {} appointments modification suggested", booking_url);

     match db::update_appointments(&conn, &booking_url, updated_appointments.0, provider.map(|p| p.person_id)){
        Ok(_) => Ok(()),
        // Special Error Handling
        Err(e) => {
            warn!(target: "PATCH /bookings/<booking_url>", "update_appointments undefined error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },

    }
}

#[delete("/bookings/<booking_url>", data = "<withdrawn_appointments>")]
pub fn withdraw_appointments(provider: Option<ProviderGuard>, conn: DBConn, booking_url : String, withdrawn_appointments: Json<Vec<Appointment>>) -> Result<(), Status> {
    
    info!(target: "DELETE /bookings/<booking_url>", "booking url {} appointments withdrawal suggested", booking_url);

     match db::withdraw_appointments(&conn, &booking_url, withdrawn_appointments.0, provider.map(|p| p.person_id)){
        Ok(_) => Ok(()),
        // Special Error Handling
        Err(e) => {
            warn!(target: "DELETE /bookings/<booking_url>", "withdraw_appointments undefined error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },

    }
}


