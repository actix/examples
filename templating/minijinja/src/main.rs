use std::collections::HashMap;

use actix_web::{
    dev::ServiceResponse,
    error,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    web, App, Error, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_lab::respond::Html;

async fn index(
    tmpl_env: web::Data<minijinja::Environment<'static>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let html = if let Some(name) = query.get("name") {
        let tmpl = tmpl_env
            .get_template("user.html")
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

        let ctx = minijinja::context! {
            name,
            text => "Welcome!",
        };

        tmpl.render(ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl_env
            .get_template("index.html")
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
            .render(())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(Html(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let mut env: minijinja::Environment<'static> = minijinja::Environment::new();
    env.set_source(minijinja::Source::from_path(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/templates"
    )));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(env.clone()))
            .service(web::resource("/").route(web::get().to(index)))
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Error handler for a 404 Page not found error.
fn not_found<B>(svc_res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res = get_error_response(&svc_res, "Page not found");

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        svc_res.into_parts().0,
        res.map_into_right_body(),
    )))
}

/// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let req = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let ctx = minijinja::context! {
        error => error,
        status_code => res.status().as_str(),
    };

    match req
        .app_data::<web::Data<minijinja::Environment>>()
        .and_then(|tmpl_env| tmpl_env.get_template("error.html").ok())
        .and_then(|tmpl| tmpl.render(ctx).ok())
    {
        Some(body) => Html(body)
            .customize()
            .with_status(res.status())
            .respond_to(&req)
            .map_into_boxed_body(),

        None => fallback(error),
    }
}
