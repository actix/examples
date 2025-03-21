use std::io;

use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result,
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{StatusCode, header::ContentType},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web,
};
use actix_web_lab::extract::Path;
use fluent_templates::{FluentLoader, Loader as _, static_loader};
use handlebars::{DirectorySourceOptions, Handlebars};
use serde_json::json;

mod lang_choice;
use self::lang_choice::LangChoice;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",

        // removes unicode isolating marks around arguments
        // you typically should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, lang: LangChoice) -> impl Responder {
    let data = json!({ "lang": lang });
    let body = hb.render("index", &data).unwrap();
    web::Html::new(body)
}

#[get("/{user}/{data}")]
async fn user(
    hb: web::Data<Handlebars<'_>>,
    Path(info): Path<(String, String)>,
    lang: LangChoice,
) -> impl Responder {
    let data = json!({
        "lang": lang,
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();
    web::Html::new(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Handlebars uses a repository for the compiled templates. This object must be shared between
    // the application threads, and is therefore passed to the App in an Arc.
    let mut handlebars = Handlebars::new();

    // register template dir with Handlebars registry
    handlebars
        .register_templates_directory(
            "./templates",
            DirectorySourceOptions {
                tpl_extension: ".html".to_owned(),
                hidden: false,
                temporary: false,
            },
        )
        .unwrap();

    // register Fluent helper with Handlebars registry
    handlebars.register_helper("fluent", Box::new(FluentLoader::new(&*LOCALES)));

    let handlebars = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .app_data(web::Data::clone(&handlebars))
            .service(index)
            .service(user)
    })
    .workers(2)
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
    let lang = LangChoice::from_req(res.request()).lang_id();
    let error = LOCALES.lookup(&lang, "error-not-found");

    let response = get_error_response(&res, &error);

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let req = res.request();
    let lang = LangChoice::from_req(req);

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.

    let hb = req
        .app_data::<web::Data<Handlebars>>()
        .expect("correctly set up handlebars in app data");

    let data = json!({
        "lang": lang,
        "error": error,
        "status_code": res.status().as_str()
    });

    let body = hb.render("error", &data);

    match body {
        Ok(body) => HttpResponse::build(res.status())
            .content_type(ContentType::html())
            .body(body),

        Err(_) => HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(error.to_string()),
    }
}
