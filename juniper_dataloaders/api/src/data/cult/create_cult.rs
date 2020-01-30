use crate::db::get_db_conn;
use crate::type_defs::{Cult, NewCult};

pub fn create_cult(data: NewCult) -> Cult {
    let conn = get_db_conn();
    let res = &conn
        .query(
            "INSERT INTO cults (name) VALUES ($1) RETURNING id, name;",
            &[&data.name],
        )
        .unwrap();
    let row = res.iter().next().unwrap();
    Cult {
        id: row.get(0),
        name: row.get(1),
    }
}
