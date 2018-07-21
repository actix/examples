extern crate actix;
extern crate actix_web;
extern crate cookie;
extern crate env_logger;
extern crate futures;
extern crate time;

use actix_web::{middleware, server, App, HttpRequest, HttpResponse};
use actix_web::middleware::identity::RequestIdentity;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};

fn index(req: &HttpRequest) -> String {
    format!("Hello {}", req.identity().unwrap_or("Anonymous".to_owned()))
}

fn login(req: &HttpRequest) -> HttpResponse {
    req.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(req: &HttpRequest) -> HttpResponse {
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
