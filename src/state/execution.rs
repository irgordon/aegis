use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    Created,
    Validated,
    BundleVerified,
    PolicyEvaluated,
    Authorized,
    Dispatching,
    Executed,
    Audited,
    Completed,
    FailedClosed,
    AuditFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionTransition {
    pub previous_state: ExecutionState,
    pub execution_state: ExecutionState,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionLifecycle {
    pub execution_state: ExecutionState,
    pub transitions: Vec<ExecutionTransition>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecutionStateError {
    pub previous_state: ExecutionState,
    pub attempted_state: ExecutionState,
}

impl ExecutionLifecycle {
    pub fn created() -> Self {
        Self {
            execution_state: ExecutionState::Created,
            transitions: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, next_state: ExecutionState) -> Result<(), ExecutionStateError> {
        if !valid_transition(&self.execution_state, &next_state) {
            return Err(ExecutionStateError {
                previous_state: self.execution_state.clone(),
                attempted_state: next_state,
            });
        }

        let transition = ExecutionTransition {
            previous_state: self.execution_state.clone(),
            execution_state: next_state.clone(),
        };
        self.transitions.push(transition);
        self.execution_state = next_state;
        Ok(())
    }

    pub fn audited_completed(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionState::Audited)?;
        self.transition_to(ExecutionState::Completed)
    }
}

pub fn valid_transition(previous: &ExecutionState, next: &ExecutionState) -> bool {
    matches!(
        (previous, next),
        (ExecutionState::Created, ExecutionState::Validated)
            | (ExecutionState::Created, ExecutionState::FailedClosed)
            | (ExecutionState::Validated, ExecutionState::BundleVerified)
            | (ExecutionState::Validated, ExecutionState::FailedClosed)
            | (
                ExecutionState::BundleVerified,
                ExecutionState::PolicyEvaluated
            )
            | (ExecutionState::PolicyEvaluated, ExecutionState::Authorized)
            | (
                ExecutionState::PolicyEvaluated,
                ExecutionState::FailedClosed
            )
            | (ExecutionState::Authorized, ExecutionState::Dispatching)
            | (ExecutionState::Authorized, ExecutionState::FailedClosed)
            | (ExecutionState::Dispatching, ExecutionState::Executed)
            | (ExecutionState::Dispatching, ExecutionState::FailedClosed)
            | (ExecutionState::Executed, ExecutionState::Audited)
            | (ExecutionState::Executed, ExecutionState::AuditFailed)
            | (ExecutionState::Audited, ExecutionState::Completed)
    )
}
