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
    use actix_http::HttpService;
    use actix_http_test::TestServer;
    use actix_web::{http, App};
    use bytes::Bytes;

    #[test]
    fn test() {
        let mut srv = TestServer::new(|| HttpService::new(App::new().service(index)));

        let req = srv.get();
        let response = srv.block_on(req.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html"
        );

        let bytes = srv.block_on(response.body()).unwrap();
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

        let req = srv.get().uri(srv.url("/?name=foo&lastname=bar"));
        let response = srv.block_on(req.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html"
        );

        let bytes = srv.block_on(response.body()).unwrap();
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

        let req = srv.get().uri(srv.url("/?name=foo"));
        let response = srv.block_on(req.send()).unwrap();
        assert!(response.status().is_server_error());

        let req = srv.get().uri(srv.url("/?lastname=bar"));
        let response = srv.block_on(req.send()).unwrap();
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE).unwrap(),
            "text/html"
        );

        let bytes = srv.block_on(response.body()).unwrap();
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
