use std::sync::{Mutex, Arc};
use crate::dao::DbContext;

pub mod model;
pub mod dao;
pub mod controller;
pub mod config;

// AppState
// This the primary dependency for our application's dependency injection.
// Each controller_test function that interacts with the database will require an `AppState` instance in
// order to communicate with the database.
pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<DbContext<'a>>,
}