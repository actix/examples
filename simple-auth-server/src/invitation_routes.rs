use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::future::Future;

use crate::email_service::send_invitation;
use crate::invitation_handler::CreateInvitation;
use crate::models::DbExecutor;

pub fn register_email(
    signup_invitation: web::Json<CreateInvitation>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(signup_invitation.into_inner())
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(invitation) => {
                send_invitation(&invitation);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response()),
        })
}
