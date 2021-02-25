use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Product {
    id: Option<i64>,
    product_type: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Part {
    id: Option<i64>,
    part_type: Option<String>,
    name: Option<String>,
}
