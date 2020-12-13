use sqlx_user_crud::dao::DbContext;
use sqlx_user_crud::config::Config;

#[actix_rt::test]
async fn new_returns_db_context_when_url_is_valid() {
    let config = Config::from_file("test_resource/config.test.json");

    let db_context = DbContext::new(&config.get_database_url()).await;
}

#[actix_rt::test]
#[should_panic]
async fn new_panics_when_url_is_invalid() {
    let config = Config::from_file("test_resource/invalid_config.json");

    let db_context = DbContext::new(&config.get_database_url()).await;
}