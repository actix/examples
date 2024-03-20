use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(information_handler)
            .service(success_handler)
            .service(redirect_handler)
            .service(client_error_handler)
            .service(server_error_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn information_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn success_handler() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn redirect_handler() -> impl Responder {
    HttpResponse::TemporaryRedirect().finish()
}

#[get("/")]
async fn client_error_handler() -> impl Responder {
    HttpResponse::ImATeapot().finish()
}

#[get("/")]
async fn server_error_handler() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{dev::Service, http, test, App, Error};

    use super::*;

    #[actix_web::test]
    /// Test informational status codes `1xx`
    async fn test_informational() -> Result<(), Error> {
        let app = App::new().service(information_handler);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        // This matches the exact value returned from `information_handler`
        assert_eq!(resp.status(), http::StatusCode::CONTINUE);

        // This matches all values considered _informational_ `1xx`
        assert!(resp.status().is_informational());

        Ok(())
    }

    #[actix_web::test]
    /// Test success status codes `2xx`
    async fn test_success() -> Result<(), Error> {
        let app = App::new().service(success_handler);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        // This matches the exact value returned from `success_handler`
        assert_eq!(resp.status(), http::StatusCode::OK);

        // This matches all values considered _successfull_ `2xx`
        assert!(resp.status().is_success());

        Ok(())
    }

    #[actix_web::test]
    /// Test redirect status codes `3xx`
    async fn test_redirect() -> Result<(), Error> {
        let app = App::new().service(redirect_handler);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        // This matches the exact value returned from `redirect_handler`
        assert_eq!(resp.status(), http::StatusCode::TEMPORARY_REDIRECT);

        // This matches all values considered _redirects_ `3xx`
        assert!(resp.status().is_redirection());

        Ok(())
    }

    #[actix_web::test]
    /// Test client error status codes `4xx`
    async fn test_client_error() -> Result<(), Error> {
        let app = App::new().service(client_error_handler);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        // This matches the exact value returned from `client_error_handler`
        assert_eq!(resp.status(), http::StatusCode::IM_A_TEAPOT);

        // This matches all values considered _client error_ `4xx`
        assert!(resp.status().is_client_error());

        Ok(())
    }

    #[actix_web::test]
    /// Test server error status codes `5xx`
    async fn test_server_error() -> Result<(), Error> {
        let app = App::new().service(server_error_handler);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        // This matches the exact value returned from `server_error_handler`
        assert_eq!(resp.status(), http::StatusCode::NOT_IMPLEMENTED);

        // This matches all values considered _server error_ `5xx`
        assert!(resp.status().is_server_error());

        Ok(())
    }
}
