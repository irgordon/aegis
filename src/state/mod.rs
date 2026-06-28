#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecutionReference {
    id: String,
}

impl ExecutionReference {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
