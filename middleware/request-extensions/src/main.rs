use std::io;

use actix_web::{
    App, HttpResponse, HttpServer, middleware,
    web::{self, ReqData},
};

mod add_msg;
use self::add_msg::{AddMsg, Msg};

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

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
