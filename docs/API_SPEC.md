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

An error response should include:

- execution ID where available
- error code
- safe error message
- terminal or retryable indicator
- audit reference where available

Errors must not expose secrets.

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
