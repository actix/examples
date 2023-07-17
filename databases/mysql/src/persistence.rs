use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};

use crate::models::{
    BankDetails, BankResponseData, BranchDetails, BranchResponseData, CustomerDetails,
    CustomerResponseData, TellerDetails, TellerResponseData,
};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyBankName,
    EmptyCountry,
    EmptyBranch,
    EmptyLocation,
    EmptyTellerName,
    EmptyCustomerName,

    MysqlError(mysql::Error),

    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyBankName
            | PersistenceError::EmptyCountry
            | PersistenceError::EmptyBranch
            | PersistenceError::EmptyLocation
            | PersistenceError::EmptyTellerName
            | PersistenceError::EmptyCustomerName => StatusCode::BAD_REQUEST,

            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn create_bank(
    pool: &mysql::Pool,
    bank_name: String,
    country: String,
) -> Result<(), PersistenceError> {
    if bank_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyBankName);
    }

    if country.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyCountry);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id =
        insert_bank_data(&mut conn, bank_name.to_lowercase(), country.to_lowercase())?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn create_branch(
    pool: &mysql::Pool,
    branch_name: String,
    location: String,
) -> Result<(), PersistenceError> {
    if branch_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyBranch);
    }

    if location.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyLocation);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_branch_data(
        &mut conn,
        branch_name.to_lowercase(),
        location.to_lowercase(),
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn create_teller(
    pool: &mysql::Pool,
    teller_name: String,
    branch_name: String,
) -> Result<(), PersistenceError> {
    if teller_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyTellerName);
    }

    if branch_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyBranch);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_teller_data(
        &mut conn,
        teller_name.to_lowercase(),
        branch_name.to_lowercase(),
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn create_customer(
    pool: &mysql::Pool,
    customer_name: String,
    branch_name: String,
) -> Result<(), PersistenceError> {
    if customer_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyCustomerName);
    }

    if branch_name.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyBranch);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_customer_data(
        &mut conn,
        customer_name.to_lowercase(),
        branch_name.to_lowercase(),
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn get_bank_data(pool: &mysql::Pool) -> Result<BankResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(BankResponseData {
        bank_data: select_bank_details(&mut conn)?,
    })
}

pub fn get_branch_data(pool: &mysql::Pool) -> Result<BranchResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(BranchResponseData {
        branch_data: select_branch_details(&mut conn)?,
    })
}

pub fn get_teller_data(pool: &mysql::Pool) -> Result<TellerResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(TellerResponseData {
        teller_data: select_teller_details(&mut conn)?,
    })
}

pub fn get_customer_data(pool: &mysql::Pool) -> Result<CustomerResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(CustomerResponseData {
        customer_data: select_customer_details(&mut conn)?,
    })
}

/// Insert data into the database table `bank_details`.
fn insert_bank_data(
    conn: &mut mysql::PooledConn,
    my_bank_name: String,
    my_country: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO bank_details (bank_name, country)
        VALUES (:bank_name, :country)
        ",
        params! {
            "bank_name" => my_bank_name,
            "country" => my_country,
        },
    )
    .map(|_| conn.last_insert_id())
}

/// Insert data into the database table `branch_details`.
fn insert_branch_data(
    conn: &mut mysql::PooledConn,
    my_branch_name: String,
    my_location: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO branch_details (branch_name, location)
        VALUES (:branch_name, :location)
        ",
        params! {
            "branch_name" => my_branch_name,
            "location" => my_location,
        },
    )
    .map(|_| conn.last_insert_id())
}

/// Insert data into the database table `teller_details`.
fn insert_teller_data(
    conn: &mut mysql::PooledConn,
    my_teller_name: String,
    my_branch_name: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO teller_details (teller_name, branch_name)
        VALUES (:teller_name, :branch_name)
        ",
        params! {
            "teller_name" => my_teller_name,
            "branch_name" => my_branch_name,
        },
    )
    .map(|_| conn.last_insert_id())
}

/// Insert data into the database table `customer_details`.
fn insert_customer_data(
    conn: &mut mysql::PooledConn,
    my_customer_name: String,
    my_branch_name: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        r"
        INSERT INTO customer_details (customer_name, branch_name)
        VALUES (:customer_name, :branch_name)
        ",
        params! {
            "customer_name" => my_customer_name,
            "branch_name" => my_branch_name,
        },
    )
    .map(|_| conn.last_insert_id())
}

/// Lists all banks' details.
fn select_bank_details(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<BankDetails>> {
    conn.query_map(
        r"
        SELECT bank_name, country FROM bank_details
        WHERE LENGTH(TRIM(COALESCE(bank_name, ''))) > 0
        AND LENGTH(TRIM(COALESCE(country, ''))) > 0
        ORDER BY id ASC
        ",
        |(my_bank_name, my_country)| BankDetails {
            bank_name: my_bank_name,
            country: my_country,
        },
    )
}

/// Lists all branches' details.
fn select_branch_details(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<BranchDetails>> {
    conn.query_map(
        r"
        SELECT branch_name, location FROM branch_details
        WHERE LENGTH(TRIM(COALESCE(branch_name, ''))) > 0
        AND LENGTH(TRIM(COALESCE(location, ''))) > 0
        ORDER BY id ASC
        ",
        |(my_branch_name, my_location)| BranchDetails {
            branch_name: my_branch_name,
            location: my_location,
        },
    )
}

/// Lists all tellers' details.
fn select_teller_details(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<TellerDetails>> {
    conn.query_map(
        r"
        SELECT teller_name, branch_name FROM teller_details
        WHERE LENGTH(TRIM(COALESCE(teller_name, ''))) > 0
        AND LENGTH(TRIM(COALESCE(branch_name, ''))) > 0
        ORDER BY id ASC
        ",
        |(my_teller_name, my_branch_name)| TellerDetails {
            teller_name: my_teller_name,
            branch_name: my_branch_name,
        },
    )
}

/// Lists all customers' details.
fn select_customer_details(
    conn: &mut mysql::PooledConn,
) -> mysql::error::Result<Vec<CustomerDetails>> {
    conn.query_map(
        r"
        SELECT customer_name, branch_name FROM customer_details
        WHERE LENGTH(TRIM(COALESCE(customer_name, ''))) > 0
        AND LENGTH(TRIM(COALESCE(branch_name, ''))) > 0
        ORDER BY id ASC
        ",
        |(my_customer_name, my_branch_name)| CustomerDetails {
            customer_name: my_customer_name,
            branch_name: my_branch_name,
        },
    )
}
