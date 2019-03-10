use actix::prelude::*;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{fs, http::Method, middleware::Logger, App};
use auth_routes::{get_me, login, logout};
use chrono::Duration;
use invitation_routes::register_email;
use models::DbExecutor;
use register_routes::register_user;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

/// creates and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    // secret is a random minimum 32 bytes long base 64 string
    let secret: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
    let domain: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    App::with_state(AppState { db })
        .middleware(Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(1))
                .secure(false), // this can only be true if you have https
        ))
        // everything under '/api/' route
        .scope("/api", |api| {
            // routes for authentication
            api.resource("/auth", |r| {
                r.method(Method::POST).with(login);
                r.method(Method::DELETE).with(logout);
                r.method(Method::GET).with(get_me);
            })
            // routes to invitation
            .resource("/invitation", |r| {
                r.method(Method::POST).with(register_email);
            })
            // routes to register as a user after the
            .resource("/register/{invitation_id}", |r| {
                r.method(Method::POST).with(register_user);
            })
        })
        // serve static files
        .handler(
            "/",
            fs::StaticFiles::new("./static/")
                .unwrap()
                .index_file("index.html"),
        )
}
