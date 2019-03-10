#![deny(warnings)]
extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate failure;
extern crate futures;
extern crate url;

use actix_web::{
    client, http, server, App, AsyncResponder, Error, HttpMessage, HttpRequest,
    HttpResponse,
};
use clap::{value_t, Arg};
use futures::{future, Future};
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

    let mut forwarded_req = client::ClientRequest::build_from(req)
        .no_default_headers()
        .uri(new_url)
        .streaming(req.payload())
        .unwrap();

    if let Some(addr) = req.peer_addr() {
        match forwarded_req.headers_mut().entry("x-forwarded-for") {
            Ok(http::header::Entry::Vacant(entry)) => {
                let addr = format!("{}", addr.ip());
                entry.insert(addr.parse().unwrap());
            }
            Ok(http::header::Entry::Occupied(mut entry)) => {
                let addr = format!("{}, {}", entry.get().to_str().unwrap(), addr.ip());
                entry.insert(addr.parse().unwrap());
            }
            _ => unreachable!(),
        }
    }

    forwarded_req
        .send()
        .map_err(Error::from)
        .and_then(construct_response)
        .responder()
}

fn construct_response(
    resp: client::ClientResponse,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let mut client_resp = HttpResponse::build(resp.status());
    for (header_name, header_value) in
        resp.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }
    if resp.chunked().unwrap_or(false) {
        Box::new(future::ok(client_resp.streaming(resp.payload())))
    } else {
        Box::new(
            resp.body()
                .from_err()
                .and_then(move |body| Ok(client_resp.body(body))),
        )
    }
}

fn main() {
    let matches = clap::App::new("HTTP Proxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("listen_port")
                .takes_value(true)
                .value_name("LISTEN PORT")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_addr")
                .takes_value(true)
                .value_name("FWD ADDR")
                .index(3)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_port")
                .takes_value(true)
                .value_name("FWD PORT")
                .index(4)
                .required(true),
        )
        .get_matches();

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
    ))
    .unwrap();

    server::new(move || {
        App::with_state(AppState::init(forward_url.clone())).default_resource(|r| {
            r.f(forward);
        })
    })
    .workers(32)
    .bind((listen_addr, listen_port))
    .expect("Cannot bind listening port")
    .system_exit()
    .run();
}
