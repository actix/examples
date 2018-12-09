use actix_web::{AsyncResponder, FutureResponse, HttpResponse, HttpRequest, ResponseError, Json};
use actix_web::middleware::identity::RequestIdentity;
use futures::future::Future;
use utils::create_token;

use app::AppState;
use auth_handler::{AuthData, LoggedUser};

pub fn login((auth_data, req): (Json<AuthData>, HttpRequest<AppState>))
             -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(auth_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let token = create_token(&user)?;
                req.remember(token);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response()),
        }).responder()
}

pub fn logout(req: HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Ok().into()
}

pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
