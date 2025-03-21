use actix_web::{
    App, Error, HttpServer, Responder,
    body::{self, MessageBody},
    dev::{self, ServiceResponse},
    middleware::{Logger, Next, from_fn},
    web::{self, Data, Json},
};
use aes_gcm_siv::{Aes256GcmSiv, KeyInit as _, Nonce, aead::Aead as _};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Req {
    id: u64,
    data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {
    data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<String>,
}

async fn make_encrypted(
    cipher: Data<Aes256GcmSiv>,
    Json(Req { id, data, .. }): Json<Req>,
) -> impl Responder {
    log::info!("creating encrypted sample request for ID = {id:?}");

    // this nonce should actually be unique per message in a production environment
    let nonce = Nonce::from_slice(b"unique nonce");
    let nonce_b64 = Some(BASE64_STANDARD.encode(nonce));

    let data_enc = cipher.encrypt(nonce, data.as_bytes()).unwrap();
    let data_enc = BASE64_STANDARD.encode(data_enc);

    web::Json(Req {
        id,
        nonce: nonce_b64,
        data: data_enc,
    })
}

async fn reverse_data(Json(Req { id, data, .. }): Json<Req>) -> impl Responder {
    log::info!("request {id:?} with data: {data}");

    let data_rev = data.chars().rev().collect();

    log::info!("response {id:?} with new data: {data_rev}");

    Json(Res {
        data: data_rev,
        nonce: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    // initialize cipher outside HttpServer closure
    let cipher = Aes256GcmSiv::new_from_slice(&[0; 32]).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(cipher.clone()))
            .service(web::resource("/encrypt").route(web::post().to(make_encrypted)))
            .service(
                web::resource("/reverse")
                    .route(web::post().to(reverse_data))
                    .wrap(from_fn(encrypt_payloads)),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}

async fn encrypt_payloads(
    mut req: dev::ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<dev::ServiceResponse<impl MessageBody>, Error> {
    // get cipher from app data
    let cipher = req.extract::<web::Data<Aes256GcmSiv>>().await.unwrap();

    // extract JSON with encrypted+encoded data field
    let Json(Req { id, nonce, data }) = req.extract::<Json<Req>>().await?;

    log::info!("decrypting request {id:?}");

    // decode nonce from payload
    let nonce = BASE64_STANDARD.decode(nonce.unwrap()).unwrap();
    let nonce = Nonce::from_slice(&nonce);

    // decode and decrypt data field
    let data_enc = BASE64_STANDARD.decode(&data).unwrap();
    let data = cipher.decrypt(nonce, data_enc.as_slice()).unwrap();

    // construct request body format with plaintext data
    let req_body = Req {
        id,
        nonce: None,
        data: String::from_utf8(data).unwrap(),
    };

    // encode request body as JSON
    let req_body = serde_json::to_vec(&req_body).unwrap();

    // re-insert request body
    req.set_payload(bytes_to_payload(web::Bytes::from(req_body)));

    // call next service
    let res = next.call(req).await?;

    log::info!("encrypting response {id:?}");

    // deconstruct response into parts
    let (req, res) = res.into_parts();
    let (res, body) = res.into_parts();

    // Read all bytes out of response stream. Only use `to_bytes` if you can guarantee all handlers
    // wrapped by this middleware return complete responses or bounded streams.
    let body = body::to_bytes(body).await.ok().unwrap();

    // parse JSON from response body
    let Res { data, .. } = serde_json::from_slice(&body).unwrap();

    // generate and encode nonce for later
    let nonce = Nonce::from_slice(b"unique nonce");
    let nonce_b64 = Some(BASE64_STANDARD.encode(nonce));

    // encrypt and encode data field
    let data_enc = cipher.encrypt(nonce, data.as_bytes()).unwrap();
    let data_enc = BASE64_STANDARD.encode(data_enc);

    // re-pack response into JSON format
    let res_body = Res {
        data: data_enc,
        nonce: nonce_b64,
    };
    let res_body_enc = serde_json::to_string(&res_body).unwrap();

    // set response body as new JSON payload and re-combine response object
    let res = res.set_body(res_body_enc);
    let res = ServiceResponse::new(req, res);

    Ok(res)
}

fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = actix_http::h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
