use std::{fs, io};

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder, body::SizedStream, delete, error, get,
    http::Method, middleware::Logger, post, route, web,
};
use actix_web_lab::extract::Path;
use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use dotenvy::dotenv;
use futures_util::{StreamExt as _, stream};
use serde_json::json;
use tokio_util::io::ReaderStream;

mod client;
mod upload_file;

use self::{client::Client, upload_file::UploadedFile};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    namespace: Text<String>,

    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/")]
async fn upload_to_s3(
    s3_client: web::Data<Client>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let namespace = form.namespace.into_inner();
    let files = form.files;

    log::info!("namespace = {namespace:?}");
    log::info!("tmp_files = {files:?}");

    // make key prefix (make sure it ends with a forward slash)
    let s3_key_prefix = format!("uploads/{namespace}/");

    // upload temp files to s3 and then remove them
    let uploaded_files = s3_client.upload_files(files, &s3_key_prefix).await?;

    Ok(HttpResponse::Ok().json(json!({
        "uploadedFiles": uploaded_files,
        "meta": json!({ "namespace": namespace }),
    })))
}

#[route("/file/{s3_key}*", method = "GET", method = "HEAD")]
async fn fetch_from_s3(
    s3_client: web::Data<Client>,
    method: Method,
    Path((s3_key,)): Path<(String,)>,
) -> Result<impl Responder, Error> {
    let (file_size, file_stream) = s3_client
        .fetch_file(&s3_key)
        .await
        .ok_or_else(|| error::ErrorNotFound("file with specified key not found"))?;

    let stream = match method {
        // data stream for GET requests
        Method::GET => ReaderStream::new(file_stream.into_async_read()).boxed_local(),

        // empty stream for HEAD requests
        Method::HEAD => stream::empty::<Result<_, io::Error>>().boxed_local(),

        _ => unreachable!(),
    };

    Ok(HttpResponse::Ok()
        .no_chunking(file_size)
        .body(SizedStream::new(file_size, stream)))
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
    web::Html::new(include_str!("./index.html").to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");

    fs::create_dir_all("./tmp").unwrap();

    log::info!("configuring S3 client");
    let aws_region = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_region)
        .load()
        .await;

    // create singleton S3 client
    let s3_client = Client::new(&aws_config);

    log::info!("using AWS region: {}", aws_config.region().unwrap());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(s3_client.clone()))
            .service(index)
            .service(upload_to_s3)
            .service(fetch_from_s3)
            .service(delete_from_s3)
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
