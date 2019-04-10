use actix_multipart::{Field, Item, Multipart, MultipartError};
use actix_web::{HttpResponse, web, error, Error};
use futures::{future::{ok as fut_ok, err as fut_err, Either}, Future, Stream};

use crate::common::{Part, Product};


pub fn get_products(query: web::Query<Option<Part>>)
                    -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn add_product(new_product: web::Json<Product>)
                   -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn get_product_detail(id: web::Path<String>)
                   -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}

pub fn remove_product(id: web::Path<String>)
                   -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_service::Service;
    use actix_web::{test, HttpResponse, HttpRequest, App,
                    http::{header, StatusCode}, web};
    use crate::appconfig::config_app;


    #[test]
    fn test_add_product() {
        let mut app = test::init_service(
           App::new()
               .configure(config_app)
        );

        let payload = r#"{"id":12345,"product_type":"fancy","name":"test"}"#.as_bytes();

        let req = test::TestRequest::post()
                            .uri("/products")
                            .header(header::CONTENT_TYPE, "application/json")
                            .set_payload(payload)
                            .to_request();

        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
