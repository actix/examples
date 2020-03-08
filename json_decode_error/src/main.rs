use actix_web::{
    error, post, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[post("/")]
async fn greet(name: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name.name))
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = format!("{}", err);
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .app_data(web::Json::<Info>::configure(|cfg| {
                cfg.error_handler(json_error_handler)
            }))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
