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

The Phase 3 local governed execution foundation is complete after reclassification. The local runtime can dispatch and execute the built-in read-only `health.check` wrapper after policy allows it, execute the local sandbox mutation wrapper `sandbox.note.write`, pass a safe local development credential handle for that sandbox mutation, return structured error reports, expose lifecycle state, optionally append a local execution state log, inspect state evidence, and generate bounded recovery plans.

It does not yet execute real external actions, replay execution, approval workflow, production credential providers, HTTP service behavior, or live UI evidence. Those concerns now belong to later phases instead of the completed local execution foundation.

Phase 4 has begun with a static Tauri shell and Slint landing window. The CLI remains a support surface for validation, inspection, testing, and automation. The initial UI scaffold does not render live backend evidence, call gateway execution, define IPC commands, or provide authority.

For contributors, the backlog has been reorganized around the shortest path from local execution evidence to a useful operator surface. Completed Phase 2 and Phase 3 foundation work is no longer an active task list. Active work now starts with Phase 4 evidence-first graphical rendering.

For engineers and architects, execution remains the primary architectural concern, but Phase 4 must keep the UI non-authoritative. The UI renders backend evidence; it does not evaluate policy, authorize execution, inject credentials, dispatch wrappers, or invent lifecycle or recovery state.

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

### Status
Complete for local built-in execution foundation.

### Objective
Execute real AI actions safely under governance.

Phase 2 proved that AEGIS can decide and record. Phase 3 proved that AEGIS can execute bounded local actions without violating the architecture: wrappers enforce decisions, credentials stay out of agent hands, mutations require stronger gates, lifecycle state is explicit, and recovery inspection remains read-only.

### Completed Capabilities
- wrapper execution boundary
- execution lifecycle state machine
- structured error reporting
- execution authorization boundary
- credential class boundary
- local development credential handle boundary
- built-in local L0 `health.check` execution
- built-in local L1 `sandbox.note.write` execution
- optional append-only local execution state log
- read-only execution recovery inspection
- bounded recovery plan generation
- mutation-capable execution path
- UI-ready structured status, error, lifecycle, audit, state, inspection, and recovery plan evidence
- integration tests for governed execution

### Deferred From Original Phase 3
- replay and recovery execution move to Phase 5
- approval workflow moves to Phase 6
- production credential providers move to Phase 6
- HTTP service, deployment, observability, and plugin architecture move to Phase 7

### Exit Criteria
- allowed actions execute only through wrappers
- denied actions never execute
- pending actions do not execute
- local credential handles are safe references and never expose secrets
- wrapper failures fail closed
- local execution state transitions can be persisted as JSONL evidence
- recovery inspection and recovery planning are read-only
- mutation-capable local requests require policy allow, authorization, credential compatibility, idempotency context, sandbox containment, and audit evidence
- audit evidence links request, policy, wrapper, credential boundary, credential handle reference where applicable, execution outcome, lifecycle, and persisted state evidence
- no HTTP service or UI implementation is required for Phase 3 completion
- runtime status and error output remains structured enough for graphical display

## Phase 3.5: UI-Ready Evidence and Documentation

### Status
Complete.

### Objective
Prepare backend evidence and documentation for a graphical operator surface without giving the UI authority.

### Completed Capabilities
- `/docs/wiki/` knowledge base
- wiki language and accuracy review
- UI evidence contract
- Slint with Tauri direction
- graphical operator feedback documentation
- stable README communication standard

### Exit Criteria
- UI-facing evidence sources are documented
- the UI authority boundary is explicit
- recovery inspection and planning are documented as read-only
- docs explain current behavior without requiring code readers to infer UI needs

## Phase 4: Graphical Operator Surface

### Status
Started.

### Objective
Render governed execution evidence in a non-authoritative graphical desktop operator surface.

### Deliverables
- Tauri desktop shell with Slint graphical UI layer
- sample execution timeline from fixture evidence
- sample status cards from fixture evidence
- normalized error cards from fixture evidence
- minimal IPC data bridge
- live read-only runtime evidence rendering
- read-only audit, state, recovery inspection, and recovery plan views

### Phase 4 Progression
Completed Phase 4 foundation:

- static Tauri shell with Slint landing window

Next sequence:

1. Render execution timeline from sample evidence.
2. Render sample status cards from fixture evidence.
3. Render normalized error cards from fixture evidence.
4. Define the minimal IPC data bridge.
5. Render live read-only runtime evidence.
6. Render audit, state, recovery inspection, and recovery plan views read-only.

### Exit Criteria
- UI displays runtime state without owning policy decisions
- UI consumes backend evidence and cannot bypass gateway execution logic
- UI renders sample evidence before live IPC
- live evidence rendering is read-only
- graphical timelines, status cards, and error cards preserve backend meaning
- audit, state, recovery inspection, and recovery plan views do not execute recovery or replay

## Phase 5: Recovery and Replay Execution

### Objective
Move from read-only recovery inspection and planning to constrained recovery and replay behavior.

### Deliverables
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

## Phase 6: Approval and Production Credential Providers

### Objective
Add human approval workflow and production credential provider boundaries after local execution and UI evidence paths are coherent.

### Deliverables
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

## Phase 7: Platform and Production Hardening

### Objective
Prepare AEGIS for production-oriented platform evaluation.

### Deliverables
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
