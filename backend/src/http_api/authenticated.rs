use rocket::Rocket;
use log::{info, warn, error};
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::http_api::auth::ProviderGuard;
use crate::db::{DBConn, DatabaseError};
use crate::http_api::AppointmentSuggestion;
use crate::db;

pub fn mount_endpoints(rocket: Rocket) -> Rocket {
    rocket.mount("/api", routes![add_appointments_by_id])
}


#[post("/bookings/id/<booking_id>", data = "<new_appointments>")]
pub fn add_appointments_by_id(provider: ProviderGuard, conn: DBConn, booking_id : i32, new_appointments: Json<Vec<AppointmentSuggestion>>) -> Result<(), Status> {
    
    info!(target: "POST /bookings/url/<booking_url>", "booking id {} appointments suggested", booking_id);

     match db::add_appointments_by_provider(&conn, booking_id, new_appointments.0, provider.person_id){
        Ok(_) => Ok(()),
        Err(DatabaseError::DieselError(e)) => {
            error!(target: "POST /bookings/url/<booking_url>", "database error for url {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        },
        Err(DatabaseError::Ambiguous) => {
            error!(target: "POST /bookings/url/<booking_url>", "multiple entries for url {}: {}", booking_id, DatabaseError::Ambiguous);
            Err(Status::InternalServerError)
        }
        Err(DatabaseError::NoEntry) => {
            warn!(target: "POST /bookings/url/<booking_url>", "no entries for url {}: {}", booking_id, DatabaseError::NoEntry);
            Err(Status::NotFound)
        },
        Err(e) => {
            warn!(target: "POST /bookings/<booking_url>", "undefined error for url {}: {}", booking_id, e);
            Err(Status::InternalServerError)
        },

    }
}