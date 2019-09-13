use actix_web::error::Error;
use actix_web::{web, App, HttpServer};

#[allow(dead_code)]
mod hack_my_web;

fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(hack_my_web::HackMyWeb) // Append my custom middleware
            .service(
                web::resource("/f18b211dd1744570bb643e800308b1e4")
                    .to(|| "Oh...!!! Here is secret page!!! Cheat me?"),
            )
            .service(
                web::resource("/{name}")
                    .to(|name: web::Path<String>| format!("Hello {}!", name)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()?;
    Ok(())
}
