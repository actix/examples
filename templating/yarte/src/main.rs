use std::collections::HashMap;

use actix_web::{
    App, Error, HttpResponse, HttpServer, ResponseError, get, middleware::Logger, web,
};
use derive_more::Display;
use yarte::{auto, ywrite_min};

#[derive(Debug, Display)]
struct MyErr(pub &'static str);

impl ResponseError for MyErr {}

#[allow(unused_must_use)] // ywrite_min causes warning: unused borrow that must be used
#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    // `ywrite_min` is work in progress check your templates before put in production
    // or use `ywrite_html`
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(auto!(ywrite_min!(String, "{{> index }}"))))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || App::new().wrap(Logger::default()).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use actix_web::{http, test as atest, web::Bytes};

    use super::*;

    #[actix_web::test]
    async fn test() {
        let app = atest::init_service(App::new().service(index)).await;

        let req = atest::TestRequest::with_uri("/").to_request();
        let resp = atest::call_service(&app, req).await;

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp).await;
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Actix \
                 Web</title></head><body><h1 id=\"welcome\" \
                 class=\"welcome\">Welcome!</h1><div><h3>What is your name?</h3><form>Name: \
                 <input type=\"text\" name=\"name\"><br>Last name: <input type=\"text\" \
                 name=\"lastname\"><br><p><input type=\"submit\"></p></form></div></body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo&lastname=bar").to_request();
        let resp = atest::call_service(&app, req).await;

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp).await;
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Actix \
                 Web</title></head><body><h1>Hi, foo bar!</h1><p id=\"hi\" \
                 class=\"welcome\">Welcome</p></body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo").to_request();
        let resp = atest::call_service(&app, req).await;

        assert!(resp.status().is_server_error());

        let bytes = atest::read_body(resp).await;

        assert_eq!(bytes, Bytes::from_static("Bad query".as_ref()));

        let req = atest::TestRequest::with_uri("/?lastname=bar").to_request();
        let resp = atest::call_service(&app, req).await;

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp).await;
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Actix \
                 Web</title></head><body><h1 id=\"welcome\" \
                 class=\"welcome\">Welcome!</h1><div><h3>What is your name?</h3><form>Name: \
                 <input type=\"text\" name=\"name\"><br>Last name: <input type=\"text\" \
                 name=\"lastname\"><br><p><input type=\"submit\"></p></form></div></body></html>"
                    .as_ref()
            )
        );
    }
}
