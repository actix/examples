use actix_web::{
    error, get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use apalis::{prelude::*, redis::RedisStorage};
use chrono::{Duration, Utc};
use serde::Deserialize;

use crate::{persistent_jobs::Email, ItemCache};

#[derive(Debug, Deserialize)]
pub(crate) struct CacheInsert {
    data: String,
    duration: u64,
}

#[get("/cache")]
pub(crate) async fn view_cache(cache: Data<ItemCache>) -> actix_web::Result<impl Responder> {
    let cached_data = &*cache.lock().unwrap();
    Ok(HttpResponse::Ok().json(cached_data))
}

#[post("/cache")]
pub(crate) async fn cache_item(
    cache: Data<ItemCache>,
    web::Json(form): web::Json<CacheInsert>,
) -> actix_web::Result<impl Responder> {
    let expires = Utc::now() + Duration::seconds(form.duration as i64);

    // insert into item cache
    cache.lock().unwrap().insert(form.data, expires);

    Ok(HttpResponse::Ok().body(format!("data cached until {:?}", expires)))
}

#[post("/email")]
pub(crate) async fn send_email(
    sender: Data<RedisStorage<Email>>,
    web::Json(form): web::Json<Email>,
) -> actix_web::Result<impl Responder> {
    (**sender)
        .clone()
        .push(form)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Accepted())
}

#[post("/email-spam")]
pub(crate) async fn send_email_batch(
    sender: Data<RedisStorage<Email>>,
) -> actix_web::Result<impl Responder> {
    let mut sender = (**sender).clone();

    for _ in 0..50 {
        sender
            .push(Email::random())
            .await
            .map_err(error::ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Accepted())
}
