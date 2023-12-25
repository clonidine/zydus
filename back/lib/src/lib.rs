pub struct Product {
    pub name: String,
    pub description: Option<String>,
}

pub enum OrderStatus {
    PENDENT,
    CANCELLED,
    OK,
}

pub struct User {
    pub name: String,
}
