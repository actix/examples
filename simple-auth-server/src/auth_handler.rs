use actix::{Handler, Message};
use diesel::prelude::*;
use errors::ServiceError;
use models::{DbExecutor, User, SlimUser};
use bcrypt::verify;
use actix_web::{FromRequest, HttpRequest, middleware::identity::RequestIdentity};
use utils::decode_token;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl Message for AuthData {
    type Result = Result<SlimUser, ServiceError>;
}


impl Handler<AuthData> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;
    fn handle(&mut self, msg: AuthData, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::{users, email};
        let conn: &PgConnection = &self.0.get().unwrap();
        let mismatch_error = Err(ServiceError::BadRequest("Username and Password don't match".into()));

        let mut items = users
            .filter(email.eq(&msg.email))
            .load::<User>(conn)?;

        if let Some(user) = items.pop() {
            match verify(&msg.password, &user.password) {
                Ok(matching) => {
                    if matching { return Ok(user.into()); } else { return mismatch_error; }
                }
                Err(_) => { return mismatch_error; }
            }
        }
        mismatch_error
    }
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl<S> FromRequest<S> for LoggedUser {
    type Config = ();
    type Result = Result<LoggedUser, ServiceError>;
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        if let Some(identity) = req.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }
        Err(ServiceError::Unauthorized)
    }
}
