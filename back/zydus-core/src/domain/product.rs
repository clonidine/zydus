use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub id: Option<u64>,
    pub price: BigDecimal,
    pub description: Option<String>,
}
