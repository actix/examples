use actix_web::{
    App, Error, HttpServer, Responder,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    get,
    guard::{Guard, GuardContext},
    middleware::DefaultHeaders,
    web,
};

mod v1 {
    use super::*;

    pub struct ApiGuard;

    impl Guard for ApiGuard {
        fn check(&self, ctx: &GuardContext<'_>) -> bool {
            ctx.head()
                .headers()
                .get("Accept-Version")
                .is_some_and(|hv| hv.as_bytes() == b"1")
        }
    }

    #[get("/hello")]
    pub async fn hello() -> impl Responder {
        "Hello World from v1 API!"
    }
}

mod v2 {
    use super::*;

    pub struct ApiGuard;

    impl Guard for ApiGuard {
        fn check(&self, ctx: &GuardContext<'_>) -> bool {
            ctx.head()
                .headers()
                .get("Accept-Version")
                .is_some_and(|hv| hv.as_bytes() == b"2")
        }
    }

    #[get("/hello")]
    pub async fn hello() -> impl Responder {
        "Hello World from the awesome new v2 API!"
    }
}

fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .service(web::scope("/api").guard(v1::ApiGuard).service(v1::hello))
        .service(web::scope("/api").guard(v2::ApiGuard).service(v2::hello))
        // using this form of API version selection means that we need to send a Vary header so that
        // caches won't try to serve the wrong response
        .wrap(DefaultHeaders::new().add(("Vary", "Accept-Version")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(create_app)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::test::{self, TestRequest};

    use super::*;

    #[actix_web::test]
    async fn api_versioning() {
        let app = test::init_service(create_app()).await;

        let req = TestRequest::with_uri("/api/hello").insert_header(("Accept-Version", "1"));
        let res = test::call_and_read_body(&app, req.to_request()).await;
        assert_eq!(res, "Hello World from v1 API!");

        let req = TestRequest::with_uri("/api/hello").insert_header(("Accept-Version", "2"));
        let res = test::call_and_read_body(&app, req.to_request()).await;
        assert_eq!(res, "Hello World from the awesome new v2 API!");
    }
}
