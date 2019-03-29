use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

use crate::models::DbExecutor;
use crate::register_handler::{RegisterUser, UserData};

pub fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let msg = RegisterUser {
        // into_inner() returns the inner string value from Path
        invitation_id: invitation_id.into_inner(),
        password: user_data.password.clone(),
    };

    db.send(msg)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(slim_user) => Ok(HttpResponse::Ok().json(slim_user)),
            Err(service_error) => Ok(service_error.error_response()),
        })
}
