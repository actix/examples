extern crate postgres;
use postgres::{Connection, TlsMode};
use std::env;

pub fn get_db_conn() -> Connection {
    let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    println!("Connecting to {}", pg_connection_string);
    let conn = Connection::connect(&pg_connection_string[..], TlsMode::None).unwrap();
    println!("Connection is fine");
    conn
}
