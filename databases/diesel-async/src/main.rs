#[macro_use]
extern crate diesel;

use std::{env, io};

use actix_web::{App, HttpResponse, HttpServer, Responder, error, get, middleware, post, web};
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use dotenvy::dotenv;
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = Pool<AsyncPgConnection>;

/// Finds item by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - an item UID from the request path
#[get("/items/{item_id}")]
async fn get_item(
    pool: web::Data<DbPool>,
    item_id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let item_id = item_id.into_inner();

    let mut conn = pool
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let item = actions::find_item_by_id(&mut conn, item_id)
        .await
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    Ok(match item {
        // item was found; return 200 response with JSON formatted item object
        Some(item) => HttpResponse::Ok().json(item),

        // item was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No item found with UID: {item_id}")),
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool().await;

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            .service(add_item)
            .service(get_item)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel-async/latest/diesel_async/pooled_connection/index.html#modules>.
async fn initialize_db_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("Env var `DATABASE_URL` not set");

    let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder().build(connection_manager).await.unwrap()
}

#[cfg(not(feature = "postgres_tests"))]
#[allow(unused_imports)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    use super::*;

    #[actix_web::test]
    async fn item_routes() {
        dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool().await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(get_item)
                .service(add_item),
        )
        .await;

        // send something that isn't a UUID to `get_item`
        let req = test::TestRequest::get().uri("/items/123").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"UUID parsing failed"),
            "unexpected body: {body:?}",
        );

        // try to find a non-existent item
        let req = test::TestRequest::get()
            .uri(&format!("/items/{}", Uuid::nil()))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"No item found"),
            "unexpected body: {body:?}",
        );

        // create new item
        let req = test::TestRequest::post()
            .uri("/items")
            .set_json(models::NewItem::new("Test item"))
            .to_request();
        let res: models::Item = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test item");

        // get an item
        let req = test::TestRequest::get()
            .uri(&format!("/items/{}", res.id))
            .to_request();
        let res: models::Item = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test item");

        // delete new item from table
        use crate::schema::items::dsl::*;
        diesel::delete(items.filter(id.eq(res.id)))
            .execute(
                &mut pool
                    .get()
                    .await
                    .expect("couldn't get db connection from pool"),
            )
            .await
            .expect("couldn't delete test item from table");
    }
}
