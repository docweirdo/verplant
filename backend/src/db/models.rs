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

#[derive(Queryable, Debug, Insertable, Serialize, Identifiable)]
#[table_name="courses"]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub default_duration: Option<i32>,
    pub default_room_id: Option<i32>,
    pub course_type: Option<String>
}

#[derive(Queryable, Debug, Serialize, Identifiable)]
#[table_name="providers"]
#[primary_key(person_id)]
pub struct Provider {
    pub person_id: i32,
    pub password_hash: String,
    is_admin: bool,
}

