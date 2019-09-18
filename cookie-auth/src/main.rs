use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

fn login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
                    .secure(false),
            ))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
