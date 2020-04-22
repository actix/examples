use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use std::borrow::BorrowMut;
use utils::payload_handler::{split_payload};

mod utils;


async fn handle_form_data(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let (form, files) = split_payload(payload.borrow_mut()).await;

    println!("bytes={:#?}", form);
    
    println!("files={:#?}", files);
    
    Ok(HttpResponse::Ok().into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title>
            <style>
                *{
                    margin: 0;
                }
                body{
                    display: flex;
                }
                form{
                    margin: auto;
                    box-sizing: border-box;
                    width: 20rem;
                    max-width: 100%;
                    display: flex;
                    flex-direction: column;
                    background-color: lightgray;
                    padding: 2rem 1rem;
                }
                input{
                    margin: 0.8rem 0;
                }
            </style>
        </head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data" id="myForm" >
                <input type="text" name="title" placeholder="give it a name"/>
                <input type="text" name="description" placeholder="describe it"/>        
                <input type="number" placeholder="how many" name="count" value=""/>    
                <input type="file" multiple name="file"/>

                <input type="submit" value="Submit"></button>
            </form>
        </body>
        <script>        
        </script>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./files").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(handle_form_data)),
        )
    })
    .bind(ip)?
    .run()
    .await
}
