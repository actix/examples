//! Actix web r2d2 example
use std::io;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid;

/// Async request handler. Ddb pool is stored in application state.
fn index(
    path: web::Path<String>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // execute sync code in threadpool
    web::block(move || {
        let conn = db.get().unwrap();

        let uuid = format!("{}", uuid::Uuid::new_v4());
        conn.execute(
            "INSERT INTO users (id, name) VALUES ($1, $2)",
            &[&uuid, &path.into_inner()],
        )
        .unwrap();

        conn.query_row("SELECT name FROM users WHERE id=$1", &[&uuid], |row| {
            row.get::<_, String>(0)
        })
    })
    .then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("r2d2-example");

    // r2d2 pool
    let manager = SqliteConnectionManager::file("test.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .route("/{name}", web::get().to_async(index))
    })
    .bind("127.0.0.1:8080")?
    .start();

    sys.run()
}
