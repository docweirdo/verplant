use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode, Validation};
use rocket::{
    self,
    Request,
    http::{Cookie, Cookies, Status},
    request::{FromRequest, Outcome}
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use time::Duration;
use log::{error, warn, info};

use crate::db;
use db::DBConn;

const COOKIE_DURATION : u64 = 20 * 60;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JWTClaims {
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64,

    id: i32,
}

#[post("/login", data = "<credentials>")]
pub fn login(
    mut cookies: Cookies,
    credentials: Json<Credentials>,
    conn: DBConn,
) -> Status {
    let email = credentials.email.trim();
    let password = credentials.password.trim();

    let id = match db::verify_user(&conn, email, password) {    //Todo: Differentiate between gravity of error somehow
        Err(e) => {
            error!(target: "/login", "verifying user failed: {}", e);
            return Status::Unauthorized;
        },
        Ok(id) => id,
    };

    // Create JWT
    let now: u64 = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let expiration_time: u64 = now + COOKIE_DURATION;
    let my_claims = JWTClaims {
        exp: expiration_time,
        iat: now,
        id,
    };

    let token: String = if let Ok(token) = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),                   //TODO: Use actual secret
    ){
        token
    } else {
        return Status::InternalServerError;     //TODO: Streamline Status Reponse with actual Error type
    };

    cookies.add(
        Cookie::build("jwt", token)
            .http_only(true)
            .max_age(Duration::seconds(COOKIE_DURATION as i64)) //TODO: Set secure only
            .path("/")
            .finish()
    );

    Status::Ok
}

#[get("/test/<password>")]
pub fn test(password: String, conn: DBConn) -> Result<(), Status> {
    db::set_password(&conn, 1, &password).map_err(|_| Status::InternalServerError)
}


pub struct ProviderGuard {
    pub person_id: i32,
    pub is_admin: bool
}

impl<'a, 'r> FromRequest<'a, 'r> for ProviderGuard {
    type Error = ();

    //TODO: Change if lets to match statements to be able to use errors in warnings
    fn from_request(request: &'a Request<'r>) -> Outcome<ProviderGuard, ()> {
        let cookies: Cookies = if let Outcome::Success(cookies) = request.guard::<Cookies>() {
            cookies
        } else { warn!(target: "ProviderGuard", "no cookies"); return Outcome::Forward(()) };
        info!("{:?}", cookies);
        let user_id: i32 = if let Some(cookie) = cookies.get("jwt") {
            if let Ok(token_data) = decode::<JWTClaims>(cookie.value(), &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
                token_data.claims.id
            } else { warn!(target: "ProviderGuard", "JWT decode failed"); return Outcome::Forward(())}
        } else { warn!(target: "ProviderGuard", "cookies have no jwt key");  return Outcome::Forward(()) };
        
        let conn = if let Outcome::Success(conn) = request.guard::<DBConn>(){
            conn
        } else { warn!(target: "ProviderGuard", "no DBConn");  return Outcome::Forward(()) };

        match db::is_user_admin(conn, user_id) {
            Ok(is_admin) => Outcome::Success(ProviderGuard { person_id: user_id, is_admin}),
            Err(e) => {
                error!(target: "ProviderGuard", "is_user_admin database error: {}", e);
                Outcome::Forward(())
            }
        } 

        
    }
}