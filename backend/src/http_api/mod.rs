use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use rocket_contrib::json::Json;


use crate::db::{DBConn, models, schema};
use schema::courses::dsl::*;


pub mod auth;

#[get("/courses")]
pub fn get_courses(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses.load::<models::Course>(&conn.0).expect("Error loading courses");
    Json(results)
}