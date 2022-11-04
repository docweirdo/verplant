use super::db::DatabaseError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::Responder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerplantError {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error("status code: {0}")]
    StatusCode(u16),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for VerplantError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            VerplantError::DatabaseError(e) => e.respond_to(req),
            VerplantError::StatusCode(code) => Status::from_code(code).respond_to(req),
        }
    }
}
