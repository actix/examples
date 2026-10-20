use std::io;

use actix_csrf_middleware::{
    CsrfMiddleware, CsrfMiddlewareConfig, CsrfRequestExt, CsrfToken, DEFAULT_SESSION_ID_KEY,
};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    cookie::{Cookie, SameSite},
    http::header::LOCATION,
    middleware, web,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Message {
    text: String,
}

fn page(csrf: &str, signed_in: bool) -> HttpResponse {
    let body = if signed_in {
        format!(
            r#"<!DOCTYPE html>
<title>Double Submit Cookie</title>
<p>Signed in. The token is now bound to your session.</p>
<form method="post" action="/message">
  <input type="hidden" name="csrf_token" value="{csrf}">
  <input name="text" value="hello">
  <button type="submit">Send</button>
</form>
<form method="post" action="/logout">
  <input type="hidden" name="csrf_token" value="{csrf}">
  <button type="submit">Sign out</button>
</form>"#
        )
    } else {
        format!(
            r#"<!DOCTYPE html>
<title>Double Submit Cookie</title>
<p>Anonymous. The token is bound to a pre-session.</p>
<form method="post" action="/login">
  <input type="hidden" name="csrf_token" value="{csrf}">
  <button type="submit">Sign in</button>
</form>"#
        )
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

async fn index(req: HttpRequest, csrf: CsrfToken) -> HttpResponse {
    page(&csrf.0, req.cookie(DEFAULT_SESSION_ID_KEY).is_some())
}

async fn login(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let session_id = "example-session-id";

    let mut resp = HttpResponse::SeeOther();
    resp.cookie(
        Cookie::build(DEFAULT_SESSION_ID_KEY, session_id)
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .finish(),
    );

    req.rotate_csrf_after_login(session_id, &mut resp)?;

    resp.append_header((LOCATION, "/"));

    Ok(resp.finish())
}

async fn logout(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let mut resp = HttpResponse::SeeOther();

    req.rotate_csrf_after_logout(&mut resp)?;

    resp.append_header((LOCATION, "/"));

    Ok(resp.finish())
}

async fn message(form: web::Form<Message>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!("accepted: {}", form.text))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let secret = b"example-secret-key-of-at-least-32-bytes";

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let csrf = CsrfMiddlewareConfig::double_submit_cookie(secret).with_secure(false);

        App::new()
            .wrap(CsrfMiddleware::new(csrf))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/message", web::post().to(message))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
