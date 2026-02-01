use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{
    Error, HttpResponse, Responder, Result, dev, error, http::StatusCode,
    middleware::ErrorHandlerResponse, web,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tera::{Context, Tera};

use crate::{
    db,
    session::{self, FlashMessage},
};

pub async fn index(
    pool: web::Data<SqlitePool>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let tasks = db::get_all_tasks(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut context = Context::new();
    context.insert("tasks", &tasks);

    // Session is set during operations on other endpoints that can redirect to index
    if let Some(flash) = session::get_flash(&session)? {
        context.insert("msg", &(flash.kind, flash.message));
        session::clear_flash(&session);
    }

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    description: String,
}

pub async fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<impl Responder, Error> {
    if params.description.is_empty() {
        session::set_flash(&session, FlashMessage::error("Description cannot be empty"))?;
        Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    } else {
        db::create_task(params.into_inner().description, &pool)
            .await
            .map_err(error::ErrorInternalServerError)?;

        session::set_flash(&session, FlashMessage::success("Task successfully added"))?;

        Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateForm {
    _method: String,
}

pub async fn update(
    db: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
    form: web::Form<UpdateForm>,
    session: Session,
) -> Result<impl Responder, Error> {
    Ok(web::Redirect::to(match form._method.as_ref() {
        "put" => toggle(db, params).await?,
        "delete" => delete(db, params, session).await?,
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {unsupported_method}");
            return Err(error::ErrorBadRequest(msg));
        }
    })
    .using_status_code(StatusCode::FOUND))
}

async fn toggle(
    pool: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
) -> Result<&'static str, Error> {
    db::toggle_task(params.id, &pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok("/")
}

async fn delete(
    pool: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
    session: Session,
) -> Result<&'static str, Error> {
    db::delete_task(params.id, &pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    session::set_flash(&session, FlashMessage::success("Task was deleted."))?;

    Ok("/")
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .customize()
        .with_status(res.status())
        .respond_to(res.request())
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
