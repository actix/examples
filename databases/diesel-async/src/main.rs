#[macro_use]
extern crate diesel;

use std::env::VarError;

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager, PoolError};
use diesel_async::AsyncPgConnection;
use dotenvy::dotenv;
use std::{env, io};
use thiserror::Error as ThisError;
use uuid::Uuid;

pub mod actions;
pub mod models;
pub mod schema;

type DbPool = Pool<AsyncPgConnection>;

/// Finds item by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - an item UID from the request path
#[get("/items/{item_id}")]
async fn get_item(
    pool: web::Data<DbPool>,
    item_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let item_uid = item_uid.into_inner();

    let mut conn = pool
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let item = actions::find_item_by_uid(&mut conn, item_uid)
    .await
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match item {
        // item was found; return 200 response with JSON formatted item object
        Some(item) => HttpResponse::Ok().json(item),

        // item was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No item found with UID: {item_uid}")),
    })
}

/// Creates new item.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new item info from the request body
#[post("/items")]
async fn add_item(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewItem>,
) -> actix_web::Result<impl Responder> {
    
    let mut conn = pool
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let item = actions::insert_new_item(&mut conn, &form.name)
    .await
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // item was added successfully; return 201 response with new item info
    Ok(HttpResponse::Created().json(item))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").expect("Env var `DATABASE_URL` not set");

    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder().build(mgr).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(add_item)
            .service(get_item)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
