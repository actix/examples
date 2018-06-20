#![allow(unused_variables)]
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use std::fs;
use std::io::Write;
use std::cell::Cell;

use actix_web::{
    error, http, middleware, multipart, server, App, Error, FutureResponse, HttpMessage,
    HttpRequest, HttpResponse,
};

use futures::future;
use futures::{Future, Stream};

pub struct AppState {
    pub counter: Cell<usize>,
}

pub fn save_file( field: multipart::Field<HttpRequest<AppState>>) -> Box<Future<Item = i64, Error = Error>> {
    let file_path_string = "upload.png";
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };
    Box::new(
        field
            .fold(0i64, move |acc, bytes| {
                let rt = file
                    .write_all(bytes.as_ref())
                    .map(|_| acc + bytes.len() as i64)
                    .map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        error::MultipartError::Payload(error::PayloadError::Io(e))
                    });
                future::result(rt)
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn handle_multipart_item( item: multipart::MultipartItem<HttpRequest<AppState>>) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(save_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn upload(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().counter.set(req.state().counter.get() + 1);
    println!("{:?}",  req.state().counter.get());
    Box::new(
        req.clone()
            .multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
}

fn index(_req: HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    Ok(HttpResponse::Ok().body(html))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("multipart-example");

    server::new(|| {
        App::with_state(AppState{counter: Cell::new(0)})
            .middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).with(index);
                r.method(http::Method::POST).with(upload);
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
