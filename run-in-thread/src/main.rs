//! Since Actix Web v4, this method is not necessary in order to spawn additional servers alongside
//! your web server since the `Server` object can simply be `spawn`ed. This is kept to illustrate
//! how to run Actix Web from a sync context.

use std::{sync::mpsc, thread, time};

use actix_web::{App, HttpRequest, HttpServer, dev::ServerHandle, middleware, rt, web};

async fn index(req: HttpRequest) -> &'static str {
    log::info!("REQ: {req:?}");
    "Hello world!"
}

async fn run_app(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");

    // srv is server controller type, `dev::Server`
    let server = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run();

    // Send server handle back to the main thread
    let _ = tx.send(server.handle());

    server.await
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (tx, rx) = mpsc::channel();

    log::info!("spawning thread for server");
    thread::spawn(move || {
        let server_future = run_app(tx);
        rt::System::new().block_on(server_future)
    });

    let server_handle = rx.recv().unwrap();

    log::info!("waiting 10 seconds");
    thread::sleep(time::Duration::from_secs(10));

    // Send a stop signal to the server, waiting for it to exit gracefully
    log::info!("stopping server");
    rt::System::new().block_on(server_handle.stop(true));
}
