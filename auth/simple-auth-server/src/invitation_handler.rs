use actix_web::{web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::email_service::send_invitation;
use crate::models::{Invitation, Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    // run diesel blocking code
    web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await??;

    Ok(HttpResponse::Ok().finish())
}

fn create_invitation(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let invitation = dbg!(query(eml, pool)?);
    send_invitation(&invitation)
}

/// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
