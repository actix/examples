use actix::{Handler, Message};
use chrono::Local;
use diesel::prelude::*;
use errors::ServiceError;
use models::{DbExecutor, Invitation, User, SlimUser};
use uuid::Uuid;
use utils::hash_password;

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

// to be used to send data via the Actix actor system
#[derive(Debug)]
pub struct RegisterUser {
    pub invitation_id: String,
    pub password: String,
}


impl Message for RegisterUser {
    type Result = Result<SlimUser, ServiceError>;
}


impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use schema::invitations::dsl::{invitations, id};
        use schema::users::dsl::users;
        let conn: &PgConnection = &self.0.get().unwrap();

        // try parsing the string provided by the user as url parameter
        // return early with error that will be converted to ServiceError
        let invitation_id = Uuid::parse_str(&msg.invitation_id)?;

        invitations.filter(id.eq(invitation_id))
            .load::<Invitation>(conn)
            .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
            .and_then(|mut result| {
                if let Some(invitation) = result.pop() {
                    // if invitation is not expired
                    if invitation.expires_at > Local::now().naive_local() {
                        // try hashing the password, else return the error that will be converted to ServiceError
                        let password: String = hash_password(&msg.password)?;
                        let user = User::with_details(invitation.email, password);
                        let inserted_user: User = diesel::insert_into(users)
                            .values(&user)
                            .get_result(conn)?;

                        return Ok(inserted_user.into());
                    }
                }
                Err(ServiceError::BadRequest("Invalid Invitation".into()))
            })
    }
}


