extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::{middleware, server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}



#[cfg(test)]
mod tests {
    use actix_web::HttpResponse;
use super::*;

    use actix_web::Binary::Slice;
    use actix_web::Body::Binary;
    use actix_web::Error;
    use actix_web::{http, test};

    #[test]
    fn test_index() -> Result<(), Error>  {
        let response: HttpResponse = test::TestRequest::default()
            .run(&index)?;
        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = match response.body() {
            Binary(body) => match body {
                Slice(s) => String::from_utf8(s.to_vec()).unwrap(),
                _ => panic!("error")
            },
            _ => panic!("error2")
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}
