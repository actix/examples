use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, error, post, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
}

#[post("/")]
async fn greet(name: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name.name))
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new().service(greet).app_data(
            web::JsonConfig::default()
                // register error_handler for JSON extractors.
                .error_handler(json_error_handler),
        )
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
