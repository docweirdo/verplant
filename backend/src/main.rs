use log::info;

pub mod db;
pub mod error;
pub mod http_api;
use rocket::launch;

use db::DBConn;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    info!("Server starting");
    let mut rocket = rocket::build().attach(DBConn::fairing());
    rocket = http_api::mount_endpoints(rocket);
    rocket = http_api::auth::mount_endpoints(rocket);
    rocket
}
