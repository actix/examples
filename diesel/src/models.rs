use super::schema::users;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
