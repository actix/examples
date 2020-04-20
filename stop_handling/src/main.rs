use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;
use tokio::signal;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:8088";
    let server =
        HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
            .bind(server_addr)?
            .run();

    let s = server.clone();
    actix_rt::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("\nSIGINT Received.  Stopping server.\n");
        s.stop(true).await;
    });

    println!("Server running at http://{}/", server_addr);
    server.await
}
// use actix_server::Server;
// #[cfg(windows)]
// fn stop_listener(server: Server) -> io::Result<()> {
//     use tokio::signal::windows::ctrl_break;
//     let mut stream = ctrl_break().map_err(|e| io::Error::from(e))?;
//     actix_rt::spawn(async move {
//         loop {
//             stream.recv().await;
//             println!("\nSIGINT Received.  Stopping server.\n");
//             server.stop(true).await;
//         }
//     });

//     Ok(())
// }

// #[cfg(unix)]
// fn stop_listener(server: Server) -> io::Result<()> {
//     use tokio::signal::unix::{signal, SignalKind};
//     let mut stream = signal(SignalKind::interrupt())?;

//     actix_rt::spawn(async move {
//         loop {
//             stream.recv().await;
//             println!("\nSIGINT Received.  Stopping server.\n");
//             server.stop(true).await;
//         }
//     });

//     Ok(())
// }
