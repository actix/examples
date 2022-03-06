use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use std::{sync::mpsc, thread};

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello world!"
}

#[post("/stop")]
async fn stop(stopper: web::Data<mpsc::Sender<()>>) -> HttpResponse {
    // make request that sends message through the Sender
    stopper.send(()).unwrap();

    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // create a channel
    let (tx, rx) = mpsc::channel::<()>();

    log::info!("starting HTTP server at http://localhost:8080");

    // start server as normal but don't .await after .run() yet
    let server = HttpServer::new(move || {
        // give the server a Sender in .data
        App::new()
            .app_data(web::Data::new(tx.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(stop)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    // clone the server handle
    let srv = server.handle();
    thread::spawn(move || {
        // wait for shutdown signal
        rx.recv().unwrap();

        // send stop server gracefully command
        srv.stop(true)
    });

    // run server until stopped (either by ctrl-c or stop endpoint)
    server.await
}
