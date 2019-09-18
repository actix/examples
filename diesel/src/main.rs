//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use bytes::BytesMut;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use futures::future::{err, Either};
use futures::{Future, Stream};

mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Diesel query
fn query(
    nm: String,
    pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: nm.as_str(),
    };
    let conn: &SqliteConnection = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

/// Async request handler
fn add(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || query(name.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_add(
    pl: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        //
        // Douman NOTE:
        // The return value in this closure helps, to clarify result for compiler
        // as otheriwse it cannot understand it
        .and_then(move |body| {
            // body is loaded, now we can deserialize serde-json
            let r_obj = serde_json::from_slice::<MyUser>(&body);

            // Send to the db for create
            match r_obj {
                Ok(obj) => {
                    Either::A(web::block(move || query(obj.name, pool)).then(|res| {
                        match res {
                            Ok(user) => Ok(HttpResponse::Ok().json(user)),
                            Err(_) => Ok(HttpResponse::InternalServerError().into()),
                        }
                    }))
                }
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
            }
        })
}

fn add2(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || query(item.into_inner().name, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // This can be called with:
            // curl -S --header "Content-Type: application/json" --request POST --data '{"name":"xyz"}'  http://127.0.0.1:8080/add
            // Use of the extractors makes some post conditions simpler such
            // as size limit protections and built in json validation.
            .service(
                web::resource("/add2")
                    .data(
                        web::JsonConfig::default()
                            .limit(4096) // <- limit size of the payload
                            .error_handler(|err, _| {
                                // <- create custom error response
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                .into()
                            }),
                    )
                    .route(web::post().to_async(add2)),
            )
            //  Manual parsing would allow custom error construction, use of
            //  other parsers *beside* json (for example CBOR, protobuf, xml), and allows
            //  an application to standardise on a single parser implementation.
            .service(web::resource("/add").route(web::post().to_async(index_add)))
            .service(web::resource("/add/{name}").route(web::get().to_async(add)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
