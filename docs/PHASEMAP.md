# AEGIS
# Phasemap v1.0

## Purpose

This document maps AEGIS development phases to version milestones.

The roadmap describes direction.

The phasemap defines versioned maturity gates.

Each version exists to make the project more complete, deterministic, secure, and auditable.

## Versioning Principle

AEGIS uses version milestones to communicate maturity, not marketing status.

A version is valid only when its documented exit criteria are satisfied.

## Current Phase Model

| Phase | Version Track | Name | Status | Purpose | Current Tasks | Deferred Tasks |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | v0.1.x | Governance Baseline | Complete | Establish documentation-driven governance. | None. | None. |
| 1 | v0.1.x | Contracts and Architecture Foundation | Complete | Define schemas, examples, and compatibility contracts. | None. | None. |
| 2 | v0.2.0 | Local Gateway MVP | Complete | Prove local governed request-to-response behavior. | None. | None. |
| 3 | v0.3.0 | Governed Execution Engine | Complete for local built-in execution foundation. | Prove safe local wrapper execution under policy, authorization, credential, audit, and state boundaries. | None. | Replay execution, approval workflow, and production credential providers. |
| 3.5 | v0.3.x | UI-Ready Evidence and Documentation | Complete | Make backend evidence understandable and renderable by a future UI. | None. | Live UI rendering and IPC. |
| 4 | v0.4.0 | Graphical Operator Surface | Started | Render backend evidence in a non-authoritative Tauri plus Slint desktop UI. | Sample evidence timeline rendering. | HTTP service, platform deployment, replay execution, approval workflow, and production credential providers. |
| 5 | v0.5.0 | Recovery and Replay Execution | Not started | Add constrained recovery and replay behavior after read-only inspection and planning. | None. | Approval workflow and production credential providers. |
| 6 | v0.6.0 | Approval and Production Credential Providers | Not started | Add human approval workflow and real credential provider boundaries. | None. | Platform and production hardening. |
| 7 | v0.7.0 | Platform and Production Hardening | Not started | Add service, deployment, observability, extension, security, release, and operational maturity. | None. | Post-1.0 ecosystem tracks. |

## Current Gate Summary

| Version | Entry Criteria | Exit Criteria |
| --- | --- | --- |
| v0.3.0 | Phase 2 local Gateway MVP complete. | Local built-in wrapper execution, authorization, credential boundaries, lifecycle, audit, state, recovery inspection, and recovery planning are validated. |
| v0.4.0 | UI-ready backend evidence exists and Tauri plus Slint direction is documented. | The UI renders fixture evidence first, then read-only live evidence, without owning policy, authorization, credential, wrapper, audit, state, or recovery decisions. |
| v0.5.0 | Recovery inspection and planning evidence exists. | Replay and recovery execution are constrained, auditable, and fail closed. |
| v0.6.0 | Local execution and recovery boundaries are stable. | Approval and production credential providers are scoped, attributable, secret-safe, and fail closed. |
| v0.7.0 | Core runtime, UI, recovery, approval, and credential provider boundaries are stable. | Platform and production controls are tested, documented, and reproducible. |

## v0.0.0: Initial Repository

### Purpose
Establish the initial repository baseline.

### Scope
This version represents the project starting point.

It may contain README material, early governance notes, and initial documentation scaffolding.

### Required Contents
- README.md
- initial project identity
- repository name
- initial documentation directory

### Exit Criteria
- repository exists
- project name is established as AEGIS
- baseline README exists
- development can begin under source control

### Status
Initial baseline.

## v0.1.0: Governance Foundation

### Purpose
Establish documentation-driven engineering for the project.

