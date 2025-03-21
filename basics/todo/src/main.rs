use std::{env, io};

use actix_files::Files;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer, http,
    middleware::{ErrorHandlers, Logger},
    web,
};
use dotenvy::dotenv;
use tera::Tera;

mod api;
mod db;
mod model;
mod session;

// NOTE: Not a suitable session key for production.
static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        log::debug!("Constructing the App");

        let mut templates = Tera::new("templates/**/*").expect("errors in tera templates");
        templates.autoescape_on(vec!["tera"]);

        let session_store = SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
            .cookie_secure(false)
            .build();

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(session_store)
            .wrap(error_handlers)
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/todo").route(web::post().to(api::create)))
            .service(web::resource("/todo/{id}").route(web::post().to(api::update)))
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
