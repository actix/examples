extern crate postgres;
use crate::db::get_db_conn;
use crate::type_defs::Person;
use dataloader::Loader;
use dataloader::{BatchFn, BatchFuture};
use futures::{future, FutureExt as _};
use std::collections::HashMap;

pub fn get_person_by_ids(hashmap: &mut HashMap<i32, Person>, ids: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT id, name, cult FROM persons WHERE id = ANY($1)",
            &[&ids],
        )
        .unwrap()
    {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            cult: row.get(2),
        };
        hashmap.insert(person.id, person);
    }
}

pub struct PersonBatcher;

impl BatchFn<i32, Person> for PersonBatcher {
    type Error = ();

    fn load(&self, keys: &[i32]) -> BatchFuture<Person, Self::Error> {
        println!("load batch {:?}", keys);
        let mut person_hashmap = HashMap::new();
        get_person_by_ids(&mut person_hashmap, keys.to_vec());
        future::ready(keys.iter().map(|key| person_hashmap[key].clone()).collect())
            .unit_error()
            .boxed()
    }
}

pub type PersonLoader = Loader<i32, Person, (), PersonBatcher>;

pub fn get_loader() -> PersonLoader {
    Loader::new(PersonBatcher)
}
