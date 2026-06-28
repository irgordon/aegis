#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayRequest {
    request_id: String,
    tool_name: String,
}

impl GatewayRequest {
    pub fn new(request_id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            tool_name: tool_name.into(),
        }
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }
}
