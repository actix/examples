use actix_web::web;

use crate::routes::area_route;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/region")
        .route("/provinces", web::get().to(area_route::provinces))
        .route("/simple/provinces", web::get().to(area_route::list_simple_all_provinces))
        .route("/simple/cities", web::get().to(area_route::list_cities_by_province_id)),
    );
}
