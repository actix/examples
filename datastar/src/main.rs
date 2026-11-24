use std::time::Duration;

use actix_sse::Sse;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, error, get, middleware::Logger, post, web,
};
use datastar::prelude::PatchElements;
use indoc::formatdoc;
use tokio::sync::watch;
use tokio_stream::{StreamExt, wrappers::WatchStream};

const OPTIONS: [(&str, &str); 4] = [
    ("tea", "Tea"),
    ("coffee", "Coffee"),
    ("matcha", "Matcha"),
    ("hot-chocolate", "Hot Chocolate"),
];

#[derive(Default)]
struct Poll {
    votes: watch::Sender<[u64; OPTIONS.len()]>,
}

impl Poll {
    fn results(counts: [u64; OPTIONS.len()]) -> String {
        let total = counts.iter().copied().map(u128::from).sum::<u128>().max(1);
        let highest = counts.iter().copied().max().unwrap_or_default();

        let rows = OPTIONS
            .into_iter()
            .zip(counts)
            .map(|((choice, name), count)| {
                let share = 100 * u128::from(count) / total;
                let track = if count > 0 && count == highest { "track leader" } else { "track" };

                formatdoc!(
                    r#"
                    <div class="option">
                        <div>
                            <div class="tally">{name} <strong>{count}</strong></div>
                            <div class="{track}"><span style="width: {share}%"></span></div>
                        </div>
                        <button id="vote-{choice}" data-on:click="@post('/vote/{choice}')">Vote</button>
                    </div>
                    "#
                )
            })
            .collect::<String>();

        format!(r#"<div id="results">{rows}</div>"#)
    }
}

#[get("/style.css")]
async fn stylesheet() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("style.css"))
}

#[get("/actix.png")]
async fn logo() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("image/png")
        .body(&include_bytes!("actix.png")[..])
}

#[get("/")]
async fn index(poll: web::Data<Poll>) -> web::Html {
    web::Html::new(
        include_str!("index.html")
            .replace("<!-- results -->", &Poll::results(*poll.votes.borrow())),
    )
}

#[get("/results")]
async fn results(poll: web::Data<Poll>) -> impl Responder {
    let updates = WatchStream::new(poll.votes.subscribe())
        .map(|counts| PatchElements::new(Poll::results(counts)).into());

    Sse::from_infallible_stream(updates).with_keep_alive(Duration::from_secs(15))
}

#[post("/vote/{choice}")]
async fn vote(
    choice: web::Path<String>,
    poll: web::Data<Poll>,
) -> actix_web::Result<impl Responder> {
    let option_index = OPTIONS
        .iter()
        .position(|(slug, _)| *slug == choice.as_str())
        .ok_or_else(|| error::ErrorBadRequest("Unknown poll option."))?;

    poll.votes.send_modify(|counts| counts[option_index] += 1);
    Ok(HttpResponse::NoContent().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    examples_common::init_standard_logger();

    let poll = web::Data::new(Poll::default());

    tracing::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(poll.clone())
            .service(stylesheet)
            .service(logo)
            .service(index)
            .service(results)
            .service(vote)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use std::{future::poll_fn, pin::Pin, time::Duration};

    use actix_web::{
        body::{BoxBody, MessageBody},
        http::StatusCode,
        rt::time::timeout,
        test,
    };

    use super::*;

    async fn next_patch(body: &mut BoxBody) -> String {
        let chunk = timeout(
            Duration::from_secs(2),
            poll_fn(|cx| Pin::new(&mut *body).poll_next(cx)),
        )
        .await
        .expect("timed out waiting for patch")
        .expect("results stream closed")
        .unwrap();
        String::from_utf8(chunk.to_vec()).unwrap()
    }

    #[tokio::test(flavor = "local")]
    async fn votes_update_the_shared_results() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Poll::default()))
                .service(stylesheet)
                .service(logo)
                .service(index)
                .service(results)
                .service(vote),
        )
        .await;

        let page =
            test::call_and_read_body(&app, test::TestRequest::get().uri("/").to_request()).await;
        assert!(
            std::str::from_utf8(&page)
                .unwrap()
                .contains("Tea <strong>0</strong>")
        );

        let invalid = test::TestRequest::post().uri("/vote/milk").to_request();
        assert_eq!(
            test::call_service(&app, invalid).await.status(),
            StatusCode::BAD_REQUEST
        );

        let mut clients = Vec::new();
        for _ in 0..2 {
            let req = test::TestRequest::get().uri("/results").to_request();
            let response = test::call_service(&app, req).await;
            assert_eq!(
                response.headers().get("content-type").unwrap(),
                "text/event-stream"
            );
            let mut body = response.into_body();
            assert!(
                next_patch(&mut body)
                    .await
                    .contains("Tea <strong>0</strong>")
            );
            clients.push(body);
        }

        let mut patch = String::new();
        for choice in ["coffee", "tea", "coffee", "matcha", "hot-chocolate"] {
            let req = test::TestRequest::post()
                .uri(&format!("/vote/{choice}"))
                .to_request();
            assert!(test::call_service(&app, req).await.status().is_success());

            patch = next_patch(&mut clients[0]).await;
            assert!(patch.contains("event: datastar-patch-elements"));
            assert_eq!(patch, next_patch(&mut clients[1]).await);
        }
        assert!(patch.contains("Tea <strong>1</strong>"));
        assert!(patch.contains("Coffee <strong>2</strong>"));
        assert!(patch.contains("Matcha <strong>1</strong>"));
        assert!(patch.contains("Hot Chocolate <strong>1</strong>"));
        assert_eq!(patch.matches("width: 20%").count(), 3);
        assert_eq!(patch.matches("width: 40%").count(), 1);
        assert_eq!(patch.matches("track leader").count(), 1);
        assert!(patch.contains(r#"class="track leader"><span style="width: 40%""#));
    }
}
