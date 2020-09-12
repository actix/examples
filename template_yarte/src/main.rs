use std::collections::HashMap;

use actix_web::{
    error::ErrorInternalServerError, get, middleware::Logger, web, App, Error,
    HttpResponse, HttpServer,
};
use yarte::TemplateMin;

#[derive(TemplateMin)]
#[template(path = "index")]
struct IndexTemplate {
    query: web::Query<HashMap<String, String>>,
}

#[get("/")]
async fn index(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    IndexTemplate { query }
        .call()
        .map(|body| {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body)
        })
        .map_err(|_| ErrorInternalServerError("Some error message"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || App::new().wrap(Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http, test as atest, web::Bytes};

    #[actix_rt::test]
    async fn test() {
        let mut app = atest::init_service(App::new().service(index)).await;

        let req = atest::TestRequest::with_uri("/").to_request();
        let resp = atest::call_service(&mut app, req).await;

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
                 web</title></head><body><h1 id=\"welcome\" \
                 class=\"welcome\">Welcome!</h1><div><h3>What is your name?</h3><form>Name: \
                 <input type=\"text\" name=\"name\"><br>Last name: <input type=\"text\" \
                 name=\"lastname\"><br><p><input type=\"submit\"></p></form></div></body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo&lastname=bar").to_request();
        let resp = atest::call_service(&mut app, req).await;

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
                 web</title></head><body><h1>Hi, foo bar!</h1><p id=\"hi\" \
                 class=\"welcome\">Welcome</p></body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo").to_request();
        let resp = atest::call_service(&mut app, req).await;

        assert!(resp.status().is_server_error());

        let bytes = atest::read_body(resp).await;

        assert_eq!(bytes, Bytes::from_static("Some error message".as_ref()));

        let req = atest::TestRequest::with_uri("/?lastname=bar").to_request();
        let resp = atest::call_service(&mut app, req).await;

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
                 web</title></head><body><h1 id=\"welcome\" \
                 class=\"welcome\">Welcome!</h1><div><h3>What is your name?</h3><form>Name: \
                 <input type=\"text\" name=\"name\"><br>Last name: <input type=\"text\" \
                 name=\"lastname\"><br><p><input type=\"submit\"></p></form></div></body></html>"
                    .as_ref()
            )
        );
    }
}
