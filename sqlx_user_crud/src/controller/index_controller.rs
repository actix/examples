use super::log_request;
use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
}

#[get("/status")]
async fn status(data: web::Data<AppState<'_>>) -> impl Responder {
    log_request("GET: /status", &data.connections);

    HttpResponse::Ok().body("I am up")
}
