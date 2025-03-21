use actix_web::{App, HttpResponse, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // a dummy data_factory implementation
            .data_factory(|| {
                // App::data_factory would accept a future as return type and poll the future when
                // App is initialized.
                //
                // The Output of the future must be Result<T, E> and T will be the transformed to
                // App::Data<T> that can be extracted from handler/request. (The E will only be used
                // to trigger an error log.)
                //
                // This data is bound to worker thread and you get an instance of it for each worker
                // of the HttpServer, hence the name data_factory.
                //
                // It is NOT shared between workers
                // (unless the underlying data is a smart pointer like Arc<T>).

                async {
                    // would be transformed into Data<usize>
                    Ok::<_, ()>(123_usize)
                }
            })
            .route(
                "/",
                web::to(|data: web::Data<usize>| async move {
                    assert_eq!(**data, 123);
                    HttpResponse::NoContent().finish()
                }),
            )
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
