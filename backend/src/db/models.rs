use crate::db::schema::*;
use serde::Serialize;


#[derive(Queryable, Debug, Insertable, Serialize, Identifiable, Associations)]
#[table_name="persons"]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>
}

#[derive(Queryable, Debug, Insertable, Serialize, Identifiable, Associations)]
//#[belongs_to(Room, foreign_key = "default_room_id")] Uncomment later when Room Struct exists
#[table_name="courses"]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub default_duration: Option<i32>,
    pub default_room_id: Option<i32>,
    pub course_type: Option<String>
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[table_name="providers"]
#[belongs_to(Person, foreign_key = "person_id")]
#[primary_key(person_id)]
pub struct Provider {
    pub person_id: i32,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[table_name="appointments"]
#[belongs_to(Person, foreign_key = "proposer_id")]
#[primary_key(id)]
pub struct Appointment {
    pub id: i32,
    pub date: String,
    pub starttime: String,
    pub endtime: String,
    pub status: String,
    pub proposer_id: i32
    // sTODO: some fields are missing
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[table_name="books"]
#[belongs_to(Course, foreign_key = "course_id")]
#[belongs_to(Customer, foreign_key = "course_id")]
#[primary_key(id)]
pub struct Booking{
    pub id: i32,
    pub url: String,
    pub course_id: i32,
    pub customer_id: i32
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[table_name="customers"]
#[belongs_to(Person, foreign_key = "person_id")]
#[primary_key(person_id)]
pub struct Customer{
    pub person_id: i32,
    pub organisation: String, 
    pub class: Option<String>
}