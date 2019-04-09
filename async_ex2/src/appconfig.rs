use actix_web::{error, web};
use bytes::Bytes;
use futures::Stream;

use crate::{
    handlers::{
        products,
        parts
    },
};


pub fn config_app<P>(cfg: &mut web::RouterConfig<P>) 
                    where P: Stream<Item = Bytes, Error = error::PayloadError> 
                             + 'static 
{
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.service(
        web::scope("/products")
            .service(
                web::resource("")
                    .route(web::get().to_async(products::get_products))
                    .route(web::post().to_async(products::add_product))
            )
            .service(
                web::scope("/{product_id}")
                    .service(
                        web::resource("")
                            .route(web::get().to_async(products::get_product_detail))
                            .route(web::delete().to_async(products::remove_product))
                    )
                    .service(
                        web::scope("/parts")
                            .service(
                                web::resource("")
                                    .route(web::get().to_async(parts::get_parts))
                                    .route(web::post().to_async(parts::add_part))
                            )
                            .service(
                                web::resource("/{part_id}")
                                    .route(web::get().to_async(parts::get_part_detail))
                                    .route(web::delete().to_async(parts::remove_part))
                            )
                    )
            )
    );
}