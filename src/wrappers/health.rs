use std::collections::BTreeMap;

use serde_json::Value;

use crate::{
    auth::ExecutionAuthorization,
    gateway::{
        ToolCallRequest, WrapperExecutionContext, WrapperExecutionError, WrapperExecutionOutput,
        WrapperExecutor,
    },
};

pub struct HealthCheckWrapper;

impl WrapperExecutor for HealthCheckWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Ok(WrapperExecutionOutput {
            result: Some(health_check_result()),
        })
    }
}

fn health_check_result() -> BTreeMap<String, Value> {
    BTreeMap::from([
        (
            "service".to_string(),
            Value::String("aegis-gateway".to_string()),
        ),
        ("status".to_string(), Value::String("healthy".to_string())),
        (
            "wrapper".to_string(),
            Value::String("health.check".to_string()),
        ),
    ])
}
