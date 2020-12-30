use super::log_request;
use super::AppState;
use crate::model::User;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(post_user);
    cfg.service(patch_user);
    cfg.service(delete_user);
}

#[get("/user/{id}")]
async fn get_user(
    user_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("GET: /user", &app_state.connections);

    let user = app_state.context.users.get_user_by_id(&user_id).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(mut user) => {
            let groups = app_state
                .context
                .users_to_groups
                .get_groups_by_user_id(&user.id)
                .await;

            match groups {
                Err(_) => HttpResponse::InternalServerError().finish(),
                Ok(groups) => {
                    user.groups = groups;
                    HttpResponse::Ok().json(user)
                }
            }
        }
    }
}

#[post("/user")]
async fn post_user(
    user: web::Json<User>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("POST: /user", &app_state.connections);

    let mut user = user.into_inner();
    user.id = Uuid::new_v4().to_string();

    let x = app_state.context.users.add_user(&user).await;

    match x {
        Ok(_) => {
            if user.groups.len() > 0 {
                let _ = app_state
                    .context
                    .users_to_groups
                    .add_user_groups(&user.id, &user.groups)
                    .await;
            }
            HttpResponse::Accepted().body(user.id)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/user")]
async fn patch_user(
    user: web::Json<User>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    let user = user.into_inner();

    let x = app_state.context.users.update_user(&user).await;

    match x {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => {
            let _ = app_state
                .context
                .users_to_groups
                .update_user_groups(&user)
                .await;
            HttpResponse::Accepted().json(user)
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/user/{id}")]
async fn delete_user(
    id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("DELETE: /user", &app_state.connections);

    let x = app_state.context.users.delete_user(id.as_str()).await;

    match x {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
