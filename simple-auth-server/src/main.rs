#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth_handler;
mod email_service;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(web::resource("/invitation").route(
                        web::post().to_async(invitation_handler::post_invitation),
                    ))
                    .service(
                        web::resource("/register/{invitation_id}").route(
                            web::post().to_async(register_handler::register_user),
                        ),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to_async(auth_handler::login))
                            .route(web::delete().to(auth_handler::logout))
                            .route(web::get().to(auth_handler::get_me)),
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
