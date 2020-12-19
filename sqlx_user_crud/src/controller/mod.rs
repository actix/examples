use std::sync::Mutex;
use super::AppState;

pub mod index_controller;
pub mod user_controller;
pub mod group_controller;

pub use index_controller::init as init_index_controller;
pub use user_controller::init as init_user_controller;
pub use group_controller::init as init_group_controller;

fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
    println!("{}\n\tconnections: {}", route, con);
}