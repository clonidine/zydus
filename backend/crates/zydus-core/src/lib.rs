pub struct Product {
    name: String,
}

impl Product {
    pub fn name(&self) -> &str {
        &self.name
    }
}
