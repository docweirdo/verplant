use diesel::prelude::*;
//use rocket_contrib::databases::diesel;
//use schema::persons::dsl::*;
use schema::courses::dsl::*;
use rocket_contrib::json::Json;

use crate::{DBConn, schema, models};

#[get("/courses")]
pub fn index(conn: DBConn) -> Json<Vec<models::Course>> {
    let results: Vec<models::Course> = courses.load::<models::Course>(&conn.0).expect("Error loading courses");
    Json(results)
}