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

### Required Capabilities
- wrapper execution boundary
- credential injection boundary
- execution lifecycle state machine
- approval workflow boundary
- durable execution state
- replay and recovery behavior
- mutation-capable execution path
- UI-ready structured status, error, lifecycle, audit, state, inspection, and recovery plan evidence
- governed execution integration tests

### Exit Criteria
- allowed actions execute only through wrappers
- denied actions never execute
- pending actions persist state and do not block active execution
- credentials are injected only at execution time and never exposed to agents
- wrapper failures fail closed
- terminal execution state is durable
- replay uses stored intent and does not call the planning layer
- mutation-capable requests are idempotent or fail closed according to policy
- execution audit evidence links request, policy, wrapper, approval where applicable, execution outcome, and persisted state
- runtime evidence remains structured enough for graphical display

## v0.4.0: Platform Capabilities

### Purpose
Expose and operate the governed execution engine through platform boundaries after runtime behavior is stable.

### Required Capabilities
- HTTP API boundary
- service deployment model
- runtime configuration model
- operational observability
- plugin or wrapper extension architecture
- Tauri desktop shell
- graphical operator workflows for execution status, errors, evidence, state, inspection, and recovery plans
- orchestrator integration references

### Exit Criteria
- service API preserves ToolCallRequest and ToolCallResponse contracts
- deployment guidance preserves immutable policy and fail-closed behavior
- configuration is explicit and validated
- operational telemetry does not replace audit evidence
- plugins cannot bypass gateway, policy, wrapper, or audit boundaries
- UI displays runtime state without owning policy decisions
- UI consumes backend evidence and cannot bypass gateway execution logic

## v0.5.0: Production Hardening

### Purpose
Prepare AEGIS for production-oriented evaluation.

### Required Capabilities
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