### Required Contents
- OPERATING_DOCTRINE.md
- PRD.md
- ARCHITECTURE.md
- INVARIANTS.md
- CODING_STYLE.md
- DOCUMENTATION.md
- USER_FLOWS.md
- ACCEPTANCE_CRITERIA.md
- ARCHITECTURAL_PRINCIPLES.md
- ROADMAP.md
- PHASEMAP.md
- TASKS.md
- SECURITY_MODEL.md
- THREAT_MODEL.md
- TRUST_BOUNDARIES.md
- POLICY_ENGINE.md
- POLICY_DISTRIBUTION.md
- AUDIT_LOGGING.md
- ORCHESTRATOR_FSM_CONTRACT.md
- API_SPEC.md
- RUNTIME_EVIDENCE.md
- TEST_STRATEGY.md
- ADR.md
- RELEASE_PROCESS.md

### Exit Criteria
- documentation hierarchy is defined
- core invariants are documented
- acceptance criteria exist
- architecture intent is clear enough for implementation to begin

## v0.2.0: Phase 2 Complete

### Purpose
Define stable request, response, audit, policy, and execution contracts, and prove the local Rust gateway can process a request through governed output without executing external tools.

### Status
Complete.

### Required Contents
- ToolCallRequest schema
- ToolCallResponse schema
- AuditRecord schema
- PolicyBundleManifest schema
- approval request schema
- execution state schema
- API specification
- valid and invalid schema examples
- compatibility documentation
- repository verification script
- changelog
- Rust request and response models
- request validation pipeline
- deterministic response mapping
- local gateway runtime
- verified local policy bundle loading
- SHA-256 checksum verification
- Ed25519 signature verification
- local policy and risk matrix evaluation
- structured JSON output
- append-only local JSONL audit logging

### Exit Criteria
- schemas validate
- required fields are defined
- malformed input behavior is documented
- contract tests exist
- valid low-risk requests pass through the local gateway path
- unknown tools are denied
- malformed requests are denied
- unverifiable policy bundles are denied
- all completed decisions emit audit evidence
- optional local audit logging appends exactly one JSONL record per completed decision
- repository verification succeeds
- compatibility expectations are documented

## v0.3.0: Governed Execution Engine

### Purpose
Execute real AI actions safely under governance.

### Status
Complete for local built-in execution foundation.

### Required Capabilities
- wrapper execution boundary
- execution lifecycle state machine
- structured error reporting
- execution authorization boundary
- credential class boundary
- local development credential handle boundary
- built-in local L0 `health.check` execution
- built-in local L1 `sandbox.note.write` execution
- optional append-only local execution state log
- read-only recovery inspection
- bounded recovery plan generation
- mutation-capable execution path
- UI-ready structured status, error, lifecycle, audit, state, inspection, and recovery plan evidence
- documented Slint with Tauri UI direction for Phase 4
- governed execution integration tests

### Exit Criteria
- allowed actions execute only through wrappers
- denied actions never execute
- pending actions do not execute
- local credential handles are safe references and never expose secrets
- wrapper failures fail closed
- local execution state transitions can be persisted as JSONL evidence
- recovery inspection and recovery planning are read-only
- mutation-capable local requests require policy allow, authorization, credential compatibility, idempotency context, sandbox containment, and audit evidence
- execution audit evidence links request, policy, wrapper, credential boundary, safe credential handle reference where applicable, execution outcome, lifecycle, and persisted state evidence
- runtime evidence remains structured enough for graphical display

## v0.3.x: UI-Ready Evidence and Documentation

### Purpose
Prepare backend evidence and documentation for graphical rendering without granting UI authority.

### Status
Complete.

### Required Capabilities
- UI evidence contract
- `/docs/wiki/` knowledge base
- Slint with Tauri UI direction
- graphical operator feedback documentation
- stable README communication standard

### Exit Criteria
- UI-facing evidence sources are documented
- the UI authority boundary is explicit
- recovery inspection and recovery planning are documented as read-only
- documentation explains current behavior without requiring code readers to infer UI needs

## v0.4.0: Graphical Operator Surface

### Purpose
Render governed execution evidence in a non-authoritative graphical desktop operator surface.

### Status
Started.

