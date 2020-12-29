use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Group {
    pub id: u64,
    pub name: String,
}

impl<'c> FromRow<'c, MySqlRow<'c>> for Group {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Group {
            id: row.get(0),
            name: row.get(1),
        })
    }
}
