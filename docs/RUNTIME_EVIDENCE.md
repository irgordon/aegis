# AEGIS
# Runtime Evidence v1.0

## Purpose

This document defines the evidence AEGIS must produce during runtime so reviewers can reconstruct what happened without relying on memory, chat history, or unstored orchestration context.

## Evidence Principle

AEGIS proves behavior through durable evidence.

A claim that an action was safe, approved, denied, replayed, or policy-compliant is insufficient unless evidence links the claim to execution identity, policy provenance, state, and outcome.

## Evidence Categories

Runtime evidence includes:

- request evidence
- validation evidence
- policy decision evidence
- approval evidence
- wrapper evidence
- execution evidence
- state transition evidence
- replay evidence
- audit evidence
- policy activation evidence

## Request Evidence

Request evidence should identify:

- request ID
- execution ID
- run ID where available
- tool name
- safe parameter representation
- actor identity where available
- receiving gateway
- timestamp

## Validation Evidence

Validation evidence should identify:

- schema version
- validation result
- validation error code where applicable
- compatibility result where applicable

## Policy Decision Evidence

Policy decision evidence should identify:

- decision
- reason code
- capability class
- policy bundle ID
- policy version
- policy hash
- signer identity where available
- rule identifier where available

## Approval Evidence

Approval evidence should identify:

- approval ID
- approver identity
- action identity
- approval scope
- approval timestamp
- approval result
- expiration or stale-state checks

## Wrapper Evidence

Wrapper evidence should identify:

- wrapper name
- wrapper version where available
- enforcement result
- credential scope reference where safe
- failure reason where applicable

## Execution Evidence

Execution evidence should identify:

- external system target
- execution status
- safe result summary
- failure reason where applicable
- idempotency key where applicable
- completion timestamp

## State Transition Evidence

State transition evidence should identify:

- previous state
- new state
- transition reason
- transition timestamp
- component responsible for the transition

## Replay Evidence

Replay evidence should identify:

- replay token
- original execution ID
- replay attempt number
- stored request reference
- pinned policy bundle
- duplicate detection result
- replay outcome

## Policy Activation Evidence

Policy activation evidence should identify:

- activated bundle ID
- previous bundle ID where applicable
- validation result
- compatibility result
- activation actor or process
- activation timestamp

## Evidence Safety

Runtime evidence must not expose secrets.

Sensitive values should be represented through safe references, redaction, hashes, or documented summaries.

## Evidence Completeness

If required evidence cannot be produced for a governed action, the action is incomplete and must fail closed where execution safety depends on the missing evidence.
