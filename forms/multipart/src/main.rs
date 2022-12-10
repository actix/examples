use std::{io::Write,collections::HashMap};

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use uuid::Uuid;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // define string mime
    let text_mime = vec![
        String::from("application/octet-stream"),
        String::from("text/plain")
    ];
    // empty hashmap to catch captured text input
    let mut text_payload = HashMap::new();
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // text field
        if text_mime.contains(&field.content_type().to_string()) {{
            // A multipart/form-data stream has to contain `content_disposition`
            let content_disposition = field.content_disposition();
            // default empty text
            let mut text_key:String = String::new();
            let mut text_value:String = String::new();
            // get input name
            match content_disposition.get_name() {
                Some(e)=>{
                    text_key = String::from(e);
                },
                None=>{}
            };
            // get input value
            while let Some(chunk) = field.try_next().await? {
                match String::from_utf8(chunk.to_vec()) {
                    Ok(e)=>{
                        text_value = String::from(e);
                    },
                    Err(_)=>{}
                }
            };
            // insert into hashmap
            text_payload.insert(text_key, text_value);
        }}else{{
            // file field. (double bracket here and above is necessary)
            // A multipart/form-data stream has to contain `content_disposition`
            let content_disposition = field.content_disposition();
    
            let filename = content_disposition
                .get_filename()
                .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
            let filepath = format!("./tmp/{filename}");
    
            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;
    
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
            }
        }}
    }
    Ok(HttpResponse::Ok().into())
}

async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="text" name="text"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
