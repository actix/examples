use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};
use serde::Deserialize;
use super::AppState;
use super::log_request;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_group_by_id);
    cfg.service(post_group);
    cfg.service(patch_group_by_name);
    cfg.service(delete_group_by_name);
}

// TODO: provide response headers
#[get("/group/{id}")]
async fn get_group_by_id(group_id: web::Path<u64>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("GET: /group", &app_state.connections);

    let x = app_state.context.groups.get_group_by_id(group_id.into_inner()).await;

    // TODO: return 404 on not found & 500 on bad request data
    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(group) => HttpResponse::Ok().json(group)
    }
}

// TODO: provide response headers
#[post("/group")]
async fn post_group(group: web::Json<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("POST: /group", &app_state.connections);

    // TODO: modify add_group to return the group that was added
    let x = app_state.context.groups.add_group(group.as_str()).await;

    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(group.into_inner()) // TODO: as per above ^ return the id of the inserted group
    }
}

#[derive(Deserialize)]
struct GroupUpdate {
    pub old: String,
    pub new: String,
}

// TODO: provide response headers
#[patch("/group")]
async fn patch_group_by_name(update: web::Json<GroupUpdate>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    // TODO: modify update_group to return the group that was added
    let x = app_state.context.groups.update_group(&update.old, &update.new).await;

    // TODO: better error handling
    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(&update.new) // TODO: as per above ^ return the id of the inserted group
    }
}

// TODO: provide response headers
#[delete("/group/{name}")]
async fn delete_group_by_name(name: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("DELETE: /group", &app_state.connections);

    let x = app_state.context.groups.delete_group(name.as_str()).await;

    // TODO: better error handling
    match x {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted group {}", name))
    }
}