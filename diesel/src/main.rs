//! Actix web diesel example

// Suppress spurious warnings for Diesel in recent Rust versions
// This is fixed on diesel master, awaiting a release
#![allow(proc_macro_derive_resolution_fallback)]

extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_diesel;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;

use actix_diesel::Database;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path,
    State,
};

use diesel::prelude::*;
use futures::Future;

mod db;
mod models;
mod schema;

/// State with Database
struct AppState {
    db: Database<SqliteConnection>,
}

/// Async request handler
fn index(
    (name, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .get(move |conn| db::create_user(conn, name.into_inner()))
        .then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info,actix_net::server::server=info");
    env_logger::init();

    // Open the database
    let db = Database::open("test.db");

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: db.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/{name}", |r| r.method(http::Method::GET).with(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
