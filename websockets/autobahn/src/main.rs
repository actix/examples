use actix::prelude::*;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web_actors::ws;

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(AutobahnWebSocket, &r, stream)
}

#[derive(Debug, Clone, Default)]
struct AutobahnWebSocket;

impl Actor for AutobahnWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AutobahnWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(text) => ctx.text(text),
                ws::Message::Binary(bin) => ctx.binary(bin),
                ws::Message::Ping(bytes) => ctx.pong(&bytes),
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => {}
            }
        } else {
            ctx.stop();
        }
    }
}

// the actor-based WebSocket examples REQUIRE `actix_web::main` for actor support
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:9001");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(ws_index)))
    })
    .workers(2)
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
