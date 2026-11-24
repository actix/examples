use std::sync::atomic::{AtomicU64, Ordering};

use actix_sse::Sse;
use actix_web::{App, HttpServer, Responder, error, get, middleware::Logger, post, web};
use datastar::prelude::PatchElements;
use futures_util::stream;
use indoc::formatdoc;

const OPTIONS: [(&str, &str); 4] = [
    ("tea", "Tea"),
    ("coffee", "Coffee"),
    ("matcha", "Matcha"),
    ("hot-chocolate", "Hot Chocolate"),
];

#[derive(Default)]
struct Poll {
    votes: [AtomicU64; OPTIONS.len()],
}

impl Poll {
    fn results(&self) -> String {
        let counts = self
            .votes
            .each_ref()
            .map(|votes| votes.load(Ordering::Relaxed));

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

    fn patch_results(&self) -> impl Responder + use<> {
        let patch = PatchElements::new(self.results());
        Sse::from_infallible_stream(stream::iter([patch.into()]))
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
    web::Html::new(include_str!("index.html").replace("<!-- results -->", &poll.results()))
}

#[get("/results")]
async fn results(poll: web::Data<Poll>) -> impl Responder {
    poll.patch_results()
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

    poll.votes[option_index].fetch_add(1, Ordering::Relaxed);
    Ok(poll.patch_results())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let poll = web::Data::new(Poll::default());

    log::info!("Starting HTTP server at http://localhost:8080");

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
    use actix_web::{http::StatusCode, test};

    use super::*;

    #[actix_web::test]
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

        for choice in ["coffee", "tea", "coffee", "matcha", "hot-chocolate"] {
            let req = test::TestRequest::post()
                .uri(&format!("/vote/{choice}"))
                .to_request();
            assert_eq!(test::call_service(&app, req).await.status(), StatusCode::OK);
        }

        let req = test::TestRequest::get().uri("/results").to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/event-stream"
        );
        let body = test::read_body(response).await;
        assert!(
            std::str::from_utf8(&body)
                .unwrap()
                .contains("event: datastar-patch-elements")
        );
        let patch = std::str::from_utf8(&body).unwrap();
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
