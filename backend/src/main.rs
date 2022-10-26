#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use log::info;

pub mod db;
pub mod http_api;

use db::DBConn;

fn main() {
    env_logger::init();

    info!("Server starting");
    let mut rocket = rocket::ignite().attach(DBConn::fairing());
    rocket = http_api::mount_endpoints(rocket);
    rocket = http_api::auth::mount_endpoints(rocket);
    rocket.launch();
}
