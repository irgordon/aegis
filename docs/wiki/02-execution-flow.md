# AEGIS
# Execution Flow

## What Is This?

This page explains how a request moves through the current local AEGIS runtime.

The main idea is simple: a request must pass through every governance boundary before execution can happen.

## Signal Flow

The current and planned operator signal flow is:

```text
CLI or future Tauri UI
  -> structured request
  -> runtime local path
  -> policy bundle verification
  -> policy decision
  -> execution authorization
  -> credential boundary
  -> local credential injection boundary when required
  -> wrapper dispatch
  -> wrapper execution
  -> audit record
  -> state transition log
  -> structured JSON output
  -> future UI visual feedback
```

The CLI exists today. The Tauri UI is future work. Runtime output is already structured so a future UI can render it without inventing policy or execution state.

## Flow Diagram

```mermaid
flowchart TD
    A["CLI input or future Tauri UI intent"] --> B["Structured ToolCallRequest"]
    B --> C["Local runtime"]
    C --> D{"Request valid?"}
    D -- "No" --> X["Failed closed"]
    D -- "Yes" --> E{"Policy bundle verified?"}
    E -- "No" --> X
    E -- "Yes" --> F["Policy evaluation"]
    F --> G{"Decision"}
    G -- "Deny" --> X
    G -- "Pending approval" --> X
    G -- "Allow" --> H["Execution authorization"]
    H --> I{"Credential boundary satisfied?"}
    I -- "No" --> X
    I -- "Yes" --> Q{"Credential handle issued or not required?"}
    Q -- "No" --> X
    Q -- "Yes" --> J["Wrapper dispatch"]
    J --> K{"Wrapper executed?"}
    K -- "No" --> X
    K -- "Yes" --> L["Audit evidence"]
    L --> M{"Audit persisted when requested?"}
    M -- "No" --> Y["Audit failed"]
    M -- "Yes" --> Z["Completed"]
    X --> S["Structured error report"]
    Y --> S
    Z --> T["Structured JSON output"]
    S --> T
    T --> U["Future UI visual feedback"]
    V["Optional state log evidence"] --> W["Read-only recovery inspection"]
    W --> N["Read-only recovery plan classification"]
    N --> U
    C --> V
```

## Successful Execution

A successful local execution follows this pattern:

```text
Created
  -> Validated
  -> BundleVerified
  -> PolicyEvaluated
  -> Authorized
  -> Dispatching
  -> Executed
  -> Audited
  -> Completed
```

For `health.check`, the wrapper returns a safe local health result.

For `sandbox.note.write`, the wrapper writes only inside a caller-supplied sandbox directory after mutation gates pass.

## Fail-Closed Execution

AEGIS fails closed when it cannot prove execution should continue.

Examples:

- malformed request
- unsupported tool
- invalid policy bundle
- checksum mismatch
- signature verification failure
- denied policy decision
- pending approval decision
- authorization mismatch
- credential boundary failure
- wrapper dispatch failure
- wrapper execution failure

Failing closed means the runtime denies or stops the action instead of guessing.

## Audit-Failed Path

Audit persistence is optional.

When an audit log path is supplied, AEGIS must append the audit record and flush the write. If audit persistence fails after execution, the runtime reports `audit_failed` instead of pretending the request completed normally.

## Recovery Inspection and Planning

Recovery inspection reads local execution state logs.

Recovery planning uses inspection output to classify what a future recovery system may be allowed to consider. It does not replay work, resume execution, or mutate state.

The current planner can classify records into bounded outcomes such as:

- not recoverable because the execution already reached a terminal state
- not recoverable because the state evidence is corrupted
- candidate for future audit retry
- candidate for future replay evaluation
- inspection failed

## Future UI Output

The future Tauri UI should render the same evidence that the CLI emits today:

- lifecycle timeline
- policy bundle status
- policy decision
- authorization status
- credential boundary and injection status
- wrapper execution status
- audit persistence status
- state log and recovery classifications
- structured error message, reason, next action, and location

The UI must not decide policy, authorize execution, or invent lifecycle state.
