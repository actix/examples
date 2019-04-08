use std::cell::Cell;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use actix_multipart::{Field, Item, Multipart, MultipartError};
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::{err, ok, Either};
use futures::{Future, Stream};

pub struct AppState {
    pub counter: Cell<usize>,
}

pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = "upload.png";
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold(0i64, move |acc, bytes| {
                println!("CHUNK: {:?}", bytes.len());
                file.write_all(bytes.as_ref())
                    .map(|_| acc + bytes.len() as i64)
                    .map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn handle_multipart_item(item: Item) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        Item::Field(field) => Box::new(save_file(field).into_stream()),
        Item::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn upload(
    multipart: Multipart,
    counter: web::Data<Cell<usize>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .from_err::<Error>()
        .take(1)
        .collect()
        .map(|v| v.into_iter().next().expect("wat"))
        .and_then(|item| match item {
            Item::Field(field) => {
                if let Some(disp) = field.content_disposition() {
                    if let Some(disp_fn) = disp.get_filename() {
                        if let Some(ext) = Path::new(&disp_fn).extension() {
                            let fname = format!("{}.{}", 10, ext.to_string_lossy());
                            let pth = Path::new("./").join(&fname);
                            if let Ok(mut ff) = File::create(&pth) {
                                return Either::A(
                                    field
                                        .from_err::<Error>()
                                        .map(move |c| ff.write_all(&c))
                                        .fold((), |_, _| Ok::<_, Error>(()))
                                        //.finish()
                                        .and_then(move |_| {
                                            ok(HttpResponse::Created().body(format!(
                                                "{{\"path\": \"{}\"}}",
                                                fname
                                            )))
                                        })
                                        .or_else(|_| {
                                            ok(HttpResponse::InternalServerError()
                                                .finish())
                                        }),
                                );
                            }
                        }
                    }
                }
                Either::B(ok(HttpResponse::BadRequest().finish()))
            }
            Item::Nested(_) => Either::B(ok(HttpResponse::BadRequest().finish())),
        })
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(Cell::new(0usize))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to_async(upload)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
