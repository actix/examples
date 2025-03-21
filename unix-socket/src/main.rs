use actix_web::{App, HttpRequest, HttpServer, middleware, web};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[actix_web::main]
#[cfg(unix)]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at unix:/tmp/actix-uds.socket");

    HttpServer::new(|| {
        App::new()
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").route(web::get().to(|| async { "Hello world!" })))
            .service(web::resource("/").to(index))
    })
    .bind_uds("/tmp/actix-uds.socket")?
    .run()
    .await
}

#[cfg(not(unix))]
fn main() -> std::io::Result<()> {
    log::info!("Example only runs on UNIX");
    Ok(())
}
