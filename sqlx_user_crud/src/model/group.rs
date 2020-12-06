use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: u64,
    pub name: String,
}

impl <'c>FromRow<'c, MySqlRow<'c>> for Group {
    fn from_row(row: &MySqlRow) -> Result<Self,sqlx::Error> {
        Ok(Group {
            id: row.get(0),
            name: row.get(1)
        })
    }
}