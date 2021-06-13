use crate::service::NewUser;
use crate::AppState;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

async fn index(app_state: web::Data<Arc<AppState>>) -> HttpResponse {
    let result = app_state.user.get_all().await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_user(
    app_state: web::Data<Arc<AppState>>,
    new_user_req: web::Json<NewUser>,
) -> HttpResponse {
    let new_user = new_user_req.into_inner();
    match app_state.user.create_user(new_user).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(index)),
    );
}
