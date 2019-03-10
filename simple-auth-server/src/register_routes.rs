use actix_web::{
    AsyncResponder, FutureResponse, HttpResponse, Json, Path, ResponseError, State,
};
use futures::future::Future;

use app::AppState;
use register_handler::{RegisterUser, UserData};

pub fn register_user(
    (invitation_id, user_data, state): (Path<String>, Json<UserData>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    let msg = RegisterUser {
        // into_inner() returns the inner string value from Path
        invitation_id: invitation_id.into_inner(),
        password: user_data.password.clone(),
    };

    state
        .db
        .send(msg)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(slim_user) => Ok(HttpResponse::Ok().json(slim_user)),
            Err(service_error) => Ok(service_error.error_response()),
        })
        .responder()
}
