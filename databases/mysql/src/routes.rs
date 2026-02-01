use actix_web::{HttpResponse, Responder, get, post, web};

use crate::{
    models::{BankData, BranchData, CustomerData, TellerData},
    persistence::{
        create_bank, create_branch, create_customer, create_teller, get_bank_data, get_branch_data,
        get_customer_data, get_teller_data,
    },
};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    String::new()
}

#[post("/bank")]
pub(crate) async fn add_bank(
    web::Json(bank_data): web::Json<BankData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let bank_name = bank_data.bank_name;
    let country = bank_data.country;

    web::block(move || create_bank(&data, bank_name, country)).await??;

    Ok(HttpResponse::NoContent())
}

#[post("/branch")]
pub(crate) async fn add_branch(
    web::Json(branch_data): web::Json<BranchData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let branch_name = branch_data.branch_name;
    let location = branch_data.location;

    web::block(move || create_branch(&data, branch_name, location)).await??;

    Ok(HttpResponse::NoContent())
}

#[post("/teller")]
pub(crate) async fn add_teller(
    web::Json(teller_data): web::Json<TellerData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let teller_name = teller_data.teller_name;
    let branch_name = teller_data.branch_name;

    web::block(move || create_teller(&data, teller_name, branch_name)).await??;

    Ok(HttpResponse::NoContent())
}

#[post("/customer")]
pub(crate) async fn add_customer(
    web::Json(customer_data): web::Json<CustomerData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let customer_name = customer_data.customer_name;
    let branch_name = customer_data.branch_name;

    web::block(move || create_customer(&data, customer_name, branch_name)).await??;

    Ok(HttpResponse::NoContent())
}

#[get("/bank")]
pub(crate) async fn get_bank(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let bank_response_data = web::block(move || get_bank_data(&data)).await??;
    Ok(web::Json(bank_response_data))
}

#[get("/branch")]
pub(crate) async fn get_branch(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let branch_response_data = web::block(move || get_branch_data(&data)).await??;
    Ok(web::Json(branch_response_data))
}

#[get("/teller")]
pub(crate) async fn get_teller(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let teller_response_data = web::block(move || get_teller_data(&data)).await??;
    Ok(web::Json(teller_response_data))
}

#[get("/customer")]
pub(crate) async fn get_customer(
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let customer_response_data = web::block(move || get_customer_data(&data)).await??;
    Ok(web::Json(customer_response_data))
}
