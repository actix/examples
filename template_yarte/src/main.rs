#[macro_use]
extern crate actix_web;

use std::collections::HashMap;

use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use yarte::Template;

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexTemplate {
    query: web::Query<HashMap<String, String>>,
}

#[get("/")]
pub fn index(query: web::Query<HashMap<String, String>>) -> impl Responder {
    IndexTemplate { query }
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(move || App::new().wrap(Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http, test as atest};
    use bytes::Bytes;

    #[test]
    fn test() {
        let mut app = atest::init_service(App::new().service(index));

        let req = atest::TestRequest::with_uri("/").to_request();
        let resp = atest::call_service(&mut app, req);

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp);
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html>\
                 <html>\
                 <head><meta charset=\"utf-8\" /><title>Actix web</title></head><body>\
                 <h1 id=\"welcome\" class=\"welcome\">Welcome!</h1><div>\
                 <h3>What is your name?</h3>\
                 <form>\
                 Name: <input type=\"text\" name=\"name\" />\
                 <br/>Last name: <input type=\"text\" name=\"lastname\" />\
                 <br/><p><input type=\"submit\"></p></form>\
                 </div>\
                 </body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo&lastname=bar").to_request();
        let resp = atest::call_service(&mut app, req);

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp);
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html>\
                 <html>\
                 <head><meta charset=\"utf-8\" /><title>Actix web</title></head>\
                 <body>\
                 <h1>Hi, foo bar!</h1><p id=\"hi\" class=\"welcome\">Welcome</p>\
                 </body></html>"
                    .as_ref()
            )
        );

        let req = atest::TestRequest::with_uri("/?name=foo").to_request();
        let resp = atest::call_service(&mut app, req);

        assert!(resp.status().is_server_error());

        let req = atest::TestRequest::with_uri("/?lastname=bar").to_request();
        let resp = atest::call_service(&mut app, req);

        assert!(resp.status().is_success());

        assert_eq!(
            resp.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let bytes = atest::read_body(resp);
        assert_eq!(
            bytes,
            Bytes::from_static(
                "<!DOCTYPE html>\
                 <html>\
                 <head><meta charset=\"utf-8\" /><title>Actix web</title></head><body>\
                 <h1 id=\"welcome\" class=\"welcome\">Welcome!</h1><div>\
                 <h3>What is your name?</h3>\
                 <form>\
                 Name: <input type=\"text\" name=\"name\" />\
                 <br/>Last name: <input type=\"text\" name=\"lastname\" />\
                 <br/><p><input type=\"submit\"></p></form>\
                 </div>\
                 </body></html>"
                    .as_ref()
            )
        );
    }
}
