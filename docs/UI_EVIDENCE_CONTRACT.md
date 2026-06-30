# UI Evidence Contract

## Purpose

This document defines how the future AEGIS Tauri desktop shell with Slint graphical UI layer should consume and display backend evidence.

The UI exists to help operators understand governed execution. It should show what happened, why it happened, where it stopped, and what evidence supports that result.

The UI must not become a second policy engine, state machine, recovery engine, wrapper dispatcher, credential issuer, or error normalizer.

AEGIS will use Tauri as the desktop application shell and Slint as the graphical UI layer when Phase 4 UI implementation begins.

Core rule:

```text
The UI renders backend evidence. It does not recreate decisions.
```

## Authority Boundary

The UI is not an authority boundary.

All authoritative decisions come from backend runtime evidence.

The UI must not:

- evaluate policy
- authorize execution
- issue credentials
- dispatch wrappers
- classify recovery
- decide replay eligibility
- rewrite audit evidence
- rewrite state evidence
- normalize errors independently
- approve execution
- reinterpret failed-closed behavior
- treat recovery plans as recovery approval
- display hidden execution paths that do not exist in backend evidence

The backend is authoritative.

The UI is an operator surface.

The backend remains authoritative for validation, policy evaluation, execution authorization, credential handling, wrapper execution, audit evidence, state evidence, recovery inspection, and recovery planning.

The UI is an operator surface, not an authority boundary.

## Evidence Sources

Future UI components should consume current backend evidence sources.

| Evidence source | What it represents | Where it comes from | UI may display | UI must not infer |
| --- | --- | --- | --- | --- |
| `response` | Gateway response for the request | `LocalRuntimeOutput.response` | request ID, execution ID, status, decision, safe message, result summary, policy provenance | that denied or pending requests may execute |
| `audit_record` | Decision and execution evidence | `LocalRuntimeOutput.audit_record` or audit JSONL | audit status, audit ID, decision evidence, policy evidence, wrapper evidence, error evidence | lifecycle state or replay eligibility beyond recorded evidence |
| `policy_bundle` | Local policy bundle verification evidence | `PolicyBundleVerification` | bundle ID, versions, checksum status, signature status, verification status | production PKI, remote trust, or policy success when verification failed |
| `policy_evaluation` | Local policy/risk evaluation result | `PolicyEvaluation` | rule ID, risk entry, decision, reason, failure reason | a new policy decision |
| `execution_authorization` | Authority for allowed wrapper execution | `ExecutionAuthorization` | authorization ID, wrapper, version, scope, credential class, status | broader scope, different wrapper, or execution permission when absent |
| `credential_boundary` | Credential class compatibility check | `CredentialBoundary` | required flag, credential class, authorized class, boundary status | credential values or secret availability |
| `credential_injection` | Safe local credential handle reference when required | `CredentialInjectionResult` | source, class, handle reference, wrapper binding, status | real secret material or production credential delivery |
| `execution_lifecycle` | In-memory runtime lifecycle evidence | `ExecutionLifecycle` | current state and ordered transitions | missing transitions or an independent lifecycle |
| `wrapper_execution` | Bounded wrapper execution evidence | `WrapperExecutionEvidence` | wrapper name, version, status, mode, safe result summary | the ability to rerun or reconstruct wrapper input |
| state log records | Persisted lifecycle transitions | `ExecutionStateLogRecord` JSONL | timeline entries, lifecycle index, transition reason, safe references | audit evidence, replay state, or repaired transitions |
| recovery inspection report | Read-only state log inspection | `ExecutionRecoveryReport` | inspection status, last known state, terminal status, recoverability, inspection errors | replay permission or state repair |
| recovery plan report | Read-only future recovery classification | `RecoveryPlanReport` | plan outcome, allowed future action, plan reason, planning errors | recovery approval, replay execution, or audit retry execution |
| structured error report | Normalized operator-facing failure report | `GatewayErrorReport`, `MalformedStateRecord`, `RecoveryPlanningError` | code, severity, message, reason, next action, location | new error categories or different severity |

