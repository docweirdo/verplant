use log::info;
use pwhash::bcrypt;
use diesel::{prelude::*, update};
use anyhow::{Result, bail};


pub mod models;
pub mod schema;

use schema::providers::dsl::*;
use schema::persons::dsl::*;
use models::*;

#[database("sqlite_db")]
pub struct DBConn(diesel::SqliteConnection);



/// Returns the id if user is verified
pub fn verify_user(conn: &DBConn, user_email: &str, password: &str) -> Result<i32> {

    let user : Person = persons.filter(email.eq(user_email)).first::<Person>(&conn.0)?;

    match providers.find(user.id).select(password_hash).first::<Option<String>>(&conn.0)? {
        Some(pw_hash) => {
            if bcrypt::verify(password, &pw_hash) {
                return Ok(user.id)
            } else {
                bail!("Password hash does not match");
            };
            },         
        _ => bail!("No password set for provider with id {}", user.id)
    }
}

pub fn set_password(conn: &DBConn, user_id: i32, password: &str) -> Result<()>{
    let hash : String = bcrypt::hash(password).unwrap();

    update(providers.find(user_id)).set(password_hash.eq(hash)).execute(&conn.0)?;
    info!("Set password for user {}", user_id);
    
    Ok(())
}   

pub fn is_user_admin(conn: DBConn, user_id: i32) -> Result<bool>{
    let result : bool = providers.find(user_id).select(is_admin).first::<i32>(&conn.0)? != 0;
    Ok(result)
}