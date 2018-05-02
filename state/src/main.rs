#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
//! There are two level of statefulness in actix-web. Application has state
//! that is shared across all handlers within same Application.
//! And individual handler can have state.
//!
//! > **Note**: http server accepts an application factory rather than an application
//! > instance. Http server constructs an application instance for each thread,
//! > thus application state
//! > must be constructed multiple times. If you want to share state between different
//! > threads, a shared object should be used, e.g. `Arc`.
//!
//! Check [user guide](https://actix.rs/book/actix-web/sec-2-application.html) for more info.

extern crate actix;
extern crate actix_web;
extern crate env_logger;

use std::cell::Cell;

use actix::prelude::*;
use actix_web::{http, middleware, server, ws, App, HttpRequest, HttpResponse};

/// Application state
struct AppState {
    counter: Cell<usize>,
}

/// simple handle
fn index(req: HttpRequest<AppState>) -> HttpResponse {
    println!("{:?}", req);
    req.state().counter.set(req.state().counter.get() + 1);

    HttpResponse::Ok().body(format!("Num of requests: {}", req.state().counter.get()))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("ws-example");

    server::new(|| {
        App::with_state(AppState{counter: Cell::new(0)}) // <- create app with state
            // enable logger
            .middleware(middleware::Logger::default())
            // register simple handler, handle all methods
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
