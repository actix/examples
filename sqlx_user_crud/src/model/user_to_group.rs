use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;

pub struct UserToGroup {
    user_id: String,
    group_id: u64,
}

impl <'c>FromRow<'c, MySqlRow<'c>> for UserToGroup {
    fn from_row(row: &MySqlRow) -> Result<Self,sqlx::Error> {
        Ok(UserToGroup {
            user_id: row.get(0),
            group_id: row.get(1)
        })
    }
}