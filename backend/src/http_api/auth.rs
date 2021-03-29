use rocket_contrib::json::{Json, JsonValue};
use rocket::{self, http::Status};
use serde::{Serialize, Deserialize};

use crate::db;
use db::DBConn;

#[derive(Serialize, Deserialize)]
struct Credentials {
   email: String,
   password: String
}


#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, conn: DBConn) -> Result<Json<JsonValue>, Status>{
    let email = credentials.email;
    let password = credentials.password;

    if let Err(()) = db::verify_user(conn, email, password){
        Err(Status::Unauthorized)
    }

    // Create JWT

}