## Execution Timeline Model

The UI timeline should be built from backend evidence. It should not invent stages that the backend did not report.

| Stage | Display label | Evidence source | Status values | Plain-language meaning | Operator next action |
| --- | --- | --- | --- | --- | --- |
| Request | Request | `response`, `audit_record.details.request_id` | present, missing | A request was or was not recognized well enough to track. | If missing, inspect the structured error before retrying. |
| Validation | Validation | `response.status`, `error_report.location`, `execution_lifecycle` | `allowed`, `denied`, `pending`, `failed`, `failed_closed` mapping | The request was accepted for policy evaluation or stopped at validation. | Fix malformed or unsupported request data when validation failed. |
| Policy Bundle Verification | Policy bundle | `policy_bundle.verification_status` | `verified`, `rejected` | The local policy bundle was trusted or rejected. | Fix bundle files, checksums, or signature when rejected. |
| Policy Evaluation | Policy | `policy_evaluation.evaluation_status`, `policy_evaluation.decision` | `evaluated`, `failed_closed`, `allow`, `deny`, `pending_approval` | Verified policy allowed, denied, paused, or failed closed. | Do not reinterpret the decision. Review policy evidence when denied or failed closed. |
| Execution Authorization | Authorization | `execution_authorization.authorization_status` | `authorized`, `denied`, absent | Allowed execution received explicit wrapper authority, or it did not. | Investigate authorization evidence before assuming execution was permitted. |
| Credential Class Boundary | Credential boundary | `credential_boundary.credential_boundary_status` | `satisfied`, `denied`, absent | Wrapper credential requirements matched authorization, failed, or were not reached. | Fix wrapper requirement or authorization class when denied. |
| Credential Injection | Credential handle | `credential_injection.credential_injection_status` | `injected`, `denied`, absent or not required | A safe local handle reference was issued only when required, denied, or not needed. | Do not ask the UI to find or create credentials. |
| Wrapper Dispatch | Wrapper dispatch | `wrapper_execution`, `error_report.location` | wrapper evidence present, wrapper error, absent | The wrapper was selected and executed, failed, or was never reached. | Use wrapper error evidence when dispatch failed. |
| Wrapper Execution | Wrapper execution | `wrapper_execution.wrapper_status` | `executed`, `observed`, `dry_run`, absent | The wrapper completed in its reported execution mode, or no wrapper ran. | Do not rerun wrappers from UI output. |
| Audit | Audit | `audit_record`, `error_report.location`, `execution_lifecycle.execution_state` | audit record present, `audit_failed`, audit error | Audit evidence exists or requested audit persistence failed. | Fix audit persistence before treating execution as complete when audit failed. |
| State Log | State log | state log records, `execution_lifecycle` | state records present, state write error, absent | Lifecycle transitions were optionally persisted or only returned in runtime output. | Do not treat the state log as the audit log. |
| Recovery Inspection | Recovery inspection | `ExecutionRecoveryReport` | `inspected`, `inspection_failed` | State evidence was inspected or could not be trusted. | Preserve evidence and review inspection errors when failed. |
| Recovery Plan | Recovery plan | `RecoveryPlanReport` | `planned`, `inspection_failed`; plan outcomes listed below | The backend classified future recovery considerations. | Do not convert plan output into recovery or replay action. |

Some timeline labels are visual mappings. They do not create new backend statuses.

## Status Card Model

A future status card should use this general shape:

