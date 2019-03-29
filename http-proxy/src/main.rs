use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;

/// Stream client request response and then send body to a server response
fn index(client: web::Data<Client>) -> impl Future<Item = HttpResponse, Error = Error> {
    client
        .get("http://127.0.0.1:8081/")
        .send()
        .map_err(Error::from) // <- convert SendRequestError to an Error
        .and_then(|resp| {
            resp.body() // <- this is MessageBody type, resolves to complete body
                .from_err() // <- convert PayloadError to an Error
                .and_then(|body| {
                    // <- we got complete body, now send as server response
                    Ok(HttpResponse::Ok().body(body))
                })
        })
}

/// streaming client request to a streaming server response
fn streaming(
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = impl Into<Error>> {
    // send client request
    client
        .get("https://www.rust-lang.org/en-US/")
        .send() // <- connect to host and send request
        .map_err(Error::from) // <- convert SendRequestError to an Error
        .and_then(|resp| {
            // <- we received client response
            Ok(HttpResponse::Ok()
                // read one chunk from client response and send this chunk to a server response
                // .from_err() converts PayloadError to an Error
                .streaming(resp))
        })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=trace");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .service(web::resource("/streaming").to_async(streaming))
            .service(web::resource("/").to_async(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
