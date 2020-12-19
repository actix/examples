use super::randomize_string;
use sqlx_user_crud::config::Config;
use sqlx_user_crud::dao::DbContext;

async fn init_db_context() -> DbContext<'static> {
    let config = Config::from_file("test_resource/config.test.json");
    DbContext::new(&config.get_database_url()).await
}

#[cfg(test)]
mod db_context_test;

#[cfg(test)]
mod user_dao_test;
mod group_dao_test;
mod user_to_group_dao_test;