
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
use std::time::Duration;


pub fn db_config() -> Pool{
    let mut cfg = Config::new();
    cfg.application_name("region-service")
        .connect_timeout(Duration::from_secs(2))
        .host("127.0.0.1")
        .port(5432)
        .dbname("test_data")
        .user("admin")
        .password("gadmin");
    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::new(mgr, 10);
    pool
}