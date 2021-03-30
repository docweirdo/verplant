#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate diesel;

use log::info;

pub mod db;
pub mod http_api;

use db::DBConn;


fn main() {

    env_logger::init();

    info!("Server starting");

    rocket::ignite().attach(DBConn::fairing()).mount("/api/", routes![http_api::get_courses, http_api::get_provider_appointments])
    .mount("/api/auth/", routes![http_api::auth::login, http_api::auth::test]).launch();

    
}
