use actix_web::{middleware, web, App, HttpServer, Error, HttpResponse};
use futures::future::{join_all};
use redis_async::{client, resp_array};
use std::net::{ToSocketAddrs};

async fn write_stuff() -> Result<HttpResponse, Error> {
    let server_details = "redis:6379";
    let server: Vec<_> = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .collect();
    let connection = client::paired_connect(&server[0])
    .await
    .expect("Cannot open connection");

    let test_data: Vec<_> = (0..100).map(|x| (x, x.to_string())).collect();

    let futures = test_data.into_iter().map(|data| {
        let connection_inner = connection.clone();
        let incr_f = connection.send(resp_array!["INCR", "realistic_test_ctr"]);
        async move {
            let ctr: String = incr_f.await.expect("Cannot increment");

            let key = format!("rt_{}", ctr);
            let d_val = data.0.to_string();
            connection_inner.send_and_forget(resp_array!["SET", &key, d_val]);
            connection_inner
                .send(resp_array!["SET", data.1, key])
                .await
                .expect("Cannot set")
        }
    });
    let _result: Vec<String> = join_all(futures).await;

    Ok(HttpResponse::Ok().into())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(write_stuff))
        )
    })
    .bind("0.0.0.0:3000")?
    .start()
    .await
}
