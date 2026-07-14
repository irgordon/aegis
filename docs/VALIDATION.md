# AEGIS
# Validation Strategy v1.0

## Purpose

This document defines how AEGIS proves that implementation, documentation, policy, schemas, and releases are correct enough to proceed.

Validation is the evidence layer for the project.

A feature is not complete because it exists. It is complete when its required behavior has been verified.

## Scope

Validation applies to:

- documentation
- schemas
- gateway behavior
- policy behavior
- wrappers
- audit records
- durable state
- human approval workflows
- replay behavior
- deployment artifacts
- releases

## Validation Principles

AEGIS validation follows these principles:

- prove behavior, not intent
- test negative paths
- fail closed on uncertainty
- validate contracts before runtime
- treat documentation as a source of truth
- capture evidence for review

## Validation Layers

AEGIS validation is organized into layers.

1. Documentation validation
2. Schema validation
3. Policy validation
4. Unit validation
5. Integration validation
6. Replay validation
7. Security validation
8. Audit validation
9. Deployment validation
10. Release validation

Each layer answers a different question.

## Documentation Validation

Documentation validation asks:

Does the documentation provide enough clear guidance for implementation?

Required checks:

- required documents exist
- terminology is consistent
- architecture does not contradict invariants
- acceptance criteria exist for planned work
- roadmap and phasemap align
- documentation changes accompany behavior changes
- release-sensitive statements distinguish published release from current development
- package, desktop, changelog, roadmap, phasemap, and task versions align with the release-truth record
- duplicate or conflicting task states are rejected

Documentation validation should block work when implementation requires guessing.

## Schema Validation

Schema validation asks:

Are request, response, audit, policy, approval, and state contracts valid and stable?

Required checks:

- schemas parse successfully
- required fields are enforced
- invalid examples fail validation
- valid examples pass validation
- enum values are bounded
- compatibility rules are documented

Schemas are contracts. Broken schemas block implementation.

## Policy Validation

Policy validation asks:

Can policy be trusted before activation?

Required checks:

- policy files parse successfully
- every tool has a capability class
- unknown tools deny by default
- L2 and L3 tools route to approval where required
- policy bundle metadata exists
- policy hash is computed
- policy signature is valid where signing exists
- compatibility metadata is valid

Invalid policy must not activate.

## Unit Validation

Unit validation asks:

Does each isolated component behave correctly?

Required checks:

- request validation rejects malformed input
- policy evaluation returns expected decisions
- wrappers fail closed
- audit builders include required fields
- state transitions are explicit
- duplicate execution detection works

Unit tests should avoid external dependencies where practical.

## Integration Validation

Integration validation asks:

Do components work together without violating architecture?

Required checks:

- orchestrator request reaches gateway
- gateway invokes policy engine
- policy result controls wrapper behavior
- wrapper result controls execution
- audit evidence is emitted
- response is returned with correct status

Integration tests must include allowed, denied, and pending paths.

## Replay Validation

Replay validation asks:

Can AEGIS resume execution deterministically without re-planning?

Required checks:

- pending state is durable
- approval event is verified
- stored tool request is replayed exactly
- LLM planning layer is not called during replay
- duplicate replay is detected
- terminal state is durable

Replay tests are mandatory for human approval workflows.

## Security Validation

Security validation asks:

Does the system preserve security invariants under normal and failure conditions?

Required checks:

- no long-lived credentials are exposed to agents
- secrets are not logged
- invalid approvals are rejected
- stale approvals are rejected
- invalid policy bundles are rejected
- wrapper failures stop execution
- malformed requests do not execute
- deny by default is enforced

Security validation must emphasize negative paths.

## Audit Validation

Audit validation asks:

Can future reviewers reconstruct what happened?

Required checks:

- every decision emits an audit record
- audit records include execution identity
- audit records include policy provenance
- denials include reason
- failures include reason
- approval decisions include approver identity
- replay attempts are recorded
- audit records do not contain secrets

Audit validation is required for all externally visible actions.

## State Validation

State validation asks:

Can execution survive restarts and failures?

Required checks:

- pending state survives restart
- terminal state survives restart
- approval state persists
- duplicate attempts are detected after restart
- state transitions are valid
- invalid transitions are rejected

Memory-only state is not sufficient for governed execution.

## Deployment Validation

Deployment validation asks:

Can AEGIS run safely in the target environment?

Required checks:

- required configuration exists
- configuration validates
- policy bundle loads successfully
- policy provenance is visible
- runtime dependencies are present
- startup fails closed when required validation fails

Deployment validation should happen before serving traffic.

## Release Validation

Release validation asks:

Is the repository ready to publish a versioned release?

Required checks:

- tests pass
- documentation is current
- changelog is updated
- known limitations are documented
- schemas are versioned where needed
- release artifacts are reproducible
- security review is complete
- phasemap exit criteria are satisfied
- release artifacts match latest-release documentation
- unreleased changes are labeled as current development
- release version identity is consistent across release-facing surfaces

## Negative Path Requirements

AEGIS must validate what does not happen.

Required negative paths include:

- unknown tool denied
- malformed request denied
- invalid policy denied
- invalid approval denied
- stale approval denied
- wrapper failure denied
- duplicate replay blocked
- unsupported schema rejected
- incompatible policy bundle rejected
- secret logging prevented

A system that only tests successful execution is not sufficiently validated.

## Evidence Requirements

Validation should produce evidence where practical.

Evidence may include:

- test output
- schema validation output
- policy validation output
- audit samples
- replay traces
- CI logs
- release notes

Evidence should be durable enough for review.

## CI Expectations

Continuous integration should eventually enforce:

- formatting
- linting
- unit tests
- integration tests
- schema validation
- policy validation
- documentation checks
- security scans

Failed validation should block merge or release depending on maturity phase.

## Manual Validation

Some validation may require human review.

Examples:

- architecture review
- threat model review
- policy risk review
- documentation quality review
- release readiness review

Manual validation should be documented in task or release notes.

## Validation Ownership

Developers own tests for their changes.

Reviewers own validation completeness.

Maintainers own release validation.

Security reviewers own security-sensitive validation.

## Validation Checklist

Before work is complete, confirm:

- required docs are updated
- acceptance criteria are met
- schemas validate
- policy validates
- tests cover expected paths
- tests cover negative paths
- audit evidence is produced
- secrets are not logged
- replay behavior is deterministic
- fail-closed behavior is proven

## Final Rule

Validation is how AEGIS earns trust.

If a behavior cannot be validated, it should not be treated as reliable.

If a security property cannot be proven, it should not be claimed.

If a failure path cannot be tested, it should be documented as a risk until validation exists.
