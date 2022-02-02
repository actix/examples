use actix_web::web::Data;
use actix_web::{get, App, HttpServer};

use redis_tang::{Builder, Pool, RedisManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));

    let num_cpus = num_cpus::get();

    // a shared redis pool for work load comparison.
    let pool = pool_builder(num_cpus, redis_url.as_str())
        .await
        .expect("fail to build pool");

    let pool = Data::new(RedisWrapper(pool));

    HttpServer::new(move || {
        let redis_url = redis_url.clone();

        App::new()
            .app_data(pool.clone())
            // a dummy data_factory implementation
            .data_factory(|| {
                /*
                    App::data_factory would accept a future as return type and poll the future when
                    App is initialized.

                    The Output of the future must be Result<T, E> and T will be the transformed to
                    App::Data<T> that can be extracted from handler/request.
                    (The E will only be used to trigger a log::error.)

                    This data is bound to worker thread and you get an instance of it for every
                    worker of the HttpServer.(hence the name data_factory)
                    *. It is NOT shared between workers
                    (unless the underlying data is a smart pointer like Arc<T>).
                */

                async {
                    // 123usize would be transformed into Data<usize>
                    Ok::<usize, ()>(123)
                }
            })
            // a data_factory redis pool for work load comparison.
            .data_factory(move || pool_builder(1, redis_url.clone()))
            .service(pool_shared_prebuilt)
            .service(pool_local)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
    This pool is shared between workers. We have all redis connections spawned tasks on main thread
    therefore it puts too much pressure on one thread.
    *. This is the case for redis::aio::MultiplexedConnection and it may not apply to other async
    redis connection type.
*/
#[get("/pool")]
async fn pool_shared_prebuilt(pool: Data<RedisWrapper>) -> &'static str {
    ping(&pool.as_ref().0).await
}

/*
   This pool is built with App::data_factory and we have 2 connections fixed for every worker.
   It's evenly distributed and have no cross workers synchronization.
*/
#[get("/pool2")]
async fn pool_local(data: Data<usize>, pool: Data<Pool<RedisManager>>) -> &'static str {
    assert_eq!(data.get_ref(), &123);

    ping(pool.as_ref()).await
}

// boiler plate for redis pool
#[derive(Clone)]
struct RedisWrapper(Pool<RedisManager>);

async fn pool_builder(
    num_cpus: usize,
    redis_url: impl redis::IntoConnectionInfo,
) -> Result<Pool<RedisManager>, ()> {
    let mgr = RedisManager::new(redis_url);
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

async fn ping(pool: &Pool<RedisManager>) -> &'static str {
    let mut client = pool.get().await.unwrap().clone();

    redis::cmd("PING")
        .query_async::<_, ()>(&mut client)
        .await
        .unwrap();

    "Done"
}
