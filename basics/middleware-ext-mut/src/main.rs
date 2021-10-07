use std::{env, io};

use actix_web::{
    middleware,
    web::{self, ReqData},
    App, HttpResponse, HttpServer,
};

mod add_msg;
use crate::add_msg::{AddMsg, Msg};

// wrap route in our middleware factory
async fn index(msg: Option<ReqData<Msg>>) -> HttpResponse {
    if let Some(msg_data) = msg {
        let Msg(message) = msg_data.into_inner();
        HttpResponse::Ok().body(message)
    } else {
        HttpResponse::InternalServerError().body("No message found.")
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/on").wrap(AddMsg::enabled()).to(index))
            .service(web::resource("/off").wrap(AddMsg::disabled()).to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
