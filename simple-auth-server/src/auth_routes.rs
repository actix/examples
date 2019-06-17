use actix::Addr;
use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use futures::Future;

use crate::auth_handler::{AuthData, LoggedUser};
use crate::models::DbExecutor;
use crate::utils::create_token;

pub fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(auth_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let token = create_token(&user)?;
                id.remember(token);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response()),
        })
}

pub fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
