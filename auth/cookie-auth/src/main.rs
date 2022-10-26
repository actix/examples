use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key, middleware, web, App, HttpMessage as _, HttpRequest, HttpResponse, HttpServer,
};
use rand::Rng;

async fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.id().unwrap_or_else(|_| "Anonymous".to_owned())
    )
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

    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&private_key))
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .build(),
            )
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
