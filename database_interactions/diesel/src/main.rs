//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::find_user_by_uid(user_uid, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::insert_new_user(&form.name, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
    })
    .bind(&bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn user_routes() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<SqliteConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .service(get_user)
                .service(add_user),
        )
        .await;

        // Insert a user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&models::NewUser {
                name: "Test user".to_owned(),
            })
            .to_request();

        let resp: models::User = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", resp.id))
            .to_request();

        let resp: models::User = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.name, "Test user");

        // Delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(resp.id)))
            .execute(&pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }
}
