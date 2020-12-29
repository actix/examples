use sqlx_user_crud::config::Config;
use sqlx_user_crud::dao::Database;

#[actix_rt::test]
async fn new_returns_db_context_when_url_is_valid() {
    let config = Config::from_file("test_resource/config.test.json");

    let _db_context = Database::new(&config.get_database_url()).await;
}
