use actix_web::{web, Error, HttpResponse};
use futures::{future::ok as fut_ok, Future};

use crate::common::{Part, Product};

pub fn get_parts(
    query: web::Query<Option<Part>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn add_part(
    new_part: web::Json<Product>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn get_part_detail(
    id: web::Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn remove_part(
    id: web::Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}