| Field | Meaning | Requirement |
| --- | --- | --- |
| `title` | Plain label for the evidence area | UI display field |
| `status` | Backend status or visual mapping from backend evidence | must preserve backend meaning |
| `severity` | Backend severity when available, or a visual priority derived without changing meaning | must not downgrade fail-closed evidence |
| `summary` | Short plain-language statement | should use backend `message` or safe evidence summary |
| `reason` | Why the status occurred | should use backend `reason`, `decision_reason`, or failure reason |
| `next_action` | What the operator should do next | should use backend `next_action` when available |
| `evidence_source` | Source object or field name | required for traceability |
| `details_available` | Whether the user can inspect more backend evidence | UI display field |
| `timestamp` | Time associated with the evidence | only display when supplied by backend evidence |

The UI must not generate new runtime IDs, timestamps, policy decisions, or recovery decisions.

## Policy Evidence

The UI should show:

- policy bundle verification status
- checksum verification status
- signature verification status
- signature algorithm when present
- signed artifact when present
- bundle ID
- policy version
- risk matrix version
- policy rule ID
- policy decision
- risk matrix entry ID
- decision reason
- policy message where supplied through safe response or decision evidence
- approval requirement or pending reference when present

Current backend status values include:

- `PolicyBundleVerificationStatus`: `verified`, `rejected`
- `ChecksumVerificationStatus`: `verified`, `metadata_missing`, `entry_missing`, `mismatch`, `malformed_metadata`, `file_read_failed`
- `SignatureVerificationStatus`: `signature_verified`, `public_key_missing`, `public_key_malformed`, `signature_missing`, `signature_malformed`, `signed_artifact_read_failed`, `signed_content_mismatch`
- `PolicyEvaluationStatus`: `evaluated`, `failed_closed`
- `ResponseDecision`: `allow`, `deny`, `pending_approval`

The UI must not:

- evaluate policy locally
- edit policy results
- reinterpret pending as allowed
- reinterpret denied as recoverable
- hide failed bundle verification
- claim production PKI or remote trust distribution from local bundle evidence

## Authorization Evidence

The UI should show:

- authorization status
- authorization ID or reference when present
- authorized wrapper
- authorized wrapper version
- tool name
- capability class
- authorized credential class
- execution scope
- authority source
- failure reason when present

Current backend status values include:

- `AuthorizationStatus`: `authorized`, `denied`
- `ExecutionAuthority`: `policy_allow`, `development_fixture`
- `ExecutionScope`: `local_gateway_health`, `local_sandbox_note_write`

The UI must not:

- authorize execution
- widen execution scope
- change wrapper identity
- change wrapper version
- change credential class
- create authorization when backend evidence is absent

## Credential Boundary Evidence

The UI should show:

- whether credentials are required
- required credential class
- authorized credential class
- credential boundary status
- failure reason when present

Current backend status values include:

- `CredentialClass`: `none`, `local_runtime`
- `CredentialBoundaryStatus`: `satisfied`, `denied`
- `CredentialBoundaryFailureReason`: `credential_class_missing`, `credential_class_mismatch`, `credential_boundary_denied`, `credentials_required_without_authorization`

The credential boundary authorizes classes only. It does not prove that a real secret exists.

The UI must not:

- display credential values
- assume a missing credential boundary is success
- allow a wrapper to request a broader credential class
- treat `local_runtime` as a production credential

## Credential Injection Evidence

The UI should show:

- credential required flag
- credential class
- credential source
- credential handle reference, only if safe and supplied by backend evidence
- wrapper name
- wrapper version
- authorization ID
- credential injection status
- failure reason when present

Current backend status values include:

- `CredentialSource`: `local_development`
- `CredentialInjectionStatus`: `injected`, `denied`
- `CredentialInjectionFailureReason`: `credential_handle_missing`, `credential_class_unsupported`, `credential_handle_wrapper_mismatch`, `credential_handle_authorization_mismatch`, `credential_injection_denied`, `credential_injection_unavailable`

Credential handle references are not secrets.

The current local handle is development-safe and not a production credential.

The UI must never display:

- tokens
- passwords
- private keys
- environment values
- secret file paths
- raw credential payloads
- API keys
- certificate contents

