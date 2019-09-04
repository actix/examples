use std::error;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::{future, Future, Stream};
use futures_timer::Delay;
use serde_json;
use serde_json::Value;

#[allow(dead_code)]
mod convention;

/// The main handler for JSONRPC server.
fn rpc_handler(
    req: HttpRequest,
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    payload.concat2().from_err().and_then(move |body| {
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
        let app_state = req.app_data().unwrap();
        let mut result = convention::Response::default();
        result.id = reqjson.id.clone();

        match rpc_select(&app_state, reqjson.method.as_str(), reqjson.params) {
            Ok(ok) => result.result = ok,
            Err(e) => result.error = Some(e),
        }

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(result.dump()))
    })
}

fn rpc_select(
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
            match app_state
                .network
                .read()
                .unwrap()
                .wait(params[0].as_u64().unwrap())
                .wait()
            {
                Ok(ok) => Ok(Value::from(ok)),
                Err(e) => Err(convention::ErrorData::new(500, &format!("{:?}", e)[..])),
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
    fn wait(
        &self,
        d: u64,
    ) -> Box<dyn Future<Item = String, Error = Box<dyn error::Error>>>;

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

    fn wait(
        &self,
        d: u64,
    ) -> Box<dyn Future<Item = String, Error = Box<dyn error::Error>>> {
        if let Err(e) = Delay::new(Duration::from_secs(d)).wait() {
            let e: Box<dyn error::Error> = Box::new(e);
            return Box::new(future::err(e));
        };
        Box::new(future::ok(String::from("pong")))
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

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let network = Arc::new(RwLock::new(ObjNetwork::new()));

    let sys = actix::System::new("actix_jrpc");
    HttpServer::new(move || {
        let app_state = AppState::new(network.clone());
        App::new()
            .data(app_state)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to_async(rpc_handler)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .workers(1)
    .start();

    let _ = sys.run();
}
