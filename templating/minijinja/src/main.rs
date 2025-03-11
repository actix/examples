use std::{collections::HashMap, env, path::PathBuf};

use actix_utils::future::{Ready, ready};
use actix_web::{
    App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, Result,
    dev::{self, ServiceResponse},
    error,
    http::{StatusCode, header::ContentType},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    web,
};
use minijinja::path_loader;
use minijinja_autoreload::AutoReloader;

struct MiniJinjaRenderer {
    tmpl_env: web::Data<minijinja_autoreload::AutoReloader>,
}

impl MiniJinjaRenderer {
    fn render<T: Into<minijinja::value::Value>>(
        &self,
        tmpl: &str,
        ctx: T,
    ) -> actix_web::Result<impl Responder + use<T>> {
        self.tmpl_env
            .acquire_env()
            .map_err(|_| error::ErrorInternalServerError("could not acquire template env"))?
            .get_template(tmpl)
            .map_err(|_| error::ErrorInternalServerError("could not find template"))?
            .render(ctx.into())
            .map(web::Html::new)
            .map_err(|err| {
                log::error!("{err}");
                error::ErrorInternalServerError("template error")
            })
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let tmpl_env = <web::Data<minijinja_autoreload::AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { tmpl_env }))
    }
}

async fn index(
    tmpl_env: MiniJinjaRenderer,
    query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<impl Responder> {
    if let Some(name) = query.get("name") {
        tmpl_env.render(
            "user.html",
            minijinja::context! {
                name,
                text => "Welcome!",
            },
        )
    } else {
        tmpl_env.render("index.html", minijinja::Value::UNDEFINED)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // If TEMPLATE_AUTORELOAD is set, then the path tracking is enabled.
    let enable_template_autoreload = env::var("TEMPLATE_AUTORELOAD").as_deref() == Ok("true");

    if enable_template_autoreload {
        log::info!("template auto-reloading is enabled");
    } else {
        log::info!(
            "template auto-reloading is disabled; run with TEMPLATE_AUTORELOAD=true to enable"
        );
    }

    // The closure is invoked every time the environment is outdated to recreate it.
    let tmpl_reloader = AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        // if watch_path is never called, no fs watcher is created
        if enable_template_autoreload {
            notifier.watch_path(&tmpl_path, true);
        }

        env.set_loader(path_loader(tmpl_path));

        Ok(env)
    });

    let tmpl_reloader = web::Data::new(tmpl_reloader);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(tmpl_reloader.clone())
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

    let tmpl_env = MiniJinjaRenderer::extract(req).into_inner().unwrap();

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

    match tmpl_env.render("error.html", ctx) {
        Ok(body) => body
            .customize()
            .with_status(res.status())
            .respond_to(req)
            .map_into_boxed_body(),

        Err(_) => fallback(error),
    }
}