## Wrapper Execution Evidence

The UI should show:

- wrapper name
- wrapper version
- wrapper status
- wrapper execution mode
- wrapper output summary
- wrapper error summary through structured error evidence
- mutation indicator when safe evidence shows mutation
- artifact or target summary when safe

Current backend status values include:

- `WrapperExecutionStatus`: `executed`, `observed`, `dry_run`
- `WrapperExecutionMode`: `observe_only`, `enforce`, `dry_run`

For `health.check`, the UI may display a simple health status from safe wrapper output.

For `sandbox.note.write`, the UI may display that a sandbox note was written and show safe sandbox-relative evidence. It must not expose unsafe filesystem assumptions, broad host paths, or wrapper arguments as executable instructions.

The UI must not:

- execute wrappers directly
- construct wrapper inputs
- rerun wrappers from displayed output
- display unsafe mutation details
- treat wrapper result output as policy evidence

## Audit Evidence

The UI should show:

- audit status
- audit record existence
- audit record ID
- audit write success or failure when evidence is available
- audit failure reason when safe
- audit evidence availability
- decision evidence from audit details
- safe error evidence from audit details

Current audit status values include:

- `allowed`
- `denied`
- `pending`
- `failed`
- `canceled`
- `replayed`
- `recorded`

Audit evidence explains what happened and why.

Audit evidence is separate from lifecycle state.

The UI must not rewrite audit records, merge audit records into state records, or treat operational logs as audit evidence.

## State Log Evidence

The UI should show:

- last known lifecycle state
- transition count
- state log write status when available
- state timeline
- lifecycle index
- transition reason
- safe references such as request ID, tool name, wrapper name, wrapper version, authorization ID, credential boundary status, credential injection status, credential class, credential handle reference, and idempotency key reference

Current lifecycle states include:

- `created`
- `validated`
- `bundle_verified`
- `policy_evaluated`
- `authorized`
- `dispatching`
- `executed`
- `audited`
- `completed`
- `failed_closed`
- `audit_failed`

The state log explains where execution moved.

The state log is not the audit log.

The UI must not:

- invent missing transitions
- repair state logs
- hide terminal failures
- create a second lifecycle vocabulary
- treat state evidence as replay execution

## Recovery Inspection Evidence

The UI should show:

- inspection status
- execution ID
- last known state
- terminal status
- recoverability status
- transition count
- inspection errors

Current inspection values include:

- `ExecutionRecoveryStatus`: `inspected`, `inspection_failed`
- `ExecutionTerminalStatus`: `terminal`, `non_terminal`
- `ExecutionRecoverability`: `not_recoverable_terminal`, `recoverable_candidate`, `inspection_failed`, `unknown`

Recovery inspection is read-only.

Inspection does not replay or resume execution.

Inspection errors should be shown as evidence problems, not runtime execution failures.

The UI must not repair state, resume execution, or hide corrupted evidence.

## Recovery Plan Evidence

The UI should show:

- plan status
- plan outcome
- allowed future action
- plan reason
- planning errors
- source inspection status
- last known state
- recoverability status

Current plan statuses include:

- `planned`
- `inspection_failed`

Current bounded plan outcomes include:

- `not_recoverable_terminal`
- `not_recoverable_corrupted`
- `candidate_for_audit_retry`
- `candidate_for_future_replay`
- `inspection_failed`

Current bounded future actions include:

- `none`
- `audit_retry_only`
- `future_replay_evaluation_only`
- `manual_review_only`

Recovery plans do not approve execution.

Recovery plans do not replay.

Recovery plans only classify what future recovery work may consider.

The UI must not convert `candidate_for_future_replay` into a replay button.

## Error Card Model

The UI should render normalized backend errors with this shape:

