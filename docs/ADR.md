# AEGIS
# Architecture Decision Records v1.0

## Purpose

This document defines how AEGIS records architecture decisions.

Architecture decisions must be explicit because AEGIS relies on documentation-driven engineering, deterministic governance, and auditable implementation choices.

## ADR Principle

Decisions that affect architecture, invariants, security posture, policy semantics, runtime evidence, compatibility, or deployment safety must be recorded before or with the change that depends on them.

## When an ADR Is Required

An ADR is required when a change affects:

- trust boundaries
- gateway responsibilities
- policy evaluation semantics
- wrapper enforcement semantics
- credential handling
- audit evidence
- replay behavior
- schema compatibility
- policy distribution
- deployment model
- major dependency choices

## ADR Format

Each ADR should include:

- title
- status
- date
- context
- decision
- consequences
- alternatives considered
- affected documents

## Status Values

ADR status values are:

- proposed
- accepted
- superseded
- rejected

Status values must not be invented without updating this document.

## Storage

Future ADR entries should be stored in a stable location under repository governance.

Until a separate ADR directory is created, this document defines the required ADR format and process.

## Decision Review

ADR review must verify:

- the decision does not contradict `OPERATING_DOCTRINE.md`
- the decision preserves `INVARIANTS.md`
- affected architecture and acceptance criteria are updated
- security and audit implications are understood
- migration guidance exists for breaking changes

## Initial Decision Register

No implementation ADRs have been accepted yet.

The initial governance decision is that AEGIS follows Documentation-Driven Engineering and implementation must not begin until the Phase 0 governance baseline is complete.

## ADR-0011: Rust Gateway MVP Runtime

### Status

Accepted

### Date

2026-06-28

### Context

AEGIS requires a gateway runtime that can preserve deterministic behavior, fail closed, enforce typed boundaries, support structured audit evidence, and safely grow toward policy and wrapper enforcement.

Phase 0 established the governance foundation. Phase 1 established protocol and schema contracts. Phase 2 begins the Gateway MVP and requires an initial runtime decision before implementation proceeds.

### Decision

Use Rust for the Phase 2 Gateway MVP runtime.

Keep Python for repository verification and schema governance tooling.

Defer TypeScript until a user interface or dashboard is required.

Use Bash only for small operational scripts where appropriate.

### Consequences

Rust becomes the initial runtime language for gateway implementation.

Gateway code must follow the Rust overlay rules.

The repository must add Rust formatting, linting, and tests to CI.

Runtime behavior must remain contract-driven by schemas and documentation.

### Alternatives Considered

Python was not selected for the initial gateway runtime because AEGIS needs stronger compile-time type boundaries and stricter default error handling for the core execution checkpoint. Python remains appropriate for repository verification and schema governance tooling.

Go was not selected because Rust provides stronger ownership and type guarantees for fail-closed execution boundaries. Go remains a viable future implementation language if a separate gateway port is justified.

TypeScript was not selected because the current phase does not require a user interface, dashboard, or browser-oriented runtime. It should be deferred until UI or integration needs exist.

Bash was not selected because the gateway requires typed contracts, structured errors, tests, and long-term maintainability beyond small operational scripting.

Zig was not selected because Rust currently provides a broader mature ecosystem for security-sensitive application services, testing, and CI integration while still preserving explicit low-level control.

### Affected Documents

- README.md
- CHANGELOG.md
- docs/TASKS.md
- .github/workflows/validate.yml
