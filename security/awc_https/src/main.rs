use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

async fn index(_req: HttpRequest) -> HttpResponse {
    let client = reqwest::Client::new();

    let now = std::time::Instant::now();
    let payload =
        client
        .get("https://upload.wikimedia.org/wikipedia/commons/b/b9/Pizigani_1367_Chart_1MB.jpg")
        .send()
        .await
        .unwrap()
        .bytes()
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
