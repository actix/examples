use actix_web::{web, Error, HttpResponse};

use crate::common::{Part, Product};

pub async fn get_parts(query: web::Query<Option<Part>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_part(new_part: web::Json<Product>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_part_detail(id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_part(id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
