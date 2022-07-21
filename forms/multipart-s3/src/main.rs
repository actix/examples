use std::{borrow::BorrowMut, env};

use actix_multipart::Multipart;
use actix_web::{middleware::Logger, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

mod utils;
use self::utils::upload::{save_file as upload_save_file, split_payload, UploadFile};

#[derive(Deserialize, Serialize, Debug)]
pub struct InpAdd {
    pub text: String,
    pub number: i32,
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let pl = split_payload(payload.borrow_mut()).await;
    println!("bytes={:#?}", pl.0);
    let inp_info: InpAdd = serde_json::from_slice(&pl.0).unwrap();
    println!("converter_struct={:#?}", inp_info);
    println!("tmpfiles={:#?}", pl.1);
    //make key
    let s3_upload_key = format!("projects/{}/", "posts_id");
    //create tmp file and upload s3 and remove tmp file
    let upload_files: Vec<UploadFile> = upload_save_file(pl.1, s3_upload_key).await.unwrap();
    println!("upload_files={:#?}", upload_files);
    Ok(HttpResponse::Ok().into())
}

async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data" id="myForm" >
                <input type="text"  id="text" name="text" value="test_text"/>
                <input type="number"  id="number" name="number" value="123123"/>
                <input type="button" value="Submit" onclick="myFunction()"></button>
            </form>
            <input type="file" multiple name="file" id="myFile"/>
        </body>
        <script>

        function myFunction(){
            var myForm = document.getElementById('myForm');
            var myFile = document.getElementById('myFile');

            let formData = new FormData();
            const obj = {
                text: document.getElementById('text').value,
                number: Number(document.getElementById('number').value)
            };
            const json = JSON.stringify(obj);
            console.log(obj);
            console.log(json);

            formData.append("data", json);
            formData.append("myFile", myFile.files[0]);

            var request = new XMLHttpRequest();
            request.open("POST", "");
            request.send(formData);
        }

        </script>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let aws_secret_access_key =
        env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    let aws_s3_bucket_name =
        env::var("AWS_S3_BUCKET_NAME").expect("AWS_S3_BUCKET_NAME must be set");

    log::info!("aws_access_key_id:     {aws_access_key_id}");
    log::info!("aws_secret_access_key: {aws_secret_access_key}");
    log::info!("aws_s3_bucket_name:    {aws_s3_bucket_name}");

    std::fs::create_dir_all("./tmp").unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
