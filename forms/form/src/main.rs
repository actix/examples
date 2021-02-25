use serde::{Deserialize, Serialize};

use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

struct AppState {
    foo: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .data(AppState {
                foo: "bar".to_string(),
            })
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/post1").route(web::post().to(handle_post_1)))
            .service(web::resource("/post2").route(web::post().to(handle_post_2)))
            .service(web::resource("/post3").route(web::post().to(handle_post_3))),
    );
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Serialize, Deserialize)]
pub struct MyParams {
    name: String,
}

/// Simple handle POST request
async fn handle_post_1(params: web::Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

/// State and POST Params
async fn handle_post_2(
    state: web::Data<AppState>,
    params: web::Form<MyParams>,
) -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {}, and in AppState I have foo: {}",
        params.name, state.foo
    ))
}

/// Request and POST Params
async fn handle_post_3(req: HttpRequest, params: web::Form<MyParams>) -> impl Responder {
    println!("Handling POST request: {:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::body::{Body, ResponseBody};
    use actix_web::dev::{HttpResponseBuilder, Service, ServiceResponse};
    use actix_web::http::{header::CONTENT_TYPE, HeaderValue, StatusCode};
    use actix_web::test::{self, TestRequest};
    use actix_web::web::Form;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for ResponseBody<Body> {
        fn as_str(&self) -> &str {
            match self {
                ResponseBody::Body(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
                ResponseBody::Other(ref b) => match b {
                    Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
                    _ => panic!(),
                },
            }
        }
    }

    #[actix_rt::test]
    async fn handle_post_1_unit_test() {
        let params = Form(MyParams {
            name: "John".to_string(),
        });
        let resp = handle_post_1(params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(resp.body().as_str(), "Your name is John");
    }

    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let mut app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post1")
            .set_form(&MyParams {
                name: "John".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(resp.response().body().as_str(), "Your name is John");
    }

    #[actix_rt::test]
    async fn handle_post_2_unit_test() {
        let state = TestRequest::default()
            .data(AppState {
                foo: "bar".to_string(),
            })
            .to_http_request();
        let data = state.app_data::<actix_web::web::Data<AppState>>().unwrap();
        let params = Form(MyParams {
            name: "John".to_string(),
        });
        let resp = handle_post_2(data.clone(), params).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(
            resp.body().as_str(),
            "Your name is John, and in AppState I have foo: bar"
        );
    }

    #[actix_rt::test]
    async fn handle_post_2_integration_test() {
        let mut app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post2")
            .set_form(&MyParams {
                name: "John".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(
            resp.response().body().as_str(),
            "Your name is John, and in AppState I have foo: bar"
        );
    }

    #[actix_rt::test]
    async fn handle_post_3_unit_test() {
        let req = TestRequest::default().to_http_request();
        let params = Form(MyParams {
            name: "John".to_string(),
        });
        let result = handle_post_3(req.clone(), params).await;
        let resp = match result.respond_to(&req).await {
            Ok(t) => t,
            Err(_) => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish()
            }
        };

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(resp.body().as_str(), "Your name is John");
    }

    #[actix_rt::test]
    async fn handle_post_3_integration_test() {
        let mut app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post3")
            .set_form(&MyParams {
                name: "John".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        assert_eq!(resp.response().body().as_str(), "Your name is John");
    }
}
