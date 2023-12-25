pub struct Product {
    pub name: String,
    pub id: Option<u64>,
    pub description: Option<String>,
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
