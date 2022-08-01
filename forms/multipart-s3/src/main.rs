use std::fs;

use actix_multipart::Multipart;
use actix_web::body::SizedStream;
use actix_web::{delete, error};
use actix_web::{
    get, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::extract::Path;
use actix_web_lab::respond::Html;
use aws_config::meta::region::RegionProviderChain;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod client;
mod temp_file;
mod upload_file;
mod utils;

use self::client::Client;
use self::temp_file::TempFile;
use self::upload_file::UploadedFile;
use self::utils::split_payload;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadMeta {
    namespace: String,
}

impl Default for UploadMeta {
    fn default() -> Self {
        Self {
            namespace: "default".to_owned(),
        }
    }
}

#[post("/")]
async fn upload_to_s3(
    s3_client: web::Data<Client>,
    mut payload: Multipart,
) -> Result<impl Responder, Error> {
    let (data, files) = split_payload(&mut payload).await;
    log::info!("bytes = {data:?}");

    let upload_meta = serde_json::from_slice::<UploadMeta>(&data).unwrap_or_default();
    log::info!("converter_struct = {upload_meta:?}");
    log::info!("tmp_files = {files:?}");

    // make key prefix (make sure it ends with a forward slash)
    let s3_key_prefix = format!("uploads/{}/", upload_meta.namespace);

    // create tmp file and upload s3 and remove tmp file
    let uploaded_files = s3_client.upload_files(files, &s3_key_prefix).await?;

    Ok(HttpResponse::Ok().json(json!({
        "uploadedFiles": uploaded_files,
        "meta": upload_meta,
    })))
}

#[get("/file/{s3_key}*")]
async fn fetch_from_s3(
    s3_client: web::Data<Client>,
    Path((s3_key,)): Path<(String,)>,
) -> Result<impl Responder, Error> {
    let (file_size, file_stream) = s3_client
        .fetch_file(&s3_key)
        .await
        .ok_or_else(|| error::ErrorNotFound("file with specified key not found"))?;

    Ok(HttpResponse::Ok().body(SizedStream::new(file_size, file_stream)))
}

#[delete("/file/{s3_key}*")]
async fn delete_from_s3(
    s3_client: web::Data<Client>,
    Path((s3_key,)): Path<(String,)>,
) -> Result<impl Responder, Error> {
    if s3_client.delete_file(&s3_key).await {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(error::ErrorNotFound("file with specified key not found"))
    }
}

#[get("/")]
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

    // create singleton S3 client
    let s3_client = Client::new(&aws_config);

    log::info!("using AWS region: {}", aws_config.region().unwrap());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(upload_to_s3)
            .service(fetch_from_s3)
            .service(delete_from_s3)
            .wrap(Logger::default())
            .app_data(web::Data::new(s3_client.clone()))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
