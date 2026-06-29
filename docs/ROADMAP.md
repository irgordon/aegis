# AEGIS
# Roadmap v1.0

## Purpose

This document defines the planned development path for AEGIS.

The roadmap is not a release promise. It is a sequencing guide that keeps implementation aligned with doctrine, architecture, invariants, and acceptance criteria.

## Roadmap Principles

AEGIS development follows these principles:

- governance before implementation
- architecture before optimization
- deterministic behavior before feature growth
- security before convenience
- evidence before claims
- small verifiable phases before broad expansion

## Current Repository State

AEGIS now has a working local Gateway MVP.

For a new reader, this means the repository can prove the gateway decision path locally: it reads a structured request, verifies a local policy bundle, evaluates simple policy, returns allowed, denied, or pending, records audit evidence, and can append a local audit record.

The Phase 3 foundation has also begun. The local runtime can dispatch and execute the built-in read-only `health.check` wrapper after policy allows it, return structured error reports, and expose an in-memory execution lifecycle.

It does not yet execute real external actions. That is why the remaining Phase 3 work focuses on governed execution beyond the local health check.

For contributors, the backlog has been reorganized around the shortest path from decision evidence to safe execution. Completed Phase 2 implementation work is no longer an active task list. Active work now starts with Phase 3 runtime execution boundaries.

For engineers and architects, execution is now the primary architectural concern. Phase-level governance still precedes implementation, but execution-specific documentation must evolve with implementation evidence as wrappers, credentials, lifecycle state, approval, replay, and mutation behavior become concrete. Documentation updates remain part of the same Definition of Done as implementation and validation.

## Phase 0: Governance Baseline

### Objective
Establish the repository as a documentation-driven engineering project.

### Deliverables
- README.md
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
- Core governance documents exist.
- Documentation hierarchy is defined.
- Invariants are documented.
- Acceptance criteria exist.
- Future implementation can proceed without guessing core intent.

## Phase 1: Protocol and Schema Foundation

### Objective
Define the stable protocol contracts used by orchestrators, gateways, policy engines, wrappers, and audit systems.

### Deliverables
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

### Exit Criteria
- Schemas validate.
- Required fields are documented.
- Invalid requests are rejected by tests.
- Tool call responses map to allowed, denied, and pending states.
- Repository verification succeeds.
- Compatibility expectations are documented.

## Phase 2: Gateway MVP

### Status
Complete.

### Objective
Implement the minimum local gateway capable of receiving requests, validating schema, verifying a local policy bundle, evaluating simple local policy, returning deterministic decisions, emitting audit evidence, and optionally persisting local audit records.

### Completed Capabilities
- Rust request and response models
- request validation pipeline
- deterministic response mapping
- explicit policy decision adapter boundary
- deny-by-default unsupported tool handling
- local runtime JSON output
- verified local policy bundle loading
- SHA-256 checksum verification
- Ed25519 signature verification
- local policy and risk matrix evaluation
- append-only local JSONL audit logging
- contract and integration tests for allowed, denied, pending, malformed, unsupported, policy failure, bundle failure, and audit persistence paths

### Exit Criteria
- valid L0 requests pass through the local gateway path
- unknown tools are denied
- malformed requests are denied
- all decisions emit audit evidence
- verified local policy bundles produce allowed, denied, and pending decisions
- invalid or unverifiable policy bundles fail closed
- optional local audit logging appends one JSONL record per completed decision
- validation passes

## Phase 3: Governed Execution Engine

### Objective
Execute real AI actions safely under governance.

Phase 2 proved that AEGIS can decide and record. Phase 3 must prove that AEGIS can execute without violating the architecture: wrappers enforce decisions, credentials stay out of agent hands, approval state is durable, replay is mechanical, and mutations are idempotent where required.

### Deliverables
- wrapper execution boundary
- credential injection boundary
- execution lifecycle state machine
- approval workflow boundary
- durable execution state
- replay and recovery behavior
- mutation-capable execution path
- integration tests for governed execution

### Phase 3 Progression
Completed Phase 3 foundation:

- wrapper dispatcher and execution boundary
- structured gateway error reports
- built-in local L0 `health.check` execution
- in-memory execution lifecycle state model

Remaining Phase 3 priorities:

1. Mutation-capable execution path
2. Credential injection boundary
3. Approval workflow boundary
4. Durable execution state
5. Replay and recovery
6. Execution and replay evidence
7. Governed execution integration testing

### Exit Criteria
- allowed actions execute only through wrappers
- denied actions never execute
- pending actions persist state and do not block active execution
- credentials are injected only at execution time and never exposed to agents
- wrapper failures fail closed
- terminal execution state is durable
- replay uses stored intent and does not call the planning layer
- mutation-capable requests are idempotent or fail closed according to policy
- audit evidence links request, policy, wrapper, approval where applicable, execution outcome, and persisted state
- no HTTP service or UI is required for Phase 3 completion

## Phase 4: Platform Capabilities

### Objective
Expose and operate the governed execution engine through platform boundaries after runtime behavior is stable.

### Deliverables
- HTTP API boundary
- service deployment model
- runtime configuration model
- operational observability
- plugin or wrapper extension architecture
- desktop UI only after runtime behavior is stable
- orchestrator integration references

### Exit Criteria
- service API preserves ToolCallRequest and ToolCallResponse contracts
- deployment guidance preserves immutable policy and fail-closed behavior
- configuration is explicit and validated
- operational telemetry does not replace audit evidence
- plugins cannot bypass gateway, policy, wrapper, or audit boundaries
- UI, if present, displays runtime state without owning policy decisions

## Phase 5: Production Hardening

### Objective
Prepare AEGIS for production-oriented evaluation.

### Deliverables
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

## Future Tracks

Potential future tracks include:

- OPA policy integration
- Cedar policy integration
- SPIFFE/SPIRE workload identity
- hardware-backed signing
- multi-party approval
- policy simulation
- risk scoring
- visual execution graph
- enterprise dashboard
- multi-language gateway implementations

## Roadmap Governance

Roadmap changes should be reviewed when they affect:

- architecture
- invariants
- release sequencing
- policy model
- security posture
- compatibility expectations

## Final Rule

The roadmap may change.

The doctrine and invariants govern how it changes.

AEGIS should grow deliberately, with every phase strengthening determinism, governance, auditability, and security.
