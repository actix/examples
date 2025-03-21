use actix_web::{App, HttpResponse, HttpServer, dev::ServerHandle, get, middleware, post, web};
use actix_web_lab::extract::Path;
use parking_lot::Mutex;

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello world!"
}

#[post("/stop/{graceful}")]
async fn stop(Path(graceful): Path<bool>, stop_handle: web::Data<StopHandle>) -> HttpResponse {
    stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // create the stop handle container
    let stop_handle = web::Data::new(StopHandle::default());

    log::info!("starting HTTP server at http://localhost:8080");

    // start server as normal but don't .await after .run() yet
    let srv = HttpServer::new({
        let stop_handle = stop_handle.clone();

        move || {
            // give the server a Sender in .data
            App::new()
                .app_data(stop_handle.clone())
                .service(hello)
                .service(stop)
                .wrap(middleware::Logger::default())
        }
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run();

    // register the server handle with the stop handle
    stop_handle.register(srv.handle());

    // run server until stopped (either by ctrl-c or stop endpoint)
    srv.await
}

#[derive(Default)]
struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        #[allow(clippy::let_underscore_future)]
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}
