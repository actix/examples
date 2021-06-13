use actix::*;

mod codec;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Start chat server actor
    let server = server::ChatServer::default().start();

    // Start tcp server in separate thread
    let srv = server.clone();
    session::tcp_server("127.0.0.1:12345", srv).await;
    println!("Started tcp server: 127.0.0.1:12345");
    Ok(())
}
