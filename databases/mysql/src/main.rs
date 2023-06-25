mod db_layer;

use actix_web::{get, post, web, App, HttpServer, Responder};
use dotenv::dotenv;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::str;

#[derive(Deserialize)]
struct BankData {
    bank_name: String,
    country: String,
}

#[derive(Deserialize)]
struct BranchData {
    branch_name: String,
    location: String,
}

#[derive(Deserialize)]
struct TellerData {
    teller_name: String,
    branch_name: String,
}

#[derive(Deserialize)]
struct CustomerData {
    customer_name: String,
    branch_name: String,
}

// output
#[derive(Serialize)]
pub struct ResponseStatus {
    pub status_code: u8,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct BankResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub bank_data: Vec<BankDetails>,
}

#[derive(Serialize)]
pub struct BranchDetails {
    pub branch_name: String,
    pub location: String,
}

#[derive(Serialize)]
pub struct BranchResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub branch_data: Vec<BranchDetails>,
}

#[derive(Serialize)]
pub struct TellerDetails {
    pub teller_name: String,
    pub branch_name: String,
}

#[derive(Serialize)]
pub struct TellerResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub teller_data: Vec<TellerDetails>,
}

#[derive(Serialize)]
pub struct CustomerDetails {
    pub customer_name: String,
    pub branch_name: String,
}

#[derive(Serialize)]
pub struct CustomerResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub customer_data: Vec<CustomerDetails>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("")
}

#[post("/addbank")]
async fn add_bank(bank_data: web::Json<BankData>, data: web::Data<Pool>) -> impl Responder {
    let bank_name = &bank_data.bank_name;
    let _country = &bank_data.country;

    let response_data = db_layer::create_bank(&data, bank_name.to_string(), _country.to_string());

    web::Json(response_data)
}

#[post("/addbranch")]
async fn add_branch(branch_data: web::Json<BranchData>, data: web::Data<Pool>) -> impl Responder {
    let branch_name = &branch_data.branch_name;
    let _location = &branch_data.location;

    let response_data =
        db_layer::create_branch(&data, branch_name.to_string(), _location.to_string());

    web::Json(response_data)
}

#[post("/addteller")]
async fn add_teller(teller_data: web::Json<TellerData>, data: web::Data<Pool>) -> impl Responder {
    let teller_name = &teller_data.teller_name;
    let branch_name = &teller_data.branch_name;

    let response_data =
        db_layer::create_teller(&data, teller_name.to_string(), branch_name.to_string());

    web::Json(response_data)
}

#[post("/addcustomer")]
async fn add_customer(
    customer_data: web::Json<CustomerData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let customer_name = &customer_data.customer_name;
    let branch_name = &customer_data.branch_name;

    let response_data =
        db_layer::create_customer(&data, customer_name.to_string(), branch_name.to_string());

    web::Json(response_data)
}

#[get("/getbank")]
async fn get_bank(data: web::Data<Pool>) -> impl Responder {
    let bank_response_data = db_layer::get_bank_data(&data);

    web::Json(bank_response_data)
}

#[get("/getbranch")]
async fn get_branch(data: web::Data<Pool>) -> impl Responder {
    let branch_response_data = db_layer::get_branch_data(&data);

    web::Json(branch_response_data)
}

#[get("/getteller")]
async fn get_teller(data: web::Data<Pool>) -> impl Responder {
    let teller_response_data = db_layer::get_teller_data(&data);

    web::Json(teller_response_data)
}

#[get("/getcustomer")]
async fn get_customer(data: web::Data<Pool>) -> impl Responder {
    let customer_response_data = db_layer::get_customer_data(&data);

    web::Json(customer_response_data)
}

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> OptsBuilder {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));
    builder
}

#[actix_web::main]
async fn main() {
    // get env vars
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let my_db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let mut http_server_status = String::from("[info] ActixWebHttpServer - Listening for HTTP on ");
    let db_port: u16 = match my_db_port.parse::<u16>() {
        Ok(a) => a,
        Err(e) => 0,
    };

    // if your password contains dollar sign "$" the remember to escape it
    // db_password = "123$abc" will need to be changed to db_password = "123\$abc"

    http_server_status.push_str(&server_addr);

    let builder: OptsBuilder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = match Pool::new(builder) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
            return;
        }
    };

    let shared_data = web::Data::new(pool);

    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(index)
            .service(add_bank)
            .service(add_branch)
            .service(add_teller)
            .service(add_customer)
            .service(get_bank)
            .service(get_branch)
            .service(get_teller)
            .service(get_customer)
    })
    .bind(server_addr)
    {
        Ok(s) => {
            println!("{:?}", http_server_status);
            s
        }
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };

    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}
