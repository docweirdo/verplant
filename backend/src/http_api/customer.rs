use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use log::{error, info, warn};
use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};


use crate::db;
use db::{models, schema, DBConn, DatabaseError, BookingReference};
use models::Appointment;
use schema::courses::dsl::*;
use crate::http_api::AppointmentSuggestion;

pub fn mount_endpoints(rocket: Rocket) -> Rocket {
    rocket.mount("/api/", routes![
        get_courses, 
        get_booking_info_by_url, 
        get_appointments_by_url, 
        add_appointments_by_url,
        update_appointments_by_url])
}


#[get("/courses")]
pub fn get_courses(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses
        .load::<models::Course>(&conn.0)
        .expect("Error loading courses");
    Json(results)
}

/// Sends back info corresponding to booking_url. At the moment only the name of the course. TODO: return ContactInformation
#[get("/bookings/url/<booking_url>/info")]
pub fn get_booking_info_by_url(
    conn: DBConn,
    booking_url: String,
) -> Result<JsonValue, Status> {
    info!(target: "/bookings/url/<booking_url>/info", "booking url {} info requested", booking_url);

    match db::get_booking_info(&conn, &booking_url) {
        Ok(course_name) => Ok(json!({ "selectedCourse": course_name })), // TODO: return more
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "/bookings/url/<booking_url>/info", "database error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::Ambiguous) => {
            error!(target: "/bookings/url/<booking_url>/info", "multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "/bookings/url/<booking_url>/info", "no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            return Err(Status::NotFound);
        }
        Err(e) => {
            error!(target: "/bookings/url/<booking_url>/info", "undefined error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        }
    }
}


#[get("/bookings/url/<booking_url>/appointments")]
pub fn get_appointments_by_url(booking_url : String, conn: DBConn) -> Result<Json<Vec<Appointment>>, Status>{
    info!(target: "/bookings/url/<booking_url>/appointments", "booking url {} appointments requested", booking_url);

    let (mut appointments, person_id) : (Vec<Appointment>, i32) = match db::get_booking_appointments_by_url(&conn, &booking_url){
        Ok(r) => r,
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "/bookings/url/<booking_url>/appointments", "database error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "/bookings/url/<booking_url>/appointments", "multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            return Err(Status::InternalServerError);
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "/bookings/url/<booking_url>/appointments", "no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            return Err(Status::NotFound);
        },
        Err(e) => {
            warn!(target: "/bookings/url/<booking_url>/appointments", "undefined error for url {}: {}", booking_url, e);
            return Err(Status::InternalServerError);
        },

    };

    let status_suggested = "SUGGESTED";

    for appointment in appointments.iter_mut() {
        if appointment.proposer_id == person_id && status_suggested.eq(&appointment.state){
            appointment.state = String::from("PENDING");
        }
    }

    Ok(Json(appointments))
}


#[post("/bookings/url/<booking_url>", data = "<new_appointments>")]
pub fn add_appointments_by_url(conn: DBConn, booking_url : String, new_appointments: Json<Vec<AppointmentSuggestion>>) -> Result<(), Status> {
    
    info!(target: "POST /bookings/url/<booking_url>", "booking url {} appointments suggested", booking_url);

     match db::add_appointments_by_customer(&conn, &booking_url, new_appointments.0){
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "POST /bookings/url/<booking_url>", "database error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "POST /bookings/url/<booking_url>", "multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "POST /bookings/url/<booking_url>", "no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            Err(Status::NotFound)
        },
        Err(e) => {
            warn!(target: "POST /bookings/<booking_url>", "undefined error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },

    }
}

#[patch("/bookings/url/<booking_url>", data = "<updated_appointments>")]
pub fn update_appointments_by_url(conn: DBConn, booking_url : String, updated_appointments: Json<Vec<Appointment>>) -> Result<(), Status> {
    
    info!(target: "PATCH /bookings/url/<booking_url>", "booking url {} appointments modification suggested", booking_url);

     match db::update_appointments(&conn, BookingReference::BookingUrl(&booking_url), updated_appointments.0, None){
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "PATCH /bookings/url/<booking_url>", "database error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "PATCH /bookings/url/<booking_url>", "multiple entries for url {}: {}", booking_url, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "PATCH /bookings/url/<booking_url>", "no entries for url {}: {}", booking_url, DatabaseError::NoEntry);
            Err(Status::NotFound)
        },
        Err(DatabaseError::InvalidChange) => {
            warn!(target: "PATCH /bookings/url/<booking_url>", "requested invalid change for url {}: {}", booking_url, DatabaseError::NoEntry);
            Err(Status::BadRequest)
        }
        Err(e) => {
            warn!(target: "PATCH /bookings/url/<booking_url>", "undefined error for url {}: {}", booking_url, e);
            Err(Status::InternalServerError)
        },

    }
}

/* 
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
} */


