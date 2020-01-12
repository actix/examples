//! Application may have multiple data objects that are shared across
//! all handlers within same Application.
//!
//! For global shared state, we wrap our state in a `actix_web::web::Data` and move it into
//! the factory closure. The closure is called once-per-thread, and we clone our state
//! and attach to each instance of the `App` with `.app_data(state.clone())`.
//!
//! For thread-local state, we construct our state within the factory closure and attach to
//! the app with `.data(state)`.
//!
//! We retrieve our app state within our handlers with a `state: Data<...>` argument.
//!
//! By default, `actix-web` runs one `App` per logical cpu core.
//! When running on <N> cores, we see that the example will increment `counter1` (global state)
//! each time the endpoint is called, but only appear to increment `counter2` every
//! Nth time on average (thread-local state). This is because the workload is being shared
//! equally among cores.
//!
//! Check [user guide](https://actix.rs/docs/application/#state) for more info.

use std::cell::Cell;
use std::io;
use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

/// simple handle
async fn index(
    counter1: web::Data<Mutex<usize>>,
    counter2: web::Data<Cell<u32>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", req);

    // Increment the counters
    *counter1.lock().unwrap() += 1;
    counter2.set(counter2.get() + 1);

    let body = format!(
        "global counter: {}, local counter: {}",
        *counter1.lock().unwrap(),
        counter2.get()
    );
    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create some global state prior to building the server
    let counter1 = web::Data::new(Mutex::new(0usize));

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {

        // Create some thread-local state
        let counter2 = Cell::new(0u32);

        App::new()
            .app_data(counter1.clone()) // add shared state
            .data(counter2) // add thread-local state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
