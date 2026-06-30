# AEGIS
# Major Components

## What Is This?

This page maps the current repository into the major runtime components that make AEGIS work.

It is a guide, not a replacement for source code or architecture documents.

## Component Map

| Component | Purpose | Main Inputs | Main Outputs |
| --- | --- | --- | --- |
| CLI entrypoint | Accept local commands and request paths | command arguments, stdin or file path | structured JSON output |
| Local runtime | Coordinate the governed request path | request JSON, bundle path, optional audit/state/sandbox paths | runtime output, audit evidence, state evidence |
| Gateway models | Represent request and response contracts | schema-backed request data | bounded response data |
| Validation pipeline | Reject malformed or unsupported requests | raw request JSON | validated request or fail-closed response |
| Policy bundle loader | Load and verify local policy bundle identity | bundle directory | verified bundle reference or structured failure |
| Policy evaluator | Evaluate verified local policy and risk matrix | validated request, verified bundle | bounded policy decision |
| Execution authorization | Define authority under which execution may proceed | allowed policy decision, request, response metadata | execution authorization evidence |
| Credential boundary | Check whether wrapper credential requirements are permitted | wrapper requirement, authorization | credential boundary evidence |
| Local credential injection | Provide safe local credential handle references when a wrapper requires and is allowed to use one | satisfied credential boundary, authorization | optional non-secret credential handle reference |
| Wrapper dispatcher | Find and run a registered wrapper | wrapper name, version, request, authorization, credential boundary, optional credential handle context | wrapper execution result or structured failure |
| Built-in wrappers | Perform governed local work | wrapper execution context | bounded wrapper result |
| Audit builder and writer | Build and optionally persist audit evidence | request, response, policy, auth, wrapper, error data | audit record and optional JSONL append |
| Execution lifecycle | Track deterministic state transitions | runtime events | lifecycle evidence |
| State writer | Optionally persist lifecycle transitions | lifecycle transitions | JSONL state log |
| Recovery inspector | Read state logs and classify executions | state JSONL records | inspection report |
| Recovery planner | Convert inspection report into bounded read-only future recovery guidance | inspection report | recovery plan report |
| Structured errors | Explain failures safely | bounded error sources | code, severity, message, reason, next action, location |

## Important Source Areas

| Path | Responsibility |
| --- | --- |
| `src/main.rs` | CLI mode selection and local command wiring |
| `src/runtime/local.rs` | Local governed runtime path |
| `src/gateway/` | Request, response, validation, entrypoint, and wrapper dispatch contracts |
| `src/policy/` | Policy bundle verification and local policy evaluation |
| `src/auth/` | Execution authorization, credential class, and local credential handle boundaries |
| `src/wrappers/` | Built-in local wrappers |
| `src/audit/` | Audit record building and optional JSONL persistence |
| `src/state/` | Lifecycle, state logging, recovery inspection, and recovery planning |
| `src/error.rs` | Structured error reporting |
| `tests/` | Contract, runtime, fail-closed, wrapper, state, and recovery tests |
| `examples/policy-bundles/local-dev/` | Local signed development policy bundle |
| `schemas/` | Protocol schemas and examples |

## Important Runtime Types

| Type or group | Why it matters |
| --- | --- |
| `ToolCallRequest` | The request contract AEGIS validates before any policy or execution work |
| `ToolCallResponse` | The bounded response contract emitted by the gateway |
| `PolicyBundleVerification` | Evidence that local policy bundle structure, versions, checksums, and signature were checked |
| `PolicyDecision` | Bounded result of policy evaluation: allow, deny, or pending approval |
| `ExecutionAuthorization` | The explicit authority that permits wrapper execution after policy allows it |
| `CredentialRequirement` | The wrapper-declared credential requirement checked before wrapper execution |
| `CredentialBoundary` | The check that wrapper credential requirements match authorized credential class |
| `CredentialInjectionResult` | A safe local credential handle reference emitted only when the boundary requires and permits one |
| `WrapperDispatcher` | The runtime component that invokes registered wrappers only after authorization and credential checks |
| `WrapperExecutor` | The wrapper contract implemented by built-in local wrappers |
| `WrapperExecutionEvidence` | Evidence about wrapper dispatch and execution result |
| `AuditRecord` | Structured evidence for decisions and execution |
| `ExecutionState` | The bounded lifecycle state vocabulary used in runtime output and state logs |
| `ExecutionLifecycle` | Deterministic state progression for a request |
| `ExecutionStateLogRecord` | One persisted lifecycle transition |
| `ExecutionRecoveryInspector` | Read-only inspector that classifies local state log evidence |
| `ExecutionRecoveryReport` | Inspection output from state logs |
| `RecoveryPlanGenerator` | Read-only planner that classifies future recovery considerations without executing them |
| `RecoveryPlanReport` | Bounded future recovery guidance |
| `GatewayErrorReport` | Plain-language structured failure report |

## Layering Rule

AEGIS keeps decision-making separated from execution.

Policy evaluation decides whether execution is allowed, denied, or pending approval.

Execution authorization defines the authority for execution.

Wrappers execute only after the gateway supplies authorization, credential boundary evidence, and any required safe local credential handle reference.

Wrappers do not decide policy, authorize themselves, widen credential class, or invent lifecycle state.
