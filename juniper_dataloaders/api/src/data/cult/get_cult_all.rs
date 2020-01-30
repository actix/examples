extern crate postgres;
use crate::db::get_db_conn;
use crate::type_defs::Cult;

pub fn get_cult_all() -> Vec<Cult> {
    let mut vec = Vec::new();
    let conn = get_db_conn();
    for row in &conn.query("SELECT id, name, cult FROM cults", &[]).unwrap() {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1),
        };
        vec.push(cult);
    }
    vec
}
