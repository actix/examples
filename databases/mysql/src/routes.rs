use actix_web::{get, post, web, HttpResponse, Responder};

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
    let _country = bank_data.country;

    create_bank(&data, bank_name, _country)?;

    Ok(HttpResponse::NoContent())
}

#[post("/branch")]
pub(crate) async fn add_branch(
    web::Json(branch_data): web::Json<BranchData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let branch_name = branch_data.branch_name;
    let _location = branch_data.location;

    let response_data = create_branch(&data, branch_name, _location);

    Ok(web::Json(response_data))
}

#[post("/teller")]
pub(crate) async fn add_teller(
    web::Json(teller_data): web::Json<TellerData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let teller_name = teller_data.teller_name;
    let branch_name = teller_data.branch_name;

    let response_data = create_teller(&data, teller_name, branch_name);

    Ok(web::Json(response_data))
}

#[post("/customer")]
pub(crate) async fn add_customer(
    web::Json(customer_data): web::Json<CustomerData>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let customer_name = customer_data.customer_name;
    let branch_name = customer_data.branch_name;

    let response_data = create_customer(&data, customer_name, branch_name);

    Ok(web::Json(response_data))
}

#[get("/bank")]
pub(crate) async fn get_bank(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let bank_response_data = get_bank_data(&data);
    Ok(web::Json(bank_response_data))
}

#[get("/branch")]
pub(crate) async fn get_branch(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let branch_response_data = get_branch_data(&data);
    Ok(web::Json(branch_response_data))
}

#[get("/teller")]
pub(crate) async fn get_teller(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let teller_response_data = get_teller_data(&data);
    Ok(web::Json(teller_response_data))
}

#[get("/customer")]
pub(crate) async fn get_customer(
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let customer_response_data = get_customer_data(&data);
    Ok(web::Json(customer_response_data))
}
