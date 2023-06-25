use crate::BankDetails;
use crate::BankResponseData;
use crate::BranchDetails;
use crate::BranchResponseData;
use crate::CustomerDetails;
use crate::CustomerResponseData;
use crate::ResponseStatus;
use crate::TellerDetails;
use crate::TellerResponseData;
use actix_web::web;
use mysql::prelude::*;
use mysql::*;

const ERROR_MESSAGE: &str = "Error occured during processing, please try again.";

pub fn create_bank(data: &web::Data<Pool>, bank_name: String, _country: String) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if bank_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Bank name is empty!");
        return response_status;
    }

    if _country.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Country is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_bank_data(&mut conn, bank_name.to_lowercase(), _country.to_lowercase())
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_bank {:?}", e),
    }

    response_status
}

pub fn create_branch(
    data: &web::Data<Pool>,
    branch_name: String,
    _location: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if branch_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    if _location.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Location is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_branch_data(
            &mut conn,
            branch_name.to_lowercase(),
            _location.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_branch {:?}", e),
    }

    response_status
}

pub fn create_teller(
    data: &web::Data<Pool>,
    teller_name: String,
    branch_name: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if teller_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Teller name is empty!");
        return response_status;
    }

    if branch_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_teller_data(
            &mut conn,
            teller_name.to_lowercase(),
            branch_name.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_teller {:?}", e),
    }

    response_status
}

pub fn create_customer(
    data: &web::Data<Pool>,
    customer_name: String,
    branch_name: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if customer_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Customer name is empty!");
        return response_status;
    }

    if branch_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_customer_data(
            &mut conn,
            customer_name.to_lowercase(),
            branch_name.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_customer {:?}", e),
    }

    response_status
}

pub fn get_bank_data(data: &web::Data<Pool>) -> BankResponseData {
    let mut vec_bank_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_bank_details(&mut conn))
    {
        Ok(s) => {
            vec_bank_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_bank_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = BankResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        bank_data: vec_bank_data,
    };

    output_data
}

pub fn get_branch_data(data: &web::Data<Pool>) -> BranchResponseData {
    let mut vec_branch_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_branch_details(&mut conn))
    {
        Ok(s) => {
            vec_branch_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_branch_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = BranchResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        branch_data: vec_branch_data,
    };

    output_data
}

pub fn get_teller_data(data: &web::Data<Pool>) -> TellerResponseData {
    let mut vec_teller_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_teller_details(&mut conn))
    {
        Ok(s) => {
            vec_teller_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_teller_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = TellerResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        teller_data: vec_teller_data,
    };

    output_data
}

pub fn get_customer_data(data: &web::Data<Pool>) -> CustomerResponseData {
    let mut vec_customer_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_customer_details(&mut conn))
    {
        Ok(s) => {
            vec_customer_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_customer_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = CustomerResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        customer_data: vec_customer_data,
    };

    output_data
}

fn insert_bank_data(
    conn: &mut PooledConn,
    my_bank_name: String,
    my_country: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table bank_details
    conn.exec_drop(
        "insert into bank_details (bank_name, country) values (:bank_name, :country);",
        params! {
            "bank_name" => my_bank_name,
            "country" => my_country,
        },
    )
    .and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_branch_data(
    conn: &mut PooledConn,
    my_branch_name: String,
    my_location: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table branch_details
    conn.exec_drop(
        "insert into branch_details (branch_name, location) values (:branch_name, :location);",
        params! {
            "branch_name" => my_branch_name,
            "location" => my_location,
        },
    )
    .and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_teller_data(
    conn: &mut PooledConn,
    my_teller_name: String,
    my_branch_name: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table teller_details
    conn.exec_drop(
        "insert into teller_details (teller_name, branch_name) values (:teller_name, :branch_name);",
        params! {
            "teller_name" => my_teller_name,
            "branch_name" => my_branch_name,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_customer_data(
    conn: &mut PooledConn,
    my_customer_name: String,
    my_branch_name: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table customer_details
    conn.exec_drop(
        "insert into customer_details (customer_name, branch_name) values (:customer_name, :branch_name);",
        params! {
            "customer_name" => my_customer_name,
            "branch_name" => my_branch_name,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}
/*
fn select_bank_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<BankDetails>, mysql::error::Error> {
    let mut bank_data = Vec::new();

    conn.exec_map(
        "select bank_name, country from bank_details where length(trim(coalesce(bank_name,''))) > :my_value and length(trim(coalesce(country,''))) > :my_value order by id asc;",
        params! {
                "my_value" => 0,
            },
            |(my_bank_name, my_country)| {
                let bank_details = BankDetails { bank_name: my_bank_name, country: my_country, };
                bank_data.push(bank_details);
            },
    )
    .and_then(|_| Ok(bank_data))
}
*/
fn select_bank_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<BankDetails>, mysql::error::Error> {
    let mut bank_data = Vec::new();

    conn.query_map(
        "select bank_name, country from bank_details where length(trim(coalesce(bank_name,''))) > 0 and length(trim(coalesce(country,''))) > 0 order by id asc;",
            |(my_bank_name, my_country)| {
                let bank_details = BankDetails { bank_name: my_bank_name, country: my_country, };
                bank_data.push(bank_details);
            },
    )
	.and_then(|_| Ok(bank_data))
}

fn select_branch_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<BranchDetails>, mysql::error::Error> {
    let mut branch_data = Vec::new();

    conn.query_map(
        "select branch_name, location from branch_details where length(trim(coalesce(branch_name,''))) > 0 and length(trim(coalesce(location,''))) > 0 order by id asc;",
            |(my_branch_name, my_location)| {
                let branch_details = BranchDetails { branch_name: my_branch_name, location: my_location, };
                branch_data.push(branch_details);
            },
    )
	.and_then(|_| Ok(branch_data))
}

fn select_teller_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<TellerDetails>, mysql::error::Error> {
    let mut teller_data = Vec::new();

    conn.query_map(
        "select teller_name, branch_name from teller_details where length(trim(coalesce(teller_name,''))) > 0 and length(trim(coalesce(branch_name,''))) > 0 order by id asc;",
            |(my_teller_name, my_branch_name)| {
                let teller_details = TellerDetails { teller_name: my_teller_name, branch_name: my_branch_name, };
                teller_data.push(teller_details);
            },
    )
	.and_then(|_| Ok(teller_data))
}

fn select_customer_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<CustomerDetails>, mysql::error::Error> {
    let mut customer_data = Vec::new();

    conn.query_map(
        "select customer_name, branch_name from customer_details where length(trim(coalesce(customer_name,''))) > 0 and length(trim(coalesce(branch_name,''))) > 0 order by id asc;",
            |(my_customer_name, my_branch_name)| {
                let teller_details = CustomerDetails { customer_name: my_customer_name, branch_name: my_branch_name, };
                customer_data.push(teller_details);
            },
    )
	.and_then(|_| Ok(customer_data))
}
