use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use awc::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};

async fn index(_req: HttpRequest) -> HttpResponse {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().openssl(builder.build()))
        .finish();

    let now = std::time::Instant::now();
    let payload =
        client
        .get("https://upload.wikimedia.org/wikipedia/commons/b/b9/Pizigani_1367_Chart_1MB.jpg")
        .send()
        .await
        .unwrap()
        .body()
        .limit(20_000_000)  // sets max allowable payload size
        .await
        .unwrap();

    println!(
        "awc time elapsed while reading bytes into memory: {} ms",
        now.elapsed().as_millis()
    );

    HttpResponse::Ok().content_type("image/jpeg").body(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
