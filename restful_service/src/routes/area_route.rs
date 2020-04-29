use actix_http::ResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{error, http, web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};
use failure::Fail;
//use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::area_model;

#[derive(Fail, Debug)]
#[fail(display = "Runtime Exception")]
pub struct Exception {
    name: String, // &'static str,
}

impl error::ResponseError for Exception {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(200).unwrap()
    }

    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(
                http::header::CONTENT_TYPE,
                "application/json; charset=utf-8",
            )
            .body(
                serde_json::to_string(&RespError {
                    code: -1,
                    message: String::from(&self.name),
                })
                .unwrap(),
            )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resp<T> {
    //where T: Serializer{
    code: i8,
    message: String,
    data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespError {
    code: i8,
    message: String,
}

impl<T> Resp<T> {
    pub fn ok(data: Option<T>) -> Resp<T> {
        Resp {
            code: 0,
            message: String::from("success"),
            data,
        }
    }

    pub fn error(data: Option<T>) -> Resp<T> {
        Resp {
            code: -1,
            message: String::from("error"),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Province {
    id: i32,
    name: String,
}

pub async fn provinces(pool: web::Data<Pool>) -> Result<HttpResponse, Exception> {
    let conn: Client = pool.get().await.unwrap();

    let vec = conn
        .query("SELECT * FROM public.base_region LIMIT 10", &[])
        .await
        .map_err(|_e| Exception {
            name: _e.to_string(),
        })?;
    let mut rst: Vec<Province> = Vec::new();
    for row in &vec {
        let id: i32 = row.get("id");
        let name: String = row.get("province");
        rst.push(Province { id, name })
    }
    Ok(HttpResponse::Ok().json(Resp::ok(Some(rst))))
}

pub async fn list_simple_all_provinces(pool: web::Data<Pool>) -> Result<HttpResponse, Exception> {
    let client = pool.get().await.unwrap();
    let provinces = area_model::list_all_simple_provinces(&client)
        .await
        .map_err(|e| Exception {
            name: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(Resp::ok(Some(provinces))))
    //todo!()
}

pub async fn list_cities_by_province_id(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Exception> {
    let pid = query
        .get("pid")
        .ok_or(Exception {
            name: String::from("parameters are illegal"),
        })?
        .parse::<i32>()
        .map_err(|_| Exception {
            name: String::from("cant convert paramter into number"),
        })?;

    let client = pool.get().await.unwrap();
    let cities = area_model::list_cities_by_province_id(&client, pid)
        .await
        .map_err(|e| Exception {
            name: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(Resp::ok(Some(cities))))
    //todo!()
}
