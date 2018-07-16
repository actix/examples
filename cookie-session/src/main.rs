//! Example of cookie based session
//! Session data is stored in cookie, it is limited to 4kb
//!
//! [Redis session example](https://github.com/actix/examples/tree/master/redis-session)
//!
//! [User guide](https://actix.rs/book/actix-web/sec-9-middlewares.html#user-sessions)

extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix_web::middleware::session::{self, RequestSession};
use actix_web::{middleware, server, App, HttpRequest, Result};
use std::env;

/// simple index handler with session
fn index(req: &HttpRequest) -> Result<&'static str> {
    println!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = req.session().get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        req.session().set("counter", counter)?;
    } else {
        req.session().set("counter", counter)?;
    }

    Ok("welcome!")
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("session-example");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .expect("Can not bind to 127.0.0.1:8080")
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
