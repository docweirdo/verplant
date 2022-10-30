use crate::db::schema::*;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Insertable, Serialize, Identifiable, Selectable)]
#[diesel(table_name = persons)]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Queryable, Debug, Insertable, Serialize, Identifiable)]
//#[belongs_to(Room, foreign_key = "default_room_id")] Uncomment later when Room Struct exists
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub default_duration: Option<i32>,
    pub default_room_id: i32,
    pub course_type: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[diesel(table_name = providers)]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(primary_key(person_id))]
pub struct Provider {
    pub person_id: i32,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(
    Queryable,
    Insertable,
    Debug,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
    AsChangeset,
    Selectable,
)]
#[diesel(table_name = appointments)]
#[diesel(belongs_to(Person, foreign_key = proposer_id))]
#[diesel(primary_key(id))]
pub struct Appointment {
    pub id: i32,
    pub start_time: String,
    pub end_time: String,
    pub books_id: i32,
    pub proposer_id: i32,
    pub room_id: i32,
    pub state: String,
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[diesel(table_name = books)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(belongs_to(Customer, foreign_key = course_id))]
#[diesel(primary_key(id))]
pub struct Booking {
    pub id: i32,
    pub url: String,
    pub course_id: i32,
    pub customer_id: i32,
}

#[derive(Queryable, Debug, Serialize, Identifiable, Associations)]
#[diesel(table_name = customers)]
#[diesel(belongs_to(Person, foreign_key = person_id))]
#[diesel(primary_key(person_id))]
pub struct Customer {
    pub person_id: i32,
    pub organisation: String,
    pub class: Option<String>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct BookingInfo {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>,
    pub organisation: String,
    pub class: Option<String>,
    pub course_id: i32,
}
