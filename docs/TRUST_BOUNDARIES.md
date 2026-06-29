# AEGIS
# Trust Boundaries v1.0

## Purpose

This document defines the trust boundaries that every AEGIS implementation must preserve.

A trust boundary is a place where data, authority, or responsibility crosses from one security context to another. Boundary crossings must be explicit, validated, auditable, and fail closed when unsafe.

## Boundary Summary

AEGIS defines these primary boundaries:

1. AI Orchestrator to Gateway
2. Gateway to Policy Engine
3. Gateway to Security Wrappers
4. Gateway and Wrappers to External Systems
5. Gateway to Durable State
6. Gateway to Audit Store
7. Policy Registry to Gateway
8. Human Approver to Approval Workflow

## Boundary 1: AI Orchestrator to Gateway

The orchestrator is trusted to request actions. It is not trusted to authorize actions.

Crossing data:

- tool name
- tool parameters
- actor or run context
- requested action identity where supplied
- orchestration metadata

Required controls:

- schema validation
- bounded status and enum values
- request identity assignment or verification
- rejection of malformed requests
- denial of unknown tools

## Boundary 2: Gateway to Policy Engine

The gateway asks the policy engine whether a requested action may proceed.

The policy engine is trusted to decide. It is not trusted or permitted to execute tools, mutate state, or write audit records as part of decision evaluation.

Crossing data:

- validated request
- execution context
- policy bundle
- relevant state references

Required controls:

- side-effect-free evaluation
- deterministic output
- explicit allow, deny, or pending result
- policy provenance capture

## Boundary 3: Gateway to Security Wrappers

Wrappers enforce decisions and execution controls.

Wrappers do not invent policy and do not become the source of authorization truth.

Crossing data:

- policy decision
- validated request
- execution identity
- credential references
- approval evidence where applicable

Required controls:

- fail-closed wrapper behavior
- scoped credential injection
- no secret exposure to agents
- wrapper result auditability

### Execution Authorization Boundary

Execution authorization sits between policy evaluation and wrapper dispatch.

For a new reader: policy answers whether AEGIS may proceed. Execution authorization records the exact wrapper, version, capability, and scope that may run.

For contributors: wrapper execution must receive an explicit authorization context. Denied and pending decisions do not create authorization and must not dispatch wrappers. Authorization mismatches fail closed with structured error reports.

For engineers: wrappers verify authorization binding before execution. Wrappers do not evaluate policy, fetch credentials, widen scope, or authorize themselves. Current Phase 3 authorization supports policy-allow execution for the built-in local wrapper only; human approval, break-glass authority, credential injection, vault access, distributed authorization, and production identity providers remain out of scope.

### Credential Class Boundary

Credential class validation sits between execution authorization and wrapper dispatch.

For a new reader: credential classes describe what kind of credentials a wrapper may eventually receive. They do not contain usernames, passwords, tokens, or secret values.

For contributors: every wrapper must explicitly declare whether it requires credentials and which credential class it requires. The current `health.check` wrapper declares no credentials. The local `sandbox.note.write` wrapper requires the `LocalRuntime` credential class. Credential mismatches fail closed before wrapper execution.

For engineers: the credential boundary validates wrapper requirements against execution authorization. It authorizes categories only. It does not retrieve secrets, load environment variables, call vaults, or create runtime identity provider integrations.

### Local Credential Injection Boundary

The local credential injection boundary sits between credential class validation and wrapper dispatch.

For a new reader: AEGIS can now pass a safe local credential handle to a wrapper that requires local execution authority. This is not a real secret or production credential.

For contributors: the local handle is modeled in `src/auth/credential.rs` and passed through wrapper execution only after authorization and credential class validation succeed. `sandbox.note.write` receives a `LocalRuntime` handle. `health.check` receives no handle. Missing or mismatched handles fail closed.

For engineers: credential handles are bounded references, not credential values. The current source is `LocalDevelopment` only. The boundary records handle reference, class, source, wrapper binding, and authorization binding without adding vaults, environment secrets, provider traits, cloud identity, or token generation.

### Local Credential Injection Boundary

Credential injection sits after credential class validation and before wrapper dispatch.

For a new reader: AEGIS can now pass a safe local credential handle to a wrapper that requires local runtime authority. This handle is not a password, token, API key, or production credential.

