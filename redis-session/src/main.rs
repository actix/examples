//! Example of redis based session
//!
//! [User guide](https://actix.rs/book/actix-web/sec-9-middlewares.html#user-sessions)
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};

/// simple handler
fn index(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok("Welcome!".into())
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // redis session middleware
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
