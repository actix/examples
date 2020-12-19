use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};
use super::AppState;
use super::log_request;
use std::sync::Mutex;
use crate::model::User;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(post_user);
    cfg.service(patch_user);
    cfg.service(delete_user);
}

// TODO: provide response headers
// TODO: use a Uuid in the path instead of a string
#[get("/user/{id}")]
async fn get_user(user_id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("GET: /user", &app_state.connections);

    let user = app_state.context.users.get_user_by_id(&user_id).await;

    // TODO: return 404 on not found & 500 on bad request data (^ see above about Uuid)
    match user {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(mut user) => {
            let groups = app_state.context.users_to_groups
                .get_groups_by_user_id(&user.id)
                .await;

            match groups {
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
                Ok(groups) => {
                    user.groups = groups;
                    HttpResponse::Ok().json(user)
                }
            }
        }
    }
}

// TODO: provide response headers
#[post("/user")]
async fn post_user(user: web::Json<User>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("POST: /user", &app_state.connections);

    let mut user = user.into_inner();
    user.id = Uuid::new_v4().to_string();

    let x = app_state.context.users.add_user(&user).await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(user.id)
    }
}

// TODO: provide response headers
#[patch("/user")]
async fn patch_user(user: web::Json<User>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    let user = user.into_inner();

    let x = app_state.context.users.update_user(&user).await;

    // TODO: better error handling
    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(user.id)
    }
}

// TODO: provide response headers
#[delete("/user/{id}")]
async fn delete_user(id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("DELETE: /user", &app_state.connections);

    let x = app_state.context.users.delete_user(id.as_str()).await;

    // TODO: better error handling
    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted user {}", id))
    }
}