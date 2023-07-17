use mysql::{params, prelude::*};

use crate::{
    BankDetails, BankResponseData, BranchDetails, BranchResponseData, CustomerDetails,
    CustomerResponseData, ResponseStatus, TellerDetails, TellerResponseData,
};

const ERROR_MESSAGE: &str = "Error occurred during processing, please try again.";

pub fn create_bank(pool: &mysql::Pool, bank_name: String, _country: String) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description = ERROR_MESSAGE.to_owned();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if bank_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Bank name is empty!");
        return response_status;
    }

    if _country.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Country is empty!");
        return response_status;
    }

    match pool.get_conn().and_then(|mut conn| {
        insert_bank_data(&mut conn, bank_name.to_lowercase(), _country.to_lowercase())
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = "Successful".to_owned();
            }
        }
        Err(err) => println!("Failed to open DB connection. create_bank {err:?}"),
    }

    response_status
}

pub fn create_branch(pool: &mysql::Pool, branch_name: String, _location: String) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description = ERROR_MESSAGE.to_owned();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if branch_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    if _location.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Location is empty!");
        return response_status;
    }

    match pool.get_conn().and_then(|mut conn| {
        insert_branch_data(
            &mut conn,
            branch_name.to_lowercase(),
            _location.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = "Successful".to_owned();
            }
        }
        Err(err) => println!("Failed to open DB connection. create_branch {err:?}"),
    }

    response_status
}

pub fn create_teller(
    pool: &mysql::Pool,
    teller_name: String,
    branch_name: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description = ERROR_MESSAGE.to_owned();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if teller_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Teller name is empty!");
        return response_status;
    }

    if branch_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    match pool.get_conn().and_then(|mut conn| {
        insert_teller_data(
            &mut conn,
            teller_name.to_lowercase(),
            branch_name.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = "Successful".to_owned();
            }
        }
        Err(err) => println!("Failed to open DB connection. create_teller {err:?}"),
    }

    response_status
}

pub fn create_customer(
    pool: &mysql::Pool,
    customer_name: String,
    branch_name: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description = ERROR_MESSAGE.to_owned();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if customer_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Customer name is empty!");
        return response_status;
    }

    if branch_name.replace(' ', "").trim().is_empty() {
        response_status.status_description = String::from("Branch name is empty!");
        return response_status;
    }

    match pool.get_conn().and_then(|mut conn| {
        insert_customer_data(
            &mut conn,
            customer_name.to_lowercase(),
            branch_name.to_lowercase(),
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = "Successful".to_owned();
            }
        }
        Err(err) => println!("Failed to open DB connection. create_customer {err:?}"),
    }

    response_status
}

pub fn get_bank_data(pool: &mysql::Pool) -> BankResponseData {
    let mut vec_bank_data = Vec::new();
    let mut my_status_code = 1_u8;
    let mut my_status_description: String = String::from("Record not found");

    match pool
        .get_conn()
        .and_then(|mut conn| select_bank_details(&mut conn))
    {
        Ok(data) => vec_bank_data = data,
        Err(err) => println!("Failed to open DB connection. {err:?}"),
    }

    if !vec_bank_data.is_empty() {
        my_status_code = 0;
        my_status_description = "Successful".to_owned();
    }

    BankResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        bank_data: vec_bank_data,
    }
}

pub fn get_branch_data(pool: &mysql::Pool) -> BranchResponseData {
    let mut vec_branch_data = Vec::new();
    let mut my_status_code = 1_u8;
    let mut my_status_description: String = String::from("Record not found");

    match pool
        .get_conn()
        .and_then(|mut conn| select_branch_details(&mut conn))
    {
        Ok(data) => vec_branch_data = data,
        Err(err) => println!("Failed to open DB connection. {err:?}"),
    }

    if !vec_branch_data.is_empty() {
        my_status_code = 0;
        my_status_description = "Successful".to_owned();
    }

    //Assign values to struct variable

    BranchResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        branch_data: vec_branch_data,
    }
}

