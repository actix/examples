use std::collections::HashMap;

use actix_web::{
    App, Error, HttpResponse, HttpServer, Result,
    body::BoxBody,
    dev::ServiceResponse,
    error,
    http::{StatusCode, header::ContentType},
    middleware,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web,
};
use serde_json::json;
use tinytemplate::TinyTemplate;

// store tiny_template in application state
async fn index(
    tmpl: web::Data<TinyTemplate<'_>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        // submitted form
        let ctx = json!({
          "name" : name.to_owned(),
          "text" : "Welcome!".to_owned()
        });
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("index.html", &serde_json::Value::Null)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", INDEX).unwrap();
        tt.add_template("user.html", USER).unwrap();
        tt.add_template("error.html", ERROR).unwrap();

        App::new()
            .app_data(web::Data::new(tt))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let tt = request
        .app_data::<web::Data<TinyTemplate<'_>>>()
        .map(|t| t.get_ref());
    match tt {
        Some(tt) => {
            let mut context = std::collections::HashMap::new();
            context.insert("error", error.to_owned());
            context.insert("status_code", res.status().as_str().to_owned());
            let body = tt.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

static ERROR: &str = include_str!("../templates/error.html");
static INDEX: &str = include_str!("../templates/index.html");
static USER: &str = include_str!("../templates/user.html");
