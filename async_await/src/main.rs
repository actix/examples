#![feature(async_await)] // TODO remove when stabilized

use futures::Future;
use futures3::{Future as Future3, FutureExt, TryFutureExt};
use actix_web::{HttpServer, App, HttpRequest, web::get};

/// Convert an async function into one which can be run by Actix
fn wrap_async_func<F, U, T, Ok, Error>(
    f: F,
) -> impl Fn(U) -> Box<dyn Future<Item = Ok, Error = Error>> + Clone + 'static
where
    Ok: 'static,
    Error: 'static,
    F: Fn(U) -> T + Clone + 'static,
    T: Future3<Output = Result<Ok, Error>> + 'static,
{
    move |u| {
        // Turn a future3 Future into futures1 Future
        let fut1 = f(u).boxed_local().compat();
        Box::new(fut1)
    }
}

async fn await_me() {}

async fn do_some_work(_req: HttpRequest) -> Result<String, actix_web::Error> {

    // Perform some kind of asynchronous work e.g. database lookup
    let () = await_me().await;

    Ok(String::from("Work work work work work work"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "127.0.0.1:8080";
    println!("Starting server on: {:?}", endpoint);
    HttpServer::new(|| {
        App::new()
            .route("/work", get().to_async(wrap_async_func(do_some_work)))
    })
        .bind(endpoint)?
        .run()?;
    Ok(())
}