### Required Capabilities
- Tauri desktop shell with Slint graphical UI layer
- static Tauri + Slint landing scaffold
- Slint-rendered sample execution timeline from fixture evidence
- Slint-rendered sample status cards from fixture evidence
- Slint-rendered normalized error cards from fixture evidence
- minimal IPC data bridge
- live read-only runtime evidence rendering
- read-only audit, state, recovery inspection, and recovery plan views

### Exit Criteria
- UI displays runtime state without owning policy decisions
- UI consumes backend evidence and cannot bypass gateway execution logic
- UI renders sample evidence before live IPC
- live evidence rendering is read-only
- graphical timelines, status cards, and error cards preserve backend meaning
- audit, state, recovery inspection, and recovery plan views do not execute recovery or replay

## v0.5.0: Recovery and Replay Execution

### Purpose
Move from read-only recovery inspection and planning to constrained recovery and replay behavior.

### Required Capabilities
- replay eligibility report
- replay dry-run plan
- constrained replay execution
- audit retry path
- recovery execution guardrails

### Exit Criteria
- replay uses stored intent and does not call the planning layer
- recovery actions preserve fail-closed behavior
- audit retry is bounded and traceable
- replay and recovery evidence is recorded
- no UI control can create recovery authority not present in backend evidence

## v0.6.0: Approval and Production Credential Providers

### Purpose
Add human approval workflow and production credential provider boundaries after local execution and UI evidence paths are coherent.

### Required Capabilities
- approval workflow boundary
- approval evidence and state persistence
- production credential provider boundary
- provider compatibility checks
- secret-safe audit and UI evidence

### Exit Criteria
- pending actions do not execute before valid approval
- approvals are attributable, scoped, and auditable
- credential providers do not expose secrets to agents, stdout, audit logs, state logs, or UI
- provider failures fail closed

## v0.7.0: Platform and Production Hardening

### Purpose
Prepare AEGIS for production-oriented platform evaluation.

### Required Capabilities
- HTTP API boundary
- service deployment model
- runtime configuration model
- operational observability
- plugin or wrapper extension architecture
- orchestrator integration references
- production PKI or trust distribution
- remote policy distribution
- high-availability deployment guidance
- performance and load testing
- security review
- fuzz testing
- compatibility guarantees
- release engineering
- operational documentation

### Exit Criteria
- service API preserves ToolCallRequest and ToolCallResponse contracts
- deployment guidance preserves immutable policy and fail-closed behavior
- configuration is explicit and validated
- operational telemetry does not replace audit evidence
- plugins cannot bypass gateway, policy, wrapper, or audit boundaries
- production trust anchors are documented and verified
- policy distribution supports explicit activation and rollback
- HA behavior preserves deterministic execution and durable state
- security review findings are resolved or tracked
- fuzz and negative-path tests cover critical parsers and boundaries
- compatibility guarantees are documented and tested
- release artifacts and operational procedures are reproducible

## v1.0.0: Reference Architecture Release

### Purpose
Establish AEGIS as a stable reference architecture and usable implementation baseline.

### Required Conditions
- core governance documents complete
- gateway MVP complete
- policy engine complete
- wrapper baseline complete
- audit baseline complete
- durable state baseline complete
- approval workflow baseline complete
- policy distribution baseline complete
- test strategy implemented
- known limitations documented

### Exit Criteria
- documentation matches implementation
- tests pass
- invariants are preserved
- release artifacts are reproducible
- public contracts are stable enough for external users

## Post-1.0 Tracks

Future versions may include:

- OPA integration
- Cedar policy support
- SPIFFE/SPIRE integration
- hardware-backed approval signing
- multi-party approval
- policy simulation
- enterprise dashboard
- multi-language implementations
- SDKs
- advanced risk scoring

## Phase Change Rules

A phase may advance only when:

- exit criteria are satisfied
- documentation is updated
- acceptance criteria are updated if needed
- tests exist or are explicitly tracked
- known limitations are documented

## Final Rule

AEGIS versions should represent evidence-backed maturity.

Do not advance a version because code exists.

Advance only when governance, architecture, implementation, testing, and documentation align.
