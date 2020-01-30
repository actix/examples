extern crate postgres;
use crate::db::get_db_conn;
use crate::type_defs::Person;

pub fn get_person_all() -> Vec<Person> {
    let mut vec = Vec::new();
    let conn = get_db_conn();
    for row in &conn
        .query("SELECT id, name, cult FROM persons", &[])
        .unwrap()
    {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            cult: row.get(2),
        };
        vec.push(person);
    }
    vec
}
