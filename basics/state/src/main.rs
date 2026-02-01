//! Application may have multiple data objects that are shared across
//! all handlers within same Application.
//!
//! For global shared state, we wrap our state in a [`actix_web::web::Data`] and move it into
//! the factory closure. The closure is called once-per-thread, and we clone our state
//! and attach to each instance of the [`App`] with `.app_data(state.clone())`.
//!
//! For thread-local state, we construct our state within the factory closure and attach to
//! the app with `.app_data(Data::new(state))`.
//!
//! We retrieve our app state within our handlers with a `state: Data<...>` argument.
//!
//! By default, Actix Web runs one [`App`] per logical cpu core.
//! When running on `<N>` cores, we see that the example will increment `counter_mutex` (global state via
//! Mutex) and `counter_atomic` (global state via Atomic variable) each time the endpoint is called,
//! but only appear to increment `counter_cell` every Nth time on average (thread-local state). This
//! is because the workload is being shared equally among cores.
//!
//! Check [user guide](https://actix.rs/docs/application/#state) for more info.

use std::{
    cell::Cell,
    io,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, middleware,
    web::{self, Data},
};

/// simple handle
async fn index(
    counter_mutex: Data<Mutex<usize>>,
    counter_cell: Data<Cell<u32>>,
    counter_atomic: Data<AtomicUsize>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{req:?}");

    // Increment the counters
    *counter_mutex.lock().unwrap() += 1;
    counter_cell.set(counter_cell.get() + 1);
    counter_atomic.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "global mutex counter: {}, local counter: {}, global atomic counter: {}",
        *counter_mutex.lock().unwrap(),
        counter_cell.get(),
        counter_atomic.load(Ordering::SeqCst),
    );
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create some global state prior to building the server
    #[allow(clippy::mutex_atomic)] // it's intentional.
    let counter_mutex = Data::new(Mutex::new(0usize));
    let counter_atomic = Data::new(AtomicUsize::new(0usize));

    log::info!("starting HTTP server at http://localhost:8080");

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        // Create some thread-local state
        let counter_cell = Cell::new(0u32);

        App::new()
            .app_data(counter_mutex.clone()) // add shared state
            .app_data(counter_atomic.clone()) // add shared state
            .app_data(Data::new(counter_cell)) // add thread-local state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
