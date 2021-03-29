use crate::db::schema::*;
use serde::Serialize;


#[derive(Queryable, Debug, Insertable, Serialize)]
#[table_name="persons"]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>
}

#[derive(Queryable, Debug, Insertable, Serialize)]
#[table_name="courses"]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub default_duration: Option<i32>,
    pub default_room_id: Option<i32>,
    pub course_type: Option<String>
}

