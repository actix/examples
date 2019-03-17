use actix_web::{error::ErrorInternalServerError, web::Query, HttpResponse, Result};
use yarte::Template;

use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexTemplate {
    query: Query<HashMap<String, String>>,
}

pub fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    IndexTemplate { query }
        .call()
        .map(|s| {
            HttpResponse::Ok()
                .content_type(IndexTemplate::mime())
                .body(s)
        })
        .map_err(|_| ErrorInternalServerError("Template parsing error"))
}
