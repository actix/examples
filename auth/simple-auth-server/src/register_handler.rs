use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    models::{Invitation, Pool, SlimUser, User},
    utils::hash_password,
};

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || {
        query(
            invitation_id.into_inner(),
            user_data.into_inner().password,
            pool,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(&user))
}

fn query(
    invitation_id: String,
    password: String,
    pool: web::Data<Pool>,
) -> Result<SlimUser, crate::errors::ServiceError> {
    use crate::schema::{invitations::dsl::*, users::dsl::*};

    let mut conn = pool.get().unwrap();

    let invitation_id = invitation_id.parse::<Uuid>()?;

    invitations
        .filter(id.eq(invitation_id))
        .load::<Invitation>(&mut conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // if invitation is not expired
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // try hashing the password, else return the error that will be converted to ServiceError
                    let password: String = hash_password(&password)?;
                    dbg!(&password);

                    let user = User::from_details(invitation.email, password);
                    let inserted_user: User = diesel::insert_into(users)
                        .values(&user)
                        .get_result(&mut conn)?;
                    dbg!(&inserted_user);

                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
