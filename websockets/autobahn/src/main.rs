use std::io;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_ws::AggregatedMessage;
use examples_common::init_standard_logger;
use tokio::task::spawn_local;

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, msg_stream) = actix_ws::handle(&r, stream)?;

    spawn_local(async move {
        let mut msg_stream = msg_stream
            .max_frame_size(16 * 1024 * 1024)
            .aggregate_continuations()
            .max_continuation_size(4 * 1024 * 1024);

        let close_reason = loop {
            match msg_stream.recv().await {
                Some(Ok(msg)) => match msg {
                    AggregatedMessage::Text(text) => {
                        if session.text(text).await.is_err() {
                            break None;
                        }
                    }
                    AggregatedMessage::Binary(bin) => {
                        if session.binary(bin).await.is_err() {
                            break None;
                        }
                    }
                    AggregatedMessage::Ping(bytes) => {
                        if session.pong(&bytes).await.is_err() {
                            break None;
                        }
                    }
                    AggregatedMessage::Pong(_) => {}
                    AggregatedMessage::Close(reason) => break reason,
                },
                Some(Err(err)) => {
                    tracing::error!("WebSocket protocol error: {err}");
                    break None;
                }
                None => break None,
            }
        };

        let _ = session.close(close_reason).await;
    });

    Ok(res)
}

#[tokio::main(flavor = "local")]
async fn main() -> io::Result<()> {
    init_standard_logger();

    tracing::info!("starting HTTP server at http://localhost:9001");

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
