#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate diesel;

use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
use schema::persons::dsl::*;

#[database("sqlite_db")]
struct DBConn(diesel::SqliteConnection);

pub mod schema;
pub mod models;


#[get("/")]
fn index(conn: DBConn) -> String {
    let results: Vec<models::Person> = persons.load::<models::Person>(&conn.0).expect("Error loading persons");
    format!("{:?}", results[0])
}

fn main() {
    println!("Hello, world!");

    rocket::ignite().attach(DBConn::fairing()).mount("/", routes![index]).launch();

    
}
