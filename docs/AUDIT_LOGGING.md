# AEGIS
# Audit Logging v1.0

## Purpose

This document defines audit evidence requirements for AEGIS.

Audit records are not ordinary logs. They are durable governance evidence used for security review, compliance, incident response, replay analysis, and operational accountability.

## Scope

Audit evidence is required for:

- request receipt
- schema validation outcome
- policy decision
- human approval decision
- wrapper execution result
- tool execution result
- denied execution
- failed execution
- pending approval
- replay attempt
- cancellation
- policy activation
- policy rollback

## Audit Principles

Audit records must be:

- structured
- attributable
- timestamped
- linked to execution identity
- linked to policy provenance
- durable
- protected from agent and orchestrator modification
- free of secrets

## Required Decision Fields

Every material decision record should include:

- audit record ID
- execution ID
- run ID where available
- action ID where available
- actor identity where available
- tool name where applicable
- decision or status
- reason code where applicable
- policy bundle ID
- policy version
- policy hash
- timestamp
- environment
- component emitting the record

## Approval Fields

Approval audit records should include:

- approval ID
- execution ID
- action ID
- approver identity
- approval decision
- approval timestamp
- approval scope
- policy provenance
- expiration or stale-state outcome where applicable

## Replay Fields

Replay audit records should include:

- replay token
- original execution ID
- replay attempt number
- stored request reference
- pinned policy bundle
- duplicate detection result
- replay outcome

## Policy Activation Fields

Policy activation audit records should include:

- bundle ID
- policy version
- policy hash
- signer identity where available
- activation timestamp
- environment
- compatibility result
- activation actor or process
- previous bundle ID where applicable

## Secret Handling

Audit records must not contain:

- API keys
- passwords
- bearer tokens
- private keys
- signing keys
- raw credentials
- unredacted authorization headers
- session cookies

Sensitive payloads should be represented by safe references, hashes, redacted fields, or documented summaries.

## Operational Logs

Operational logs may support debugging and operations, but they do not replace audit records.

Logs must not be the only location for governance evidence.

## Local Development Audit Log

The Phase 2 local gateway can save audit records to a local file.

For a new reader: this means AEGIS can now keep a local record of why the gateway allowed, denied, or paused a request. These records help explain what happened after the process exits.

This is still pre-alpha. It is not production audit infrastructure.

## JSONL Format

The local development audit log uses newline-delimited JSON.

Each completed gateway decision appends one JSON object on one line:

```text
{"event_type":"policy_decision","status":"allowed",...}
{"event_type":"policy_decision","status":"denied",...}
```

The runtime accepts:

```bash
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  schemas/examples/valid/ToolCallRequest.json
```

If `--audit-log` is omitted, stdout behavior remains unchanged.

## Append-Only Behavior

The local audit writer:

- creates the file if it is missing
- opens the file in append mode
- writes exactly one audit record for each completed gateway decision
- writes a newline after each JSON object
- flushes the write before the process exits
- does not truncate existing audit files
- does not rewrite previous records

Malformed requests, unsupported tools, denied decisions, pending decisions, and allowed decisions all produce audit records.

For the local runtime, record order follows successful append order in the target file. The current local binary processes one request per invocation.

## Local Audit Failure Behavior

If `--audit-log` is provided and the record cannot be written, the local runtime fails closed.

The runtime returns structured JSON instead of silently continuing without durable evidence.

Expected local failure cases include:

- audit path is a directory
- parent directory is missing
- file permissions prevent writing
- record serialization fails
- file write or flush fails

## Structured Error Evidence

AEGIS failures should be understandable. When the local gateway denies or fails because of validation, bundle verification, policy evaluation, wrapper dispatch, audit persistence, runtime I/O, or an unexpected internal condition, the operator-facing output includes a structured error report.

Audit records store a smaller safe subset:

- error code
- error location
- error reason
- next action

For a new reader: this means the audit record can help explain what happened and what someone should check next.

For contributors: do not copy large error objects, stack traces, dependency errors, secrets, or raw credentials into audit records. Add tests for new failure paths that verify audit evidence remains bounded and secret-free.

For engineers: audit error evidence is diagnostic metadata, not a recovery protocol. It preserves fail-closed behavior while making validation, policy, wrapper, and persistence boundaries observable enough for later UI and incident review.

## Local Wrapper Execution Evidence

The Phase 3 local runtime records wrapper execution evidence when the built-in `health.check` wrapper runs after an allowed policy decision.

Wrapper execution evidence includes:

- wrapper name
- wrapper version
- wrapper status
- wrapper execution mode
- bounded wrapper result summary

