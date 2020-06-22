use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

use redis_tang::{Builder, Pool, RedisManager};

// change according to your redis setting.
const REDIS_URL: &str = "redis://127.0.0.1";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let num_cpus = num_cpus::get();

    // a shared redis pool for work load comparision.
    let pool = pool_builder(num_cpus).await.expect("fail to build pool");
    let pool = Wrapper(pool);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // a dummy data_factory implementation
            .data_factory(|| {
                /*
                    App::data_factory would accept a future as return type and poll the future when App is initialized.

                    The Output of the future must be Result<T, E> and T will be the transformed to App::Data<T> that can be extracted from handler/request.
                    (The E will only be used to trigger a log::error.)

                    This data is bound to worker thread and you get an instance of it for every worker of the HttpServer.(hence the name data_factory)
                    *. It is NOT shared between workers(unless the underlying data is a smart pointer like Arc<T>).
                */

                async {
                    // 123usize would be transformed into Data<usize>
                    Ok::<usize, ()>(123)
                }
            })
            // a data_factory redis pool for work load comparision.
            .data_factory(|| pool_builder(1))
            .service(web::resource("/pool").route(web::get().to(pool_shared_prebuilt)))
            .service(web::resource("/pool2").route(web::get().to(pool_local)))
    })
    .bind("127.0.0.1:8080")?
    .workers(num_cpus)
    .run()
    .await
}

// this pool is shared between workers.
// we have all redis connections spawned tasks on main thread therefore it puts too much pressure on one thread.
// *. This is the case for redis::aio::MultiplexedConnection and it may not apply to other async redis connection type.
async fn pool_shared_prebuilt(pool: Data<Wrapper>) -> &'static str {
    let mut client = pool.0.get().await.unwrap().clone();

    redis::cmd("PING")
        .query_async::<_, ()>(&mut client)
        .await
        .unwrap();

    "Done"
}

// this pool is built with App::data_factory and we have 2 connections fixed for every worker.
// It's evenly distributed and have no cross workers synchronization.
async fn pool_local(data: Data<usize>, pool: Data<Pool<RedisManager>>) -> &'static str {
    assert_eq!(data.get_ref(), &123);

    let mut client = pool.get().await.unwrap().clone();

    redis::cmd("PING")
        .query_async::<_, ()>(&mut client)
        .await
        .unwrap();

    "Done"
}

// some boiler plate for create redis pool
#[derive(Clone)]
struct Wrapper(Pool<RedisManager>);

async fn pool_builder(num_cpus: usize) -> Result<Pool<RedisManager>, ()> {
    let mgr = RedisManager::new(REDIS_URL);
    Builder::new()
        .always_check(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .min_idle(num_cpus * 2)
        .max_size(num_cpus * 2)
        .build(mgr)
        .await
        .map_err(|_| ())
}
