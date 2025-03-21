use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, middleware, web};
use serde::{Deserialize, Serialize};

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .app_data(web::Data::new(AppState {
                foo: "bar".to_owned(),
            }))
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
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {}, and in AppState I have foo: {}",
        params.name, state.foo
    )))
}

/// Request and POST Params
async fn handle_post_3(req: HttpRequest, params: web::Form<MyParams>) -> impl Responder {
    println!("Handling POST request: {req:?}");

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name))
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::to_bytes,
        dev::ServiceResponse,
        http::{
            StatusCode,
            header::{CONTENT_TYPE, HeaderValue},
        },
        test::{self, TestRequest},
        web::{Bytes, Form},
    };

    use super::*;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn handle_post_1_unit_test() {
        let params = Form(MyParams {
            name: "John".to_owned(),
        });
        let resp = handle_post_1(params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_1_integration_test() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post1")
            .set_form(MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_2_unit_test() {
        let state = TestRequest::default()
            .data(AppState {
                foo: "bar".to_owned(),
            })
            .to_http_request();
        let data = state.app_data::<actix_web::web::Data<AppState>>().unwrap();
        let params = Form(MyParams {
            name: "John".to_owned(),
        });
        let resp = handle_post_2(data.clone(), params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "Your name is John, and in AppState I have foo: bar"
        );
    }

    #[actix_web::test]
    async fn handle_post_2_integration_test() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post2")
            .set_form(MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let resp = resp.into_parts().1;
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "Your name is John, and in AppState I have foo: bar"
        );
    }

    #[actix_web::test]
    async fn handle_post_3_unit_test() {
        let req = TestRequest::default().to_http_request();
        let params = Form(MyParams {
            name: "John".to_owned(),
        });
        let result = handle_post_3(req.clone(), params).await;
        let resp = result.respond_to(&req);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );
        let body = match to_bytes(resp.into_body()).await {
            Ok(x) => x,
            _ => panic!(),
        };
        assert_eq!(body.as_str(), "Your name is John");
    }

    #[actix_web::test]
    async fn handle_post_3_integration_test() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post()
            .uri("/post3")
            .set_form(MyParams {
                name: "John".to_owned(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain")
        );

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Your name is John");
    }
}
