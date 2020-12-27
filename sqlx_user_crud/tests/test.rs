use std::time::SystemTime;
use sqlx_user_crud::dao::DbContext;
use sqlx_user_crud::config::Config;
use uuid::Uuid;

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
    as u64
}

fn randomize_string(input: &'static str) -> String {
    format!("{0}{1}", input, Uuid::new_v4().to_string())
}

async fn init_db_context() -> DbContext<'static> {
    let config = Config::from_file("test_resource/config.test.json");
    DbContext::new(&config.get_database_url()).await
}

#[cfg(test)]
mod controller_test;

#[cfg(test)]
mod dao_test;