use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use rocket_contrib::json::Json;
use log::info;
pub mod auth;

use crate::db::{DBConn, models, schema};
use schema::courses::dsl::*;
use auth::ProviderGuard;

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

