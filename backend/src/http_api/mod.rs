use rocket::{Build, Rocket};
use serde::Deserialize;

pub mod auth;
pub mod authenticated;
pub mod customer;

#[derive(Deserialize, Debug)]
pub struct AppointmentSuggestion {
    pub start_time: String,
    pub end_time: String,
    pub room_id: Option<i32>,
}

pub fn mount_endpoints(mut rocket: Rocket<Build>) -> Rocket<Build> {
    rocket = customer::mount_endpoints(rocket);
    authenticated::mount_endpoints(rocket)
}
