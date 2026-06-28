# AEGIS
# Test Strategy v1.0

## Purpose

This document defines the test strategy for AEGIS.

Testing exists to prove that implementation preserves governance, architecture, invariants, security, determinism, replay, and auditability.

## Scope

The test strategy covers:

- documentation validation
- schema validation
- policy validation
- unit tests
- integration tests
- security tests
- replay tests
- audit tests
- deployment validation
- release validation

## Test Principles

AEGIS tests must:

- prove behavior, not intent
- include negative paths
- verify fail-closed behavior
- avoid hidden external dependencies where practical
- preserve deterministic expectations
- validate evidence generation
- test contracts before runtime behavior depends on them

## Documentation Tests

Documentation checks should verify:

- required documents exist
- README links resolve
- documentation hierarchy is consistent
- roadmap and phasemap align
- terminology is consistent
- docs do not contradict invariants

## Schema Tests

Schema checks should verify:

- schemas parse successfully
- required fields are enforced
- valid examples pass
- invalid examples fail
- enum values are bounded
- compatibility fields are present where required

## Policy Tests

Policy tests should verify:

- known safe action allows when policy permits
- unknown tools deny by default
- L2 and L3 actions route to pending approval
- malformed policy fails closed
- incompatible policy fails closed
- policy decisions include provenance
- repeated evaluation is deterministic

## Unit Tests

Unit tests should cover isolated behavior for:

- request validation
- policy evaluation
- response mapping
- audit record construction
- state transitions
- approval binding
- duplicate execution detection
- wrapper failure handling

## Integration Tests

Integration tests should cover component interaction:

- orchestrator request reaches gateway
- gateway invokes policy engine
- policy decision controls wrappers
- wrappers control execution
- audit evidence is emitted
- allowed, denied, and pending paths are represented

## Security Tests

Security tests should verify:

- agents do not receive long-lived credentials
- secrets are not logged
- invalid approvals are rejected
- stale approvals are rejected
- malformed requests do not execute
- invalid bundles do not activate
- wrapper failures stop execution

## Replay Tests

Replay tests should verify:

- replay uses stored request and state
- replay does not call the planning layer
- duplicate replay is detected
- approved action identity is preserved
- terminal state is durable
- replay audit evidence is emitted

## Audit Tests

Audit tests should verify:

- every material decision emits audit evidence
- audit records include execution identity
- audit records include policy provenance
- denials and failures include reasons
- approval records include approver identity
- audit records do not contain secrets

## Validation Commands

As implementation expands, validation should include the applicable commands below:

```text
markdownlint docs/
jsonschema schemas/
pytest
ruff check
mypy
cargo test
go test
npm test
```

Only commands relevant to the current implementation language and repository contents are required at each phase.

## Phase 0 Validation

Before implementation begins, Phase 0 validation requires:

- all governance documents exist
- required directories exist
- README documentation links resolve
- roadmap and phasemap are aligned enough to sequence implementation
- TASKS.md reflects current work state
