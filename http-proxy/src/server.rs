use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};

fn index(body: web::Bytes) -> impl Responder {
    HttpResponse::Ok().body(body)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=trace");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| "Hello world!"))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8081")?
    .run()
}
