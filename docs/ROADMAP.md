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

### Objective
Implement the minimum local gateway capable of receiving requests, validating schema, verifying a local policy bundle, evaluating simple local policy, returning deterministic decisions, emitting audit evidence, and optionally persisting local audit records.

### Deliverables
- gateway entrypoint
- request validation
- policy decision interface
- response mapping
- basic audit record creation
- deny-by-default behavior
- local runtime JSON output
- local policy bundle loading
- SHA-256 checksum verification
- Ed25519 signature verification
- local policy and risk matrix evaluation
- append-only local JSONL audit logging

### Exit Criteria
- valid L0 requests can pass through the local gateway path
- unknown tools are denied
- malformed requests are denied
- all decisions emit audit evidence
- verified local policy bundles can produce allowed, denied, and pending decisions
- invalid or unverifiable policy bundles fail closed
- optional local audit logging appends one JSONL record per completed decision
- tests cover allowed, denied, pending, malformed, unsupported, bundle failure, policy failure, and audit persistence paths

## Phase 3: Policy Engine Hardening

### Objective
Harden declarative policy evaluation beyond the local development MVP while preserving independence from wrappers and orchestration logic.

### Deliverables
- production-oriented gateway_policy.yaml validation
- production-oriented risk_matrix.yaml validation
- expanded capability class handling
- policy compatibility validation
- complete policy provenance capture
- compatibility metadata

### Exit Criteria
- policy evaluation is side-effect free
- policy output is deterministic
- policy version and hash appear in audit records
- unclassified tools deny by default

## Phase 4: Security Wrappers

### Objective
Implement the enforcement layer that acts on gateway and policy decisions.

### Deliverables
- task-scoped authorization wrapper
- permission isolation wrapper
- credential injection wrapper
- HITL approval verification wrapper
- wrapper failure handling

### Exit Criteria
- wrappers fail closed
- secrets are not exposed to agents
- wrapper decisions are auditable
- wrapper tests cover failure paths

## Phase 5: Durable State and Replay

### Objective
Support long-running workflows, pending approvals, crash recovery, and deterministic replay.

### Deliverables
- execution state model
- pending approval state
- replay token handling
- idempotency locks
- exactly-once execution checks
- resume protocol

### Exit Criteria
- pending state survives restart
- replay does not call planning layer
- duplicate execution is prevented
- terminal state is durable

## Phase 6: Human Approval Workflows

### Objective
Implement explicit human-in-the-loop approval behavior for high-risk actions.

### Deliverables
- approval request model
- approval decision model
- approver identity binding
- approval expiration
- stale approval denial
- approval audit evidence

### Exit Criteria
- L2 and L3 actions can be routed to pending
- approval is bound to action identity
- denied approvals prevent execution
- stale approvals cannot be reused

## Phase 7: Policy Distribution

### Objective
Support signed, immutable, locally enforced policy bundles.

### Deliverables
- policy bundle manifest
- checksum verification
- signature verification
- bundle compatibility checks
- activation rules
- rollback guidance

### Exit Criteria
- invalid bundles are rejected
- active policy is identifiable
- gateway decisions include policy provenance
- runtime silent policy mutation is prohibited

## Phase 8: Observability and Audit

### Objective
Make execution, policy decisions, approval state, and failure modes visible and investigable.

### Deliverables
- structured audit records
- operational logs
- metrics
- trace IDs
- audit export format
- SIEM-friendly field names

### Exit Criteria
- every decision emits audit evidence
- audit records are structured
- audit records include execution and policy provenance
- operational logs do not expose secrets

## Phase 9: Deployment Reference

### Objective
Provide reference deployment patterns for local, container, and enterprise environments.

### Deliverables
- local development guide
- container build
- Kubernetes deployment example
- GitOps deployment example
- configuration examples
- secret handling guidance

### Exit Criteria
- gateway can run locally
- gateway can run as a container
- policy bundle can be mounted or deployed immutably
- deployment guidance preserves invariants

## Phase 10: Orchestrator Integrations

### Objective
Demonstrate how different AI orchestrators can integrate without changing AEGIS architecture.

### Deliverables
- generic HTTP integration example
- LangGraph adapter example
- AutoGen adapter example
- CrewAI adapter example
- reference FSM integration

### Exit Criteria
- each integration uses ToolCallRequest and ToolCallResponse
- each integration handles pending correctly
- no integration bypasses the gateway
- replay does not re-plan

## Phase 11: Production Hardening

### Objective
Prepare AEGIS for production-like environments.

### Deliverables
- high availability guidance
- migration strategy
- backup and restore guidance
- rate limiting
- concurrency controls
- security review
- performance profiling

### Exit Criteria
- gateway failure modes are documented
- state recovery is tested
- concurrency does not break exactly-once execution
- release process is documented

## Phase 12: Reference Implementation Maturity

### Objective
Stabilize AEGIS as a reference architecture and implementation.

### Deliverables
- versioned release
- changelog
- compatibility matrix
- complete test suite
- documented limitations
- contribution guide

### Exit Criteria
- release artifacts are reproducible
- documentation matches implementation
- public APIs are stable enough for external use
- known risks are documented

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
