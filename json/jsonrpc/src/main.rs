use std::{
    error,
    pin::Pin,
    sync::{Arc, RwLock},
    time::Duration,
};

use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use bytes::Bytes;
use futures_util::FutureExt as _;
use serde_json::Value;

#[allow(dead_code)]
mod convention;

/// The main handler for JSONRPC server.
async fn rpc_handler(body: Bytes, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let reqjson: convention::Request = match serde_json::from_slice(body.as_ref()) {
        Ok(ok) => ok,
        Err(_) => {
            let r = convention::Response {
                jsonrpc: String::from(convention::JSONRPC_VERSION),
                result: Value::Null,
                error: Some(convention::ErrorData::std(-32700)),
                id: Value::Null,
            };
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(r.dump()));
        }
    };
    let mut result = convention::Response {
        id: reqjson.id.clone(),
        ..convention::Response::default()
    };

    match rpc_select(&app_state, reqjson.method.as_str(), reqjson.params).await {
        Ok(ok) => result.result = ok,
        Err(e) => result.error = Some(e),
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result.dump()))
}

async fn rpc_select(
    app_state: &AppState,
    method: &str,
    params: Vec<Value>,
) -> Result<Value, convention::ErrorData> {
    match method {
        "ping" => {
            let r = app_state.network.read().unwrap().ping();
            Ok(Value::from(r))
        }
        "wait" => {
            if params.len() != 1 || !params[0].is_u64() {
                return Err(convention::ErrorData::std(-32602));
            }

            let fut = app_state
                .network
                .read()
                .unwrap()
                .wait(params[0].as_u64().unwrap());

            match fut.await {
                Ok(ok) => Ok(Value::from(ok)),
                Err(e) => Err(convention::ErrorData::new(500, &format!("{e:?}")[..])),
            }
        }
        "get" => {
            let r = app_state.network.read().unwrap().get();
            Ok(Value::from(r))
        }
        "inc" => {
            app_state.network.write().unwrap().inc();
            Ok(Value::Null)
        }
        _ => Err(convention::ErrorData::std(-32601)),
    }
}

pub trait ImplNetwork {
    fn ping(&self) -> String;

    #[allow(clippy::type_complexity)]
    fn wait(&self, d: u64) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn error::Error>>>>>;

    fn get(&self) -> u32;

    fn inc(&mut self);
}

pub struct ObjNetwork {
    c: u32,
}

impl ObjNetwork {
    fn new() -> Self {
        Self { c: 0 }
    }
}

impl ImplNetwork for ObjNetwork {
    fn ping(&self) -> String {
        String::from("pong")
    }

    fn wait(&self, d: u64) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn error::Error>>>>> {
        async move {
            actix_web::rt::time::sleep(Duration::from_secs(d)).await;
            Ok(String::from("pong"))
        }
        .boxed_local()
    }

    fn get(&self) -> u32 {
        self.c
    }

    fn inc(&mut self) {
        self.c += 1;
    }
}

#[derive(Clone)]
pub struct AppState {
    network: Arc<RwLock<dyn ImplNetwork>>,
}

impl AppState {
    pub fn new(network: Arc<RwLock<dyn ImplNetwork>>) -> Self {
        Self { network }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let network = Arc::new(RwLock::new(ObjNetwork::new()));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let app_state = AppState::new(network.clone());
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(rpc_handler)))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}
