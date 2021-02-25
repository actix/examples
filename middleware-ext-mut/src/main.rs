use actix_web::{get, web, App, HttpResponse, HttpServer};
use std::{env, io};

mod send_data;

// wrap route in send_data::SendDataService middleware
#[get("/", wrap = "send_data::SendDataFactory")]
async fn index(msg: Option<web::ReqData<send_data::Msg>>) -> HttpResponse {
    if let Some(m) = msg {
        let send_data::Msg(message) = m.into_inner();
        message.into()
    } else {
        "No message found.".into()
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
