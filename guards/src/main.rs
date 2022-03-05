use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    get,
    guard::{Guard, GuardContext},
    middleware::DefaultHeaders,
    web, App, Error, HttpServer, Responder,
};

mod v1 {
    use super::*;

    pub struct ApiGuard;

    impl Guard for ApiGuard {
        fn check(&self, ctx: &GuardContext<'_>) -> bool {
            ctx.head()
                .headers()
                .get("Accept-Version")
                .map_or(false, |hv| hv.as_bytes() == b"1")
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
                .map_or(false, |hv| hv.as_bytes() == b"2")
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
