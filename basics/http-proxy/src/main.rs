use std::net::ToSocketAddrs;

use actix_web::{
    http::StatusCode, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use awc::Client;
use clap::StructOpt;
use url::Url;

async fn forward(
    req: HttpRequest,
    payload: web::Payload,
    url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.get_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    // TODO: This forwarded implementation is incomplete as it only handles the unofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.insert_header(("x-forwarded-for", format!("{}", addr.ip())))
    } else {
        forwarded_req
    };

    let res = forwarded_req.send_stream(payload).await.map_err(|e| {
        actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }

    Ok(client_resp.streaming(res))
}

#[derive(clap::Parser, Debug)]
struct CliArguments {
    listen_addr: String,
    listen_port: u16,
    forward_addr: String,
    forward_port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CliArguments::parse();
    let forward_url = Url::parse(&format!(
        "http://{}",
        (args.forward_addr, args.forward_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    ))
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Client::new()))
            .app_data(web::Data::new(forward_url.clone()))
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
    .bind((args.listen_addr, args.listen_port))?
    .system_exit()
    .run()
    .await
}