For contributors: local credential handles are modeled in `src/auth/credential.rs` and passed through wrapper dispatch. The current local source is `LocalDevelopment` only. Tests must cover missing handles, wrapper binding mismatch, authorization binding mismatch, and secret-free output.

For engineers: credential handles are bounded references, not secret values. The handle is bound to the wrapper name, wrapper version, credential class, and execution authorization. This prepares future vault or identity integration without adding secret retrieval, environment loading, cloud identity, production providers, or external calls.

### Local Built-In Wrapper Execution

Phase 3 begins wrapper execution with one built-in local L0 wrapper: `health.check`.

For a new reader: AEGIS can now run one safe local health check after verified policy allows it. This does not mean AEGIS can execute real external actions yet.

For contributors: `health.check` is registered in the local Rust runtime, allowed by the local development policy bundle, and exercised by the health check wrapper tests. Policy changes to the local bundle require regenerating checksums and signatures with `scripts/regenerate-local-policy-signature.sh`.

For engineers: the allowed-only execution rule is strict. Denied and pending decisions do not dispatch wrappers. Missing wrappers, version mismatches, and wrapper execution errors fail closed with structured error reports and audit evidence. The built-in health check does not use credentials, network access, subprocesses, shell execution, filesystem writes, approval workflow, replay, or durable execution state.

### Local Sandbox Mutation Boundary

Phase 3 includes one local L1 mutation wrapper: `sandbox.note.write`.

For a new reader: AEGIS can now write one local sandbox note after verified policy allows it. The write is limited to a caller-supplied sandbox directory. AEGIS still cannot execute real external actions.

For contributors: `sandbox.note.write` must pass policy allow, execution authorization, credential class validation, idempotency context validation, sandbox directory validation, and path containment checks before it writes. It writes only to `notes/<note_id>.txt` under the sandbox root. Policy fixture changes require regenerating checksums and signatures with `scripts/regenerate-local-policy-signature.sh`.

For engineers: this boundary proves controlled local mutation without broad filesystem authority. The wrapper rejects missing sandbox roots, missing idempotency context, unsafe note identifiers, path traversal, empty content, and credential class mismatch. It does not perform network access, shell execution, subprocess execution, credential injection, approval workflow, replay, durable execution state, or production filesystem behavior.

## Boundary 4: Gateway and Wrappers to External Systems

External systems include APIs, cloud providers, databases, source control, ticketing systems, email, deployment systems, and enterprise applications.

Crossing data:

- approved tool invocation
- scoped credentials
- external response
- error or failure metadata

Required controls:

- execution only after authorization
- least-privilege credentials
- bounded retries
- explicit result mapping
- audit of material outcomes

## Boundary 5: Gateway to Durable State

Durable state preserves execution continuity and replay safety.

Crossing data:

- execution state
- pending approval state
- terminal state
- replay token
- idempotency metadata

Required controls:

- durable writes before relying on state
- explicit state transitions
- duplicate execution detection
- recovery behavior that preserves invariants

## Boundary 6: Gateway to Audit Store

Audit storage preserves evidence for later review.

Crossing data:

- request identity
- actor identity
- policy decision
- policy provenance
- approval metadata
- execution result
- failure reason where applicable

Required controls:

- immutable or append-only storage where possible
- no secret values
- records for allowed, denied, pending, failed, replayed, canceled, and error outcomes
- protection from agent and orchestrator modification

## Boundary 7: Policy Registry to Gateway

Policy registries distribute candidate policy bundles. The gateway enforces only activated local immutable bundles.

Crossing data:

- policy bundle
- manifest
- checksums
- signatures
- compatibility metadata

Required controls:

- validation before activation
- explicit activation event
- no silent live registry enforcement
- rollback provenance

## Boundary 8: Human Approver to Approval Workflow

Human approvers provide decisions for actions that require review.

Crossing data:

- approval decision
- approver identity
- timestamp
- approval scope
- action identity

Required controls:

- action-specific approval binding
- stale approval rejection
- denial enforcement
- audit of approval decisions

## Boundary Review Rule

Any implementation change that moves data or authority across one of these boundaries must update the relevant documentation, tests, schemas, and acceptance criteria before it is considered complete.
