use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;
use super::Group;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String, // TODO: use the Uuid type
    pub name: String,
    pub email: String, // TODO: use the EmailAddress type
    pub groups: Vec<Group>,
}

impl <'c>FromRow<'c, MySqlRow<'c>> for User {
    fn from_row(row: &MySqlRow) -> Result<Self,sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            groups: Vec::with_capacity(0),
        })
    }
}