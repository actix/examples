use actix_web::{Query, Responder};
use yarte::Template;

use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexTemplate {
    query: Query<HashMap<String, String>>,
}

pub fn index(query: Query<HashMap<String, String>>) -> impl Responder {
    IndexTemplate { query }
}
