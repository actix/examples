#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
//! Application may have multiple data objects that are shared across
//! all handlers within same Application. Data could be added
//! with `App::data()` method, multiple different data objects could be added.
//!
//! > **Note**: http server accepts an application factory rather than an
//! application > instance. Http server constructs an application instance for
//! each thread, > thus application data
//! > must be constructed multiple times. If you want to share data between
//! different > threads, a shared object should be used, e.g. `Arc`.
//!
//! Check [user guide](https://actix.rs/book/actix-web/sec-2-application.html) for more info.

use std::io;
use std::sync::{Arc, Mutex};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

/// simple handle
fn index(state: web::Data<Arc<Mutex<usize>>>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    *(state.lock().unwrap()) += 1;

    HttpResponse::Ok().body(format!("Num of requests: {}", state.lock().unwrap()))
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let counter = Arc::new(Mutex::new(0));

    //move is necessary to give closure below ownership of counter
    HttpServer::new(move || {
        App::new()
            .data(counter.clone()) // <- create app with shared state
            // enable logger
            .middleware(middleware::Logger::default())
            // register simple handler, handle all methods
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
