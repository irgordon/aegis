#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GatewayStatus {
    Allowed,
    Denied,
    Pending,
    Failed,
    Canceled,
    Replayed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GatewayError {
    GatewayExecutionUnavailable,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayResponse {
    request_id: String,
    status: GatewayStatus,
    reason: Option<String>,
    pending_reference: Option<String>,
}

impl GatewayResponse {
    pub fn allowed(request_id: impl Into<String>) -> Self {
        Self::new(request_id, GatewayStatus::Allowed, None, None)
    }

    pub fn denied(request_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new(request_id, GatewayStatus::Denied, Some(reason.into()), None)
    }

    pub fn pending(request_id: impl Into<String>, reference: impl Into<String>) -> Self {
        Self::new(
            request_id,
            GatewayStatus::Pending,
            None,
            Some(reference.into()),
        )
    }

    pub fn status(&self) -> &GatewayStatus {
        &self.status
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn pending_reference(&self) -> Option<&str> {
        self.pending_reference.as_deref()
    }

    fn new(
        request_id: impl Into<String>,
        status: GatewayStatus,
        reason: Option<String>,
        pending_reference: Option<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            status,
            reason,
            pending_reference,
        }
    }
}
