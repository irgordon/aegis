mod builder;
mod record;

pub use builder::{AuditRecordBuilder, AuditRecordMetadata, GatewayAuditContexts};
pub use record::{AuditEventType, AuditRecord, AuditRecordDetails, AuditStatus};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AuditReference {
    id: String,
}

impl AuditReference {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
