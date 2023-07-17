use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BankData {
    pub bank_name: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct BranchData {
    pub branch_name: String,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct TellerData {
    pub teller_name: String,
    pub branch_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerData {
    pub customer_name: String,
    pub branch_name: String,
}

#[derive(Debug, Serialize)]
pub struct BankDetails {
    pub bank_name: String,
    pub country: String,
}

#[derive(Debug, Serialize)]
pub struct BankResponseData {
    pub bank_data: Vec<BankDetails>,
}

#[derive(Debug, Serialize)]
pub struct BranchDetails {
    pub branch_name: String,
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct BranchResponseData {
    pub branch_data: Vec<BranchDetails>,
}

#[derive(Debug, Serialize)]
pub struct TellerDetails {
    pub teller_name: String,
    pub branch_name: String,
}

#[derive(Debug, Serialize)]
pub struct TellerResponseData {
    pub teller_data: Vec<TellerDetails>,
}

#[derive(Debug, Serialize)]
pub struct CustomerDetails {
    pub customer_name: String,
    pub branch_name: String,
}

#[derive(Debug, Serialize)]
pub struct CustomerResponseData {
    pub customer_data: Vec<CustomerDetails>,
}
