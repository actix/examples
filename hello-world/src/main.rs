use actix_web::{middleware, web, App, HttpRequest, HttpServer};

fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| "Hello world!"))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::dev::Body::Bytes;
    use actix_web::dev::ResponseBody::{Other};
    use actix_web::HttpResponse;
    use actix_web::Error;
    use actix_web::{http, test};

     #[test]
    fn test_index() -> Result<(), Error>  {
        let response: HttpResponse = test::TestRequest::default()
            .run(&index)?;
        assert_eq!(response.status(), http::StatusCode::OK);

         let response_body = match response.body() {
            Other(body) => match body {
                Bytes(bytes) => String::from_utf8(bytes.to_vec()).unwrap(),
                _ => panic!("Unknow body type: #1")
            },
            _ => panic!("Unknow body type: #2")
        };

         assert_eq!(response_body, r##"Hello world!"##);

         Ok(())
    }
}
