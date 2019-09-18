#[macro_use]
extern crate tera;

use std::collections::HashMap;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};

// store tera template in application state
fn index(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name.to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
