use crate::schema::persons;

#[derive(Queryable, Debug, Insertable)]
#[table_name="persons"]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: Option<String>
}