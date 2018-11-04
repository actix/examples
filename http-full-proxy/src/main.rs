#![deny(warnings)]
extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate failure;
extern crate futures;
extern crate url;

use actix_web::{
    client, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};
use clap::{value_t, Arg};
use futures::Future;
use std::net::ToSocketAddrs;
use url::Url;

struct AppState {
    forward_url: Url,
}

impl AppState {
    pub fn init(forward_url: Url) -> AppState {
        AppState { forward_url }
    }
}

fn forward(
    req: &HttpRequest<AppState>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let mut new_url = req.state().forward_url.clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    client::ClientRequest::build_from(req)
        .no_default_headers()
        .uri(new_url)
        .streaming(req.payload())
        .unwrap()
        .send()
        .map_err(Error::from)
        .and_then(move |resp| {
            let mut client_resp = HttpResponse::build(resp.status());
            for (header_name, header_value) in
                resp.headers().iter().filter(|(h, _)| *h != "connection")
            {
                client_resp.header(header_name.clone(), header_value.clone());
            }
            Ok(client_resp.streaming(resp.payload()))
        }).responder()
}

fn main() {
    let matches = clap::App::new("HTTP Proxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("listen_port")
                .takes_value(true)
                .value_name("LISTEN PORT")
                .index(2)
                .required(true),
        ).arg(
            Arg::with_name("forward_addr")
                .takes_value(true)
                .value_name("FWD ADDR")
                .index(3)
                .required(true),
        ).arg(
            Arg::with_name("forward_port")
                .takes_value(true)
                .value_name("FWD PORT")
                .index(4)
                .required(true),
        ).get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let listen_port = value_t!(matches, "listen_port", u16).unwrap_or_else(|e| e.exit());

    let forwarded_addr = matches.value_of("forward_addr").unwrap();
    let forwarded_port =
        value_t!(matches, "forward_port", u16).unwrap_or_else(|e| e.exit());

    let forward_url = Url::parse(&format!(
        "http://{}",
        (forwarded_addr, forwarded_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    )).unwrap();

    server::new(move || {
        App::with_state(AppState::init(forward_url.clone())).default_resource(|r| {
            r.f(forward);
        })
    }).workers(32)
    .bind((listen_addr, listen_port))
    .expect("Cannot bind listening port")
    .system_exit()
    .run();
}
