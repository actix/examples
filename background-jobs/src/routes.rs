use actix_web::{
    HttpResponse, Responder, error, get, post,
    web::{self, Data},
};
use apalis::prelude::*;
use apalis_redis::RedisStorage;
use chrono::{TimeDelta, Utc};
use serde::Deserialize;

use crate::{ItemCache, persistent_jobs::Email};

#[derive(Debug, Deserialize)]
pub(crate) struct CacheInsert {
    data: String,
    duration: u32,
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
    let expires = Utc::now() + TimeDelta::try_seconds(form.duration as i64).unwrap();

    // insert into item cache
    cache.lock().unwrap().insert(form.data, expires);

    Ok(HttpResponse::Ok().body(format!("data cached until {expires}")))
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
