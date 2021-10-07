//! Example of login and logout using redis-based sessions
//!
//! Every request gets a session, corresponding to a cache entry and cookie.
//! At login, the session key changes and session state in cache re-assigns.
//! At logout, session state in cache is removed and cookie is invalidated.
//!
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{
    middleware, web,
    web::{get, post, resource},
    App, HttpResponse, HttpServer, Result,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}

async fn index(session: Session) -> Result<HttpResponse> {
    let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
}

async fn do_something(session: Session) -> Result<HttpResponse> {
    let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .map_or(1, |inner| inner + 1);
    session.set("counter", counter)?;

    Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
}

#[derive(Deserialize)]
struct Identity {
    user_id: String,
}

async fn login(user_id: web::Json<Identity>, session: Session) -> Result<HttpResponse> {
    let id = user_id.into_inner().user_id;
    session.set("user_id", &id)?;
    session.renew();

    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    Ok(HttpResponse::Ok().json(IndexResponse {
        user_id: Some(id),
        counter,
    }))
}

async fn logout(session: Session) -> Result<HttpResponse> {
    let id: Option<String> = session.get("user_id")?;
    if let Some(x) = id {
        session.purge();
        Ok(format!("Logged out: {}", x).into())
    } else {
        Ok("Could not log out anonymous user".into())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            // redis session middleware
            .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(resource("/").route(get().to(index)))
            .service(resource("/do_something").route(post().to(do_something)))
            .service(resource("/login").route(post().to(login)))
            .service(resource("/logout").route(post().to(logout)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_http::httpmessage::HttpMessage;
    use actix_web::{
        middleware, test,
        web::{get, post, resource},
        App,
    };
    use serde_json::json;

    #[actix_rt::test]
    async fn test_workflow() {
        // Step 1:  GET index
        //   - set-cookie actix-session will be in response (session cookie #1)
        //   - response should be: {"counter": 0, "user_id": None}
        // Step 2:  GET index, including session cookie #1 in request
        //   - set-cookie will *not* be in response
        //   - response should be: {"counter": 0, "user_id": None}
        // Step 3: POST to do_something, including session cookie #1 in request
        //   - adds new session state in redis:  {"counter": 1}
        //   - response should be: {"counter": 1, "user_id": None}
        // Step 4: POST again to do_something, including session cookie #1 in request
        //   - updates session state in redis:  {"counter": 2}
        //   - response should be: {"counter": 2, "user_id": None}
        // Step 5: POST to login, including session cookie #1 in request
        //   - set-cookie actix-session will be in response  (session cookie #2)
        //   - updates session state in redis: {"counter": 2, "user_id": "ferris"}
        // Step 6: GET index, including session cookie #2 in request
        //   - response should be: {"counter": 2, "user_id": "ferris"}
        // Step 7: POST again to do_something, including session cookie #2 in request
        //   - updates session state in redis: {"counter": 3, "user_id": "ferris"}
        //   - response should be: {"counter": 2, "user_id": None}
        // Step 8: GET index, including session cookie #1 in request
        //   - set-cookie actix-session will be in response (session cookie #3)
        //   - response should be: {"counter": 0, "user_id": None}
        // Step 9: POST to logout, including session cookie #2
        //   - set-cookie actix-session will be in response with session cookie #2
        //     invalidation logic
        // Step 10: GET index, including session cookie #2 in request
        //   - set-cookie actix-session will be in response (session cookie #3)
        //   - response should be: {"counter": 0, "user_id": None}

        let private_key = rand::thread_rng().gen::<[u8; 32]>();
        let srv = test::start(move || {
            App::new()
                .wrap(
                    RedisSession::new("127.0.0.1:6379", &private_key)
                        .cookie_name("test-session"),
                )
                .wrap(middleware::Logger::default())
                .service(resource("/").route(get().to(index)))
                .service(resource("/do_something").route(post().to(do_something)))
                .service(resource("/login").route(post().to(login)))
                .service(resource("/logout").route(post().to(logout)))
        });

        // Step 1:  GET index
        //   - set-cookie actix-session will be in response (session cookie #1)
        //   - response should be: {"counter": 0, "user_id": None}
        let req_1a = srv.get("/").send();
        let mut resp_1 = req_1a.await.unwrap();
        let cookie_1 = resp_1
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session")
            .unwrap();
        let result_1 = resp_1.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_1,
            IndexResponse {
                user_id: None,
                counter: 0
            }
        );

        // Step 2:  GET index, including session cookie #1 in request
        //   - set-cookie will *not* be in response
        //   - response should be: {"counter": 0, "user_id": None}
        let req_2 = srv.get("/").cookie(cookie_1.clone()).send();
        let resp_2 = req_2.await.unwrap();
        let cookie_2 = resp_2
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session");
        assert_eq!(cookie_2, None);

        // Step 3: POST to do_something, including session cookie #1 in request
        //   - adds new session state in redis:  {"counter": 1}
        //   - response should be: {"counter": 1, "user_id": None}
        let req_3 = srv.post("/do_something").cookie(cookie_1.clone()).send();
        let mut resp_3 = req_3.await.unwrap();
        let result_3 = resp_3.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_3,
            IndexResponse {
                user_id: None,
                counter: 1
            }
        );

        // Step 4: POST again to do_something, including session cookie #1 in request
        //   - updates session state in redis:  {"counter": 2}
        //   - response should be: {"counter": 2, "user_id": None}
        let req_4 = srv.post("/do_something").cookie(cookie_1.clone()).send();
        let mut resp_4 = req_4.await.unwrap();
        let result_4 = resp_4.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_4,
            IndexResponse {
                user_id: None,
                counter: 2
            }
        );

        // Step 5: POST to login, including session cookie #1 in request
        //   - set-cookie actix-session will be in response  (session cookie #2)
        //   - updates session state in redis: {"counter": 2, "user_id": "ferris"}
        let req_5 = srv
            .post("/login")
            .cookie(cookie_1.clone())
            .send_json(&json!({"user_id": "ferris"}));
        let mut resp_5 = req_5.await.unwrap();
        let cookie_2 = resp_5
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session")
            .unwrap();
        assert_ne!(cookie_1.value(), cookie_2.value());

        let result_5 = resp_5.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_5,
            IndexResponse {
                user_id: Some("ferris".into()),
                counter: 2
            }
        );

        // Step 6: GET index, including session cookie #2 in request
        //   - response should be: {"counter": 2, "user_id": "ferris"}
        let req_6 = srv.get("/").cookie(cookie_2.clone()).send();
        let mut resp_6 = req_6.await.unwrap();
        let result_6 = resp_6.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_6,
            IndexResponse {
                user_id: Some("ferris".into()),
                counter: 2
            }
        );

        // Step 7: POST again to do_something, including session cookie #2 in request
        //   - updates session state in redis: {"counter": 3, "user_id": "ferris"}
        //   - response should be: {"counter": 2, "user_id": None}
        let req_7 = srv.post("/do_something").cookie(cookie_2.clone()).send();
        let mut resp_7 = req_7.await.unwrap();
        let result_7 = resp_7.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_7,
            IndexResponse {
                user_id: Some("ferris".into()),
                counter: 3
            }
        );

        // Step 8: GET index, including session cookie #1 in request
        //   - set-cookie actix-session will be in response (session cookie #3)
        //   - response should be: {"counter": 0, "user_id": None}
        let req_8 = srv.get("/").cookie(cookie_1.clone()).send();
        let mut resp_8 = req_8.await.unwrap();
        let cookie_3 = resp_8
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session")
            .unwrap();
        let result_8 = resp_8.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_8,
            IndexResponse {
                user_id: None,
                counter: 0
            }
        );
        assert_ne!(cookie_3.value(), cookie_2.value());

        // Step 9: POST to logout, including session cookie #2
        //   - set-cookie actix-session will be in response with session cookie #2
        //     invalidation logic
        let req_9 = srv.post("/logout").cookie(cookie_2.clone()).send();
        let resp_9 = req_9.await.unwrap();
        let cookie_4 = resp_9
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session")
            .unwrap();

        let now = time::OffsetDateTime::now_utc();
        assert!(now.year() != cookie_4.expires().map(|t| t.year()).unwrap());

        // Step 10: GET index, including session cookie #2 in request
        //   - set-cookie actix-session will be in response (session cookie #3)
        //   - response should be: {"counter": 0, "user_id": None}
        let req_10 = srv.get("/").cookie(cookie_2.clone()).send();
        let mut resp_10 = req_10.await.unwrap();
        let result_10 = resp_10.json::<IndexResponse>().await.unwrap();
        assert_eq!(
            result_10,
            IndexResponse {
                user_id: None,
                counter: 0
            }
        );

        let cookie_5 = resp_10
            .cookies()
            .unwrap()
            .clone()
            .into_iter()
            .find(|c| c.name() == "test-session")
            .unwrap();
        assert_ne!(cookie_5.value(), cookie_2.value());
    }
}
