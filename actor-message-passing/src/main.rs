extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix::prelude::*;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use futures::Future;

#[derive(Debug)]
pub enum SystemError {
    Decr(DecrError),
    Incr(IncrError),
    Mailbox(actix::MailboxError),
    Sum(SumError),
}

impl std::convert::From<actix::MailboxError> for SystemError {
    fn from(e: actix::MailboxError) -> Self {
        SystemError::Mailbox(e)
    }
}

impl std::convert::From<SystemError> for actix_web::Error {
    fn from(e: SystemError) -> Self {
        match e {
            SystemError::Mailbox(e) => {
                println!("MAIL BOX ERROR: {:?}", e);
                actix_web::error::ErrorInternalServerError(e)
            }
            SystemError::Decr(e) => {
                println!("DECR ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
            SystemError::Incr(e) => {
                println!("INCR ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
            SystemError::Sum(e) => {
                println!("SUM ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
        }
    }
}

#[derive(Debug)]
pub enum IncrError {
    Boom,
}

impl std::fmt::Display for IncrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Incr call failed!")
    }
}

pub struct SyncIncr {
    pub counter: i32,
}

impl Actor for SyncIncr {
    type Context = SyncContext<Self>;
}

pub struct Add {
    pub input: i32,
}

impl Message for Add {
    type Result = Result<i32, SystemError>;
}

impl Handler<Add> for SyncIncr {
    type Result = Result<i32, SystemError>;

    fn handle(&mut self, msg: Add, _: &mut Self::Context) -> Self::Result {
        self.counter = self.counter + msg.input;
        Ok(self.counter.clone())
    }
}

#[derive(Debug)]
pub enum DecrError {
    Ugh,
}

impl std::fmt::Display for DecrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Decr call failed!")
    }
}

pub struct SyncDecr {
    pub counter: i32,
}

impl Actor for SyncDecr {
    type Context = SyncContext<Self>;
}

pub struct Sub {
    pub input: i32,
}

impl Message for Sub {
    type Result = Result<i32, SystemError>;
}

impl Handler<Sub> for SyncDecr {
    type Result = Result<i32, SystemError>;

    fn handle(&mut self, msg: Sub, _: &mut Self::Context) -> Self::Result {
        self.counter = self.counter - msg.input;
        Ok(self.counter.clone())
    }
}

#[derive(Debug)]
pub enum SumError {
    Bleh,
}

impl std::fmt::Display for SumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sum call failed!")
    }
}

pub struct SyncSum {}

impl Actor for SyncSum {
    type Context = SyncContext<Self>;
}

pub struct Sum {
    pub a: i32,
    pub b: i32,
}

impl Message for Sum {
    type Result = Result<i32, SystemError>;
}

impl Handler<Sum> for SyncSum {
    type Result = Result<i32, SystemError>;

    fn handle(&mut self, msg: Sum, _: &mut Self::Context) -> Self::Result {
        Ok(msg.a + msg.b)
    }
}

struct AppState {
    decr: Addr<SyncDecr>,
    incr: Addr<SyncIncr>,
    sum: Addr<SyncSum>,
}

fn index((params, state): (Path<(i32, i32)>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let add = params.0;
    let sub = params.1;

    state
        .incr
        .send(Add { input: add })
        .join(state.decr.send(Sub { input: sub }))
        .flatten()
        .and_then(move |(a, b)| state.sum.send(Sum { a, b }).flatten())
        .map(|sum| actix_web::HttpResponse::Ok().json(sum))
        .from_err()
        .responder()
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let system = actix::System::new("actix-comms-test");

    actix_web::server::new(move || {
        actix_web::App::with_state(AppState {
            decr: SyncArbiter::start(1, || SyncDecr { counter: 100 }),
            incr: SyncArbiter::start(1, || SyncIncr { counter: 41 }),
            sum: SyncArbiter::start(1, || SyncSum {}),
        }).middleware(actix_web::middleware::Logger::default())
        .resource("/{add}/{sub}", |r| {
            r.method(actix_web::http::Method::GET).with_async(index)
        })
    }).bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");

    let _ = system.run();
}