| Field | UI rendering rule |
| --- | --- |
| `message` | Card title or summary |
| `reason` | Plain-language explanation |
| `next_action` | Operator guidance |
| `severity` | Visual priority |
| `location` | Source area |
| `code` | Technical reference for support and debugging |

The UI must display plain-language `message`, `reason`, and `next_action` before raw technical details.

The UI must not display raw internal parser errors, raw malformed input, request payloads, wrapper arguments, or secret-like material.

## Error Logging and Error Normalization

The UI must rely on backend-normalized errors.

The UI must not maintain its own independent error taxonomy.

The UI may group or style errors visually, but it must preserve:

- `code`
- `severity`
- `message`
- `reason`
- `next_action`
- `location`

Current `ErrorSeverity` values are:

- `warning`
- `error`
- `critical`

Current `ErrorLocation` values include:

- `request_validation`
- `policy_bundle_verification`
- `policy_evaluation`
- `execution_authorization`
- `credential_boundary`
- `credential_injection`
- `sandbox_mutation`
- `execution_state_log`
- `execution_recovery_inspection`
- `recovery_plan_generation`
- `wrapper_dispatch`
- `audit_persistence`
- `runtime_io`
- `unexpected_internal`

The UI must not:

- change error meaning
- downgrade severity
- hide fail-closed errors
- turn inspection errors into execution errors
- turn planning errors into recovery approval

## Redaction and Secret Safety

The UI must never display:

- tokens
- passwords
- API keys
- private keys
- authorization token material
- raw credential values
- environment variable values
- secret file contents
- raw malformed lines containing secret-like material
- request payloads that have not been redacted by backend evidence
- wrapper arguments that could be reused as executable instructions

The UI may display safe references only when backend evidence already supplies them as safe evidence.

Examples:

- `credential_handle_ref`
- `policy_rule_id`
- `execution_id`
- `request_id`
- `wrapper_name`
- `wrapper_version`
- `authorization_id`
- `policy_bundle_id`

Do not include fake secrets in examples, screenshots, tests, or mock data.

## Visual Status Language

Use plain-language labels.

Preferred labels include:

- Allowed
- Denied
- Pending
- Authorized
- Blocked
- Credential required
- Credential not required
- Credential satisfied
- Credential failed
- Credential handle issued
- Credential handle not required
- Executed
- Not executed
- Audit recorded
- Audit failed
- State recorded
- Failed closed
- Inspection failed
- Plan available
- Not recoverable
- Future evaluation only
- Manual review only

Avoid labels that imply automatic recovery, hidden execution, or production readiness.

Do not use labels such as:

- Self-healed
- Auto-recovered
- Safe to replay
- Approved to rerun
- Credential granted
- Production ready

## Future Tauri Integration Notes

Tauri will provide the future graphical desktop shell.

Slint will provide the future graphical UI layer.

The backend remains authoritative.

The evidence contract is UI-framework independent at the data layer.

Slint components should render the same backend evidence fields defined in this contract.

Future IPC commands should return the same structured evidence model or a deliberate versioned projection of it.

Tauri integration must preserve the backend evidence model rather than creating a separate frontend state machine.

UI components should be built around evidence cards and execution timelines.

Do not define IPC commands here.

Do not scaffold Tauri here.

Do not scaffold Slint here.

Do not select frontend libraries beyond existing documented direction.

Do not implement UI here.

## Non-Goals

This document does not implement or define:

- Tauri scaffold
- Slint scaffold
- Rust UI integration
- Slint markup files
- UI components
- CSS
- icons
- screenshots
- design system
- IPC commands
- HTTP API
- web server
- frontend state management
- authentication UI
- approval UI
- replay UI
- recovery execution UI
- dashboard implementation
- TUI
- terminal dashboard
- CLI redesign
- backend runtime changes
- policy changes
- authorization changes
- credential changes
- wrapper changes
- audit changes
- state changes
- recovery changes
- error model changes
- schemas
- database
- external integrations
- new architecture
- new invariants
