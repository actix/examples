use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use service::UserService;
use std::sync::Arc;

mod handlers;
mod service;

struct MongoDB {
    client: Client,
}
impl MongoDB {
    async fn init() -> Result<Self, mongodb::error::Error> {
        let client_uri = std::env::var("MONGODB_URI").expect("set MONGODB_URI env var");
        let client_options = ClientOptions::parse(&client_uri).await?;
        Ok(MongoDB {
            client: Client::with_options(client_options)?,
        })
    }
    fn connection_base(&self, base: &str) -> mongodb::Database {
        self.client.database(base)
    }
}

struct AppState {
    user: UserService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo = MongoDB::init().await.expect("mongodb connection");
    let db = mongo.connection_base("myApp");
    let app_state = Arc::new(AppState {
        user: UserService::new(db.collection("users")),
    });

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .configure(handlers::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
