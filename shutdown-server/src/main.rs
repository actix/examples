use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use futures::executor;
use std::{sync::mpsc, thread};
use tokio::signal::unix::{signal, SignalKind};

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    // create a channel
    let (tx, rx) = mpsc::channel::<()>();
    let stopper = tx.clone();

    let bind = "127.0.0.1:8080";

    // start server as normal but don't .await after .run() yet
    let server = HttpServer::new(move || {
        // give the server a Sender in .data
        let stopper = tx.clone();

        App::new()
            .data(stopper)
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(stop)
    })
    .bind(&bind)?
    .run();

    // clone the Server handle
    let srv = server.clone();
    thread::spawn(move || {
        // wait for shutdown signal
        rx.recv().unwrap();

        // stop server gracefully
        executor::block_on(srv.stop(true))
    });

    let mut stream = signal(SignalKind::interrupt())?;
    actix_rt::spawn(async move {
        loop {
            stream.recv().await;
            println!("\n*** SIGINT received. Stopping server, gracefully. ***\n");
            stopper.send(()).unwrap();
        }
    });

    // run server
    server.await
}