pub fn get_teller_data(pool: &mysql::Pool) -> TellerResponseData {
    let mut vec_teller_data = Vec::new();
    let mut my_status_code = 1_u8;
    let mut my_status_description: String = String::from("Record not found");

    match pool
        .get_conn()
        .and_then(|mut conn| select_teller_details(&mut conn))
    {
        Ok(data) => vec_teller_data = data,
        Err(err) => println!("Failed to open DB connection. {err:?}"),
    }

    if !vec_teller_data.is_empty() {
        my_status_code = 0;
        my_status_description = "Successful".to_owned();
    }

    TellerResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        teller_data: vec_teller_data,
    }
}

pub fn get_customer_data(pool: &mysql::Pool) -> CustomerResponseData {
    let mut vec_customer_data = Vec::new();
    let mut my_status_code = 1_u8;
    let mut my_status_description: String = String::from("Record not found");

    match pool
        .get_conn()
        .and_then(|mut conn| select_customer_details(&mut conn))
    {
        Ok(data) => vec_customer_data = data,
        Err(err) => println!("Failed to open DB connection. {err:?}"),
    }

    if !vec_customer_data.is_empty() {
        my_status_code = 0;
        my_status_description = "Successful".to_owned();
    }

    CustomerResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        customer_data: vec_customer_data,
    }
}

fn insert_bank_data(
    conn: &mut mysql::PooledConn,
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
    .map(|_| conn.last_insert_id())
}

fn insert_branch_data(
    conn: &mut mysql::PooledConn,
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
    .map(|_| conn.last_insert_id())
}

fn insert_teller_data(
    conn: &mut mysql::PooledConn,
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
    ).map(|_| conn.last_insert_id())
}

fn insert_customer_data(
    conn: &mut mysql::PooledConn,
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
    ).map(|_| conn.last_insert_id())
}

fn select_bank_details(
    conn: &mut mysql::PooledConn,
) -> std::result::Result<Vec<BankDetails>, mysql::error::Error> {
    let mut bank_data = Vec::new();

    conn.query_map(
        "select bank_name, country from bank_details where length(trim(coalesce(bank_name,''))) > 0 and length(trim(coalesce(country,''))) > 0 order by id asc;",
            |(my_bank_name, my_country)| {
                let bank_details = BankDetails { bank_name: my_bank_name, country: my_country, };
                bank_data.push(bank_details);
            },
    ).map(|_| bank_data)
}

fn select_branch_details(
    conn: &mut mysql::PooledConn,
) -> std::result::Result<Vec<BranchDetails>, mysql::error::Error> {
    let mut branch_data = Vec::new();

    conn.query_map(
        "select branch_name, location from branch_details where length(trim(coalesce(branch_name,''))) > 0 and length(trim(coalesce(location,''))) > 0 order by id asc;",
            |(my_branch_name, my_location)| {
                let branch_details = BranchDetails { branch_name: my_branch_name, location: my_location, };
                branch_data.push(branch_details);
            },
    ).map(|_| branch_data)
}

fn select_teller_details(
    conn: &mut mysql::PooledConn,
) -> std::result::Result<Vec<TellerDetails>, mysql::error::Error> {
    let mut teller_data = Vec::new();

    conn.query_map(
        "select teller_name, branch_name from teller_details where length(trim(coalesce(teller_name,''))) > 0 and length(trim(coalesce(branch_name,''))) > 0 order by id asc;",
            |(my_teller_name, my_branch_name)| {
                let teller_details = TellerDetails { teller_name: my_teller_name, branch_name: my_branch_name, };
                teller_data.push(teller_details);
            },
    ).map(|_| teller_data)
}

fn select_customer_details(
    conn: &mut mysql::PooledConn,
) -> std::result::Result<Vec<CustomerDetails>, mysql::error::Error> {
    let mut customer_data = Vec::new();

    conn.query_map(
        "select customer_name, branch_name from customer_details where length(trim(coalesce(customer_name,''))) > 0 and length(trim(coalesce(branch_name,''))) > 0 order by id asc;",
            |(my_customer_name, my_branch_name)| {
                let teller_details = CustomerDetails { customer_name: my_customer_name, branch_name: my_branch_name, };
                customer_data.push(teller_details);
            },
    ).map(|_| customer_data)
}
