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

## ADR-0012: Release Truth and Product Version Identity

### Status

Accepted

### Date

2026-07-14

### Context

The public `v0.4.1` release and the current development branch diverged after the
release tag was created. Documentation began describing smoke-test guidance,
first-run help, and desktop identity changes that existed on the development
branch but not in the immutable published artifacts.

The repository also used unrelated version labels across GitHub releases,
changelog entries, Cargo packages, and Tauri metadata. That made it unclear
whether a version described a public release, crate compatibility, or an internal
repository iteration.

### Decision

Adopt a Release Truth invariant.

Maintain one machine-readable release-truth record containing the latest
published release, the current development target, the active engineering phase,
and the planned platform order.

Use one product version for release-facing Git tags, GitHub Releases, artifact
names, Cargo package versions, Tauri application metadata, and release changelog
headings. Use an `Unreleased` changelog section between published releases.

Engineering phases describe implementation maturity. Release versions describe
validated outcomes. A phase does not reserve a version number.

Preserve existing public tags and releases as immutable historical evidence.

### UI Integrity Review

The landing screen identity text changes from an ambiguous release label to
explicit latest-release and current-development labels. It shows no new runtime
state, requests no user action, changes no navigation, and adds no authority.
The explicit text prevents users from mistaking development behavior for the
published artifact. Meaning does not depend on color, and the existing layout,
evidence labels, accessibility treatment, and backend authority boundary remain
unchanged.

### Consequences

Repository verification must reject contradictory release statements, duplicate
task states, and version metadata that differs from the governed development
target.

Public documentation defaults to latest-release truth. Development behavior must
be labeled explicitly.

The historical `v0.4.1` release remains unchanged. The first release under this
decision is the planned `v0.4.2` Developer Preview Refresh.

Legacy `0.2.x` changelog headings remain historical repository iteration labels.
They are not public release versions and no new headings use that scheme.

### Alternatives Considered

Keeping separate undocumented versions for Cargo, Tauri, changelog entries, and
GitHub Releases was rejected because it preserves ambiguity.

Moving or recreating `v0.4.1` was rejected because it would destroy immutable
release evidence.

Waiting until `v0.5.0` was rejected because the post-tag work improves the
existing Developer Preview without introducing a larger capability milestone.

### Affected Documents

- README.md
- CHANGELOG.md
- docs/PRD.md
- docs/ARCHITECTURE.md
- docs/INVARIANTS.md
- docs/DOCUMENTATION.md
- docs/ACCEPTANCE_CRITERIA.md
- docs/TEST_STRATEGY.md
- docs/VALIDATION.md
- docs/RELEASE_PROCESS.md
- docs/COMPATIBILITY.md
- docs/ROADMAP.md
- docs/PHASEMAP.md
- docs/TASKS.md
- config/release-truth.json
- scripts/verify.py
- .github/workflows/validate.yml
