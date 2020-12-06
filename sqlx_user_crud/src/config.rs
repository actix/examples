use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    url: String,
    port: u16,
}
#[derive(Deserialize)]
struct DaoConfig {
    user: String,
    password: String,
    address: String,
    database: String,
}
#[derive(Deserialize)]
struct Config {
    app: AppConfig,
    dao: DaoConfig,
}

impl Config {

    fn FromFile(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    fn GetAppUrl(&self) -> String {
        format!("{0}:{1}", self.app.url, self.app.port)
    }

    fn GetDatabaseUrl(&self) -> String {
        format!("mysql://{0}:{1}@{2}/{3}"
                , self.dao.user
                , self.dao.password
                , self.dao.address
                , self.dao.database)
    }
}