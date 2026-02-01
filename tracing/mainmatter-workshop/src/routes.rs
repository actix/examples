use std::time::Duration;

use actix_web::{HttpResponse, Responder, get, web::ThinData};
use metrics_exporter_prometheus::PrometheusHandle;

#[get("/hello")]
pub(crate) async fn hello() -> impl Responder {
    "Hello, World!"
}

#[get("/sleep")]
pub(crate) async fn sleep() -> impl Responder {
    actix_web::rt::time::sleep(Duration::from_millis(500)).await;
    HttpResponse::Ok()
}

#[get("/metrics")]
pub(crate) async fn metrics(metrics_handle: ThinData<PrometheusHandle>) -> impl Responder {
    metrics_handle.render()
}
