#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate diesel;


pub mod db;
pub mod http_api;

use db::DBConn;

fn main() {
    println!("Hello, world!");

    rocket::ignite().attach(DBConn::fairing()).mount("/api/", routes![http_api::get_courses])
    .mount("api/auth/", routes![http_api::auth::login]).launch();

    
}
