use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpMessage as _, HttpRequest, HttpResponse, HttpServer,
};

async fn index(id: Option<Identity>) -> String {
    if let Some(id) = id {
        format!(
            "Hello {}",
            id.id().unwrap_or_else(|_| "Identity Id Error.".to_owned())
        )
    } else {
        "Hello Anonymous!".to_owned()
    }
}
async fn login(req: HttpRequest) -> HttpResponse {
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    HttpResponse::Found()
        .insert_header(("location", "/"))
        .finish()
}

async fn logout(id: Identity) -> HttpResponse {
    id.logout();

    HttpResponse::Found()
        .insert_header(("location", "/"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Generate a random secret key. Note that it is important to use a unique
    // secret key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let secret_key = Key::generate();
    //
    // If Key is read from the file, see commented line below.
    // for random string below use: openssl rand -base64 64
    // const PRIVATE_KEY: &str = "yD508YGdEzkgK48ysgdNX65FbOmf2I5z3jINtpOCSJ8qSP0fNz9uoonCwu9gplh0
    // YpAFfvYLRLtBUtECZmRJqQ==";
    const JUST_A_MINUTE: i64 = 60;

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    // Key::from(PRIVATE_KEY.as_bytes())
                    secret_key.clone(),
                )
                .cookie_name("auth-example".to_owned())
                .cookie_secure(false)
                .session_lifecycle(
                    PersistentSession::default().session_ttl(Duration::seconds(JUST_A_MINUTE)),
                )
                .build(),
            )
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
