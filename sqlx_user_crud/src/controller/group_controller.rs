use super::log_request;
use super::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_group_by_id);
    cfg.service(post_group);
    cfg.service(patch_group_by_name);
    cfg.service(delete_group_by_name);
}

#[get("/group/{id}")]
async fn get_group_by_id(
    group_id: web::Path<u64>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("GET: /group", &app_state.connections);

    let x = app_state
        .context
        .groups
        .get_group_by_id(group_id.into_inner())
        .await;

    match x {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(group) => HttpResponse::Ok().json(group),
    }
}

#[post("/group")]
async fn post_group(
    group: web::Json<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("POST: /group", &app_state.connections);

    let x = app_state.context.groups.add_group(group.as_str()).await;

    match x {
        Ok(_) => {
            let group = app_state
                .context
                .groups
                .get_group_by_name(group.as_str())
                .await;

            match group {
                Ok(g) => HttpResponse::Accepted().json(g),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupUpdate {
    pub old: String,
    pub new: String,
}

#[patch("/group")]
async fn patch_group_by_name(
    update: web::Json<GroupUpdate>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    let x = app_state
        .context
        .groups
        .update_group(&update.old, &update.new)
        .await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(&update.new),
    }
}

#[delete("/group/{name}")]
async fn delete_group_by_name(
    name: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("DELETE: /group", &app_state.connections);

    let x = app_state.context.groups.delete_group(name.as_str()).await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted group {}", name)),
    }
}