Denied and pending decisions do not include wrapper execution evidence because the wrapper is not dispatched.

For contributors: wrapper evidence belongs in audit details alongside policy evaluation evidence. Do not store raw credentials, tokens, unbounded external responses, or stack traces in wrapper result summaries.

For engineers: this is the first governed execution evidence path. It proves the gateway can connect verified policy, allowed-only wrapper dispatch, response result mapping, and durable JSONL audit persistence without adding external systems, credential injection, replay, or durable execution state.

## Execution Lifecycle Evidence

The Phase 3 local runtime includes execution lifecycle evidence in structured output and audit records.

For a new reader: this shows where a request was when AEGIS finished handling it. A successful `health.check` request reaches `completed`. A denied or invalid request reaches `failed_closed`. A request that executed but could not write requested audit evidence reaches `audit_failed`.

Lifecycle audit evidence includes:

- current execution state
- previous state where available
- ordered transitions

For contributors: lifecycle states are bounded Rust enums, not free-form strings. Add a new state only when current code reaches it, audit evidence needs it, and tests cover the transition.

For engineers: lifecycle evidence is an in-memory deterministic state trace. It is not durable execution storage, replay state, event sourcing, or recovery metadata. It gives future replay and durable execution work a stable evidence shape without implementing those behaviors in this phase.

## Execution Authorization Evidence

The Phase 3 local runtime records execution authorization evidence before wrapper dispatch.

For a new reader: this shows who or what allowed wrapper execution, which wrapper was allowed, and what scope was permitted.

Authorization audit evidence includes:

- authorization ID
- authority source
- authorized wrapper and version
- tool name
- capability class
- execution scope
- credential class reference
- authorization status

Denied and pending policy decisions do not authorize wrapper execution.

For contributors: authorization evidence belongs in audit details alongside policy evaluation and wrapper evidence. Do not record secrets, tokens, credentials, vault references, or raw approval token material.

For engineers: authorization evidence binds policy-allowed execution to wrapper dispatch without implementing credential injection, approval workflow, break-glass execution, distributed authorization, or durable execution state. Authorization mismatch failures are fail-closed and should include bounded error evidence.

## Credential Boundary Evidence

The Phase 3 local runtime records credential boundary evidence before wrapper dispatch.

For a new reader: this shows whether a wrapper needed credentials and what credential class was allowed. It does not show any real credential value.

Credential boundary evidence includes:

- whether credentials are required
- wrapper credential class
- authorized credential class
- credential boundary status
- failure reason where applicable

For contributors: credential evidence belongs in audit details alongside execution authorization. Do not record usernames, passwords, API keys, bearer tokens, certificates, vault references, secret IDs, environment variables, or credential values.

For engineers: credential boundary evidence proves class compatibility only. It prepares the future credential injection boundary without implementing secret retrieval, vault integration, external identity providers, production mutation wrappers, or durable execution state.

## Local Sandbox Mutation Evidence

The Phase 3 local runtime records mutation evidence when `sandbox.note.write` runs after all gateway gates pass.

For a new reader: this shows that AEGIS wrote a local sandbox note and why the write was allowed.

Sandbox mutation evidence includes:

- wrapper name and version
- capability class
- execution authorization evidence
- credential boundary evidence
- idempotency context evidence
- sandbox root reference
- sandbox-relative path
- mutation status

For contributors: record only bounded local development evidence. Do not record secrets, credentials, environment variables, unrestricted host paths when avoidable, or unbounded wrapper output.

For engineers: local sandbox mutation evidence proves allowed-only L1 mutation under policy, authorization, credential, idempotency, and path-containment gates. It is not replay state, durable execution state, database idempotency, production filesystem access, or external action evidence.

## Durability Assumptions

The local writer flushes process buffers before exit.

It does not claim:

- database durability
- WORM compliance
- hash chaining
- audit log signatures
- encryption at rest
- compression
- remote SIEM delivery
- syslog integration

Those controls belong to later phases.

## Contributor Validation

Audit persistence changes should run:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Immutability

Audit storage should be append-only or write-once-read-many where practical.

Agents and orchestrators must not be able to modify or delete audit records.

## Failure Behavior

If AEGIS cannot produce mandatory audit evidence for a material decision, execution must fail closed unless a documented emergency operating mode explicitly applies.

Emergency operating modes must not erase the requirement to reconstruct what happened.

## Validation

Audit validation must verify:

- required fields exist
- records are structured
- secrets are absent
- denials include reasons
- failures include reasons
- approval decisions include approver identity
- replay attempts are recorded
- policy provenance is present
