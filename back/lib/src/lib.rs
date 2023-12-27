use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub id: Option<u64>,
    pub price: BigDecimal,
    pub description: Option<String>,
}

pub trait Repository<T> {
    fn find_one(id: u64) -> T;
}

pub enum OrderStatus {
    PENDENT,
    CANCELLED,
    OK,
}

pub struct User {
    pub name: String,
    pub id: Option<u64>,
}
