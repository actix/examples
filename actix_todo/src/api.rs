use actix_files::NamedFile;
use actix_session::Session;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, http, web, Error, HttpResponse, Responder, Result};
use futures::future::{err, Either, Future, IntoFuture};
use tera::{Context, Tera};

use crate::db;
use crate::session::{self, FlashMessage};

pub fn index(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || db::get_all_tasks(&pool))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => {
                let mut context = Context::new();
                context.insert("tasks", &tasks);

                //Session is set during operations on other endpoints
                //that can redirect to index
                if let Some(flash) = session::get_flash(&session)? {
                    context.insert("msg", &(flash.kind, flash.message));
                    session::clear_flash(&session);
                }

                let rendered =
                    tmpl.render("index.html.tera", &context).map_err(|e| {
                        error::ErrorInternalServerError(e.description().to_owned())
                    })?;

                Ok(HttpResponse::Ok().body(rendered))
            }
            Err(e) => Err(e),
        })
}

#[derive(Deserialize)]
pub struct CreateForm {
    description: String,
}

pub fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<db::PgPool>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = Error> {
    if params.description.is_empty() {
        Either::A(
            session::set_flash(
                &session,
                FlashMessage::error("Description cannot be empty"),
            )
            .map(|_| redirect_to("/"))
            .into_future(),
        )
    } else {
        Either::B(
            web::block(move || db::create_task(params.into_inner().description, &pool))
                .from_err()
                .then(move |res| match res {
                    Ok(_) => {
                        session::set_flash(
                            &session,
                            FlashMessage::success("Task successfully added"),
                        )?;
                        Ok(redirect_to("/"))
                    }
                    Err(e) => Err(e),
                }),
        )
    }
}

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Deserialize)]
pub struct UpdateForm {
    _method: String,
}

pub fn update(
    db: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
    form: web::Form<UpdateForm>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match form._method.as_ref() {
        "put" => Either::A(Either::A(toggle(db, params))),
        "delete" => Either::A(Either::B(delete(db, params, session))),
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {}", unsupported_method);
            Either::B(err(error::ErrorBadRequest(msg)))
        }
    }
}

fn toggle(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || db::toggle_task(params.id, &pool))
        .from_err()
        .then(move |res| match res {
            Ok(_) => Ok(redirect_to("/")),
            Err(e) => Err(e),
        })
}

fn delete(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || db::delete_task(params.id, &pool))
        .from_err()
        .then(move |res| match res {
            Ok(_) => {
                session::set_flash(
                    &session,
                    FlashMessage::success("Task was deleted."),
                )?;
                Ok(redirect_to("/"))
            }
            Err(e) => Err(e),
        })
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}
