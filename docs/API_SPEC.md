# AEGIS
# API Specification v1.0

## Purpose

This document defines the governance-level API contract for AEGIS.

The exact transport may vary by implementation. The contract below defines the stable request, response, state, and error semantics that implementations must preserve.

## Scope

This specification covers:

- tool execution request submission
- tool execution response states
- approval-related references
- replay-related references
- audit and policy provenance requirements
- error behavior

## Transport Neutrality

AEGIS may expose this contract through HTTP, local IPC, message queues, or framework adapters.

Transport choices must not change the security properties, state semantics, or deterministic behavior defined here.

## Tool Execution Request

A tool execution request represents an orchestrator asking AEGIS to govern a proposed external action.

Required conceptual fields:

- request ID or action ID
- tool name
- tool parameters
- actor identity where available
- run ID where available
- task ID where available
- environment
- idempotency key where applicable
- orchestrator metadata where safe

Requests must be schema validated before policy evaluation.

Malformed requests do not execute.

## Tool Execution Response

A response must include:

- execution ID
- status
- decision where applicable
- result where execution completed
- denial or failure reason where applicable
- pending reference where applicable
- replay reference where applicable
- policy provenance
- audit reference where available

Allowed status values are bounded by the orchestrator FSM contract.

## Status Values

The canonical status values are:

- allowed
- denied
- pending
- failed
- canceled
- replayed

Unknown statuses are invalid.

## Error Semantics

Errors must be explicit and deterministic.

An externally visible gateway error should include:

- bounded error code
- severity
- plain-language message
- reason
- next action
- failure location
- execution ID where available
- request ID where available
- audit reference where available

Errors must not expose secrets.

For a new reader: an AEGIS error should say what happened, why it happened, and what to do next. The local runtime returns structured JSON errors so future user interfaces can display the same information without guessing from logs.

For contributors: new externally visible error paths should use the shared gateway error report model. Add a bounded code, a bounded location, a human-readable message, a reason, a next action, and tests that prove the output is valid JSON and secret-free.

For engineers: structured errors are part of the fail-closed boundary. They do not make failures recoverable by default, and they do not expose stack traces or raw dependency errors. Safe diagnostic fields may include request ID, execution ID, policy bundle ID, tool name, wrapper name, and source error kind.

Bounded error locations include:

- request_validation
- policy_bundle_verification
- policy_evaluation
- wrapper_dispatch
- audit_persistence
- runtime_io
- unexpected_internal

## Policy Provenance

Every policy-governed response must identify:

- policy bundle ID
- policy version
- policy hash
- environment
- signer identity where available

## Approval Semantics

A pending response must include a pending reference that binds the approval workflow to the specific action and execution state.

Approval completion must not alter request parameters.

## Replay Semantics

Replay requests or callbacks must reference stored execution state. Replay must not contain regenerated tool parameters from an LLM planning step.

Replay uncertainty fails closed.

## Schema Relationship

Schema files define machine-validatable fields for this API contract.

The governance contract is defined here. Implementations must keep schemas consistent with this document, `INVARIANTS.md`, and `ACCEPTANCE_CRITERIA.md`.

## Compatibility

Breaking API changes require:

- documented justification
- schema version update
- migration guidance
- updated tests
- updated acceptance criteria where behavior changes
