use std::sync::mpsc;
use std::{thread, time};

use actix_web::dev::ServerHandle;
use actix_web::{middleware, rt, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    log::info!("REQ: {:?}", req);
    "Hello world!"
}

async fn run_app(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    // srv is server controller type, `dev::Server`
    let server = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Send server handle back to the main thread
    let _ = tx.send(server.handle());
    server.await
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=trace");
    env_logger::init();

    let (tx, rx) = mpsc::channel();

    log::info!("START SERVER");
    thread::spawn(move || {
        let future = run_app(tx);
        rt::System::new().block_on(future)
    });

    let server_handle = rx.recv().unwrap();

    log::info!("WAITING 10 SECONDS");
    thread::sleep(time::Duration::from_secs(10));

    log::info!("STOPPING SERVER");
    // Send a stop signal to the server, waiting for it to exit gracefully
    rt::System::new().block_on(server_handle.stop(true));
}
