use crate::db::schema::*;
#[derive(Debug, Insertable)]
#[table_name = "appointments"]
pub struct NewAppointment {
    pub start_time: String,
    pub end_time: String,
    pub books_id: i32,
    pub proposer_id: i32,
    pub room_id: i32,
    pub state: String,
}
