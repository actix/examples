extern crate actix;
extern crate actix_web;
extern crate cookie;
extern crate env_logger;
extern crate futures;
extern crate time;

use actix_web::middleware::session::RequestSession;
use actix_web::{middleware, server, App, HttpRequest, HttpResponse};

mod auth;
use auth::{CookieIdentityPolicy, IdentityService, RequestIdentity};

fn index(mut req: HttpRequest) -> String {
    format!("Hello {}", req.identity().unwrap_or("Anonymous"))
}

fn login(mut req: HttpRequest) -> HttpResponse {
    req.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(mut req: HttpRequest) -> HttpResponse {
    req.forget();
    HttpResponse::Found().header("location", "/").finish()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("cookie-auth");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
                    .secure(false),
            ))
            .resource("/login", |r| r.f(login))
            .resource("/logout", |r| r.f(logout))
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
