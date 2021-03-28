#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate diesel;


#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);

pub mod schema;
pub mod models;
pub mod http_api;

fn main() {
    println!("Hello, world!");

    rocket::ignite().attach(DBConn::fairing()).mount("/api/", routes![http_api::index]).launch();

    
}
