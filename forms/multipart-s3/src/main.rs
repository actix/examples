use std::fs;

use actix_multipart::Multipart;
use actix_web::{middleware::Logger, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use aws_config::meta::region::RegionProviderChain;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

mod utils;

use self::utils::{
    s3::Client,
    upload::{save_file as upload_save_file, split_payload, UploadFile},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct InpAdd {
    pub text: String,
    pub number: i32,
}

async fn save_file(
    s3_client: web::Data<Client>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let pl = split_payload(&mut payload).await;
    println!("bytes={:#?}", pl.0);

    let inp_info: InpAdd = serde_json::from_slice(&pl.0).unwrap();
    println!("converter_struct={:#?}", inp_info);
    println!("tmpfiles={:#?}", pl.1);

    // make key
    let s3_upload_key = format!("projects/{}/", "posts_id");

    // create tmp file and upload s3 and remove tmp file
    let upload_files: Vec<UploadFile> = upload_save_file(&s3_client, pl.1, &s3_upload_key)
        .await
        .unwrap();
    println!("upload_files={:#?}", upload_files);

    Ok(HttpResponse::Ok().into())
}

async fn index() -> impl Responder {
    Html(include_str!("./index.html").to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");

    fs::create_dir_all("./tmp").unwrap();

    log::info!("configuring S3 client");
    let aws_region = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::from_env().region(aws_region).load().await;
    let s3_client = Client::new(&aws_config);

    log::info!("using AWS region: {}", aws_config.region().unwrap());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(s3_client.clone()))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
