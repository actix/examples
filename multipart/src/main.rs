extern crate actix_web;
extern crate actix_multipart;
extern crate serde;

#[macro_use(concat_string)]
extern crate concat_string;

use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt};
use std::cell::Cell;
use serde::Deserialize;

pub struct AppState {
    pub counter: Cell<usize>,
}

#[derive(Deserialize, Debug)]
struct Config {
  port: String
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        let mut f = std::fs::File::create(filepath).unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let mut pos = 0;
            while pos < data.len() {
                let bytes_written = f.write(&data[pos..])?;
                pos += bytes_written;
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();
    let config = envy::from_env::<Config>().unwrap();
    let p =  config.port;
    let port = concat_string!("0.0.0.0:", p);
    HttpServer::new(|| {
    App::new()
    .data(Cell::new(0usize))
    .wrap(middleware::Logger::default())
    .service(
        web::resource("/")
            .route(web::get().to(index))
            .route(web::post().to(save_file)),
        )
    })
    .bind(port)?
    .run()
}
