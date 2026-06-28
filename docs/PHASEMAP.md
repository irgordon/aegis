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

## v0.2.0: Contract Foundation

### Purpose
Define stable request, response, audit, policy, and execution contracts.

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

### Exit Criteria
- schemas validate
- required fields are defined
- malformed input behavior is documented
- contract tests exist or are planned in TASKS.md
- repository verification succeeds
- compatibility expectations are documented

## v0.3.0: Gateway MVP

### Purpose
Implement the minimum executable gateway path.

### Required Capabilities
- receive tool request
- validate request schema
- assign or verify execution identity
- evaluate simple policy
- return allow, deny, or pending
- emit audit record

### Exit Criteria
- valid low-risk request passes through gateway
- unknown tool is denied
- malformed request is denied
- every decision emits audit evidence

## v0.4.0: Policy Engine Baseline

### Purpose
Introduce declarative policy evaluation.

### Required Capabilities
- gateway_policy.yaml support
- risk_matrix.yaml support
- capability classification
- policy validation
- policy provenance fields
- deny-by-default tool behavior

### Exit Criteria
- policy evaluation is side-effect free
- deterministic decision tests exist
- unclassified tools deny by default
- policy version and hash are captured

## v0.5.0: Security Wrapper Baseline

### Purpose
Introduce enforceable execution wrappers.

### Required Capabilities
- task-scoped authorization
- permission isolation
- credential injection boundary
- wrapper failure handling
- wrapper audit evidence

### Exit Criteria
- wrappers fail closed
- secrets are not exposed to agents
- wrapper failures prevent execution
- wrapper paths are tested

## v0.6.0: Durable State and Replay

### Purpose
Support long-running execution and deterministic replay.

### Required Capabilities
- execution state model
- pending approval persistence
- replay token handling
- attempt number tracking
- terminal state recording
- duplicate execution detection

### Exit Criteria
- pending state survives restart
- replay does not re-plan
- duplicate replay is blocked or safely resolved
- exactly-once execution is preserved

## v0.7.0: Human Approval Workflows

### Purpose
Support asynchronous human-in-the-loop governance.

### Required Capabilities
- approval request creation
- approver identity capture
- approval binding to action identity
- denial handling
- stale approval rejection
- approval audit evidence

### Exit Criteria
- L2 and L3 actions route to pending
- approvals are action-specific
- denied approvals do not execute
- stale approvals cannot be reused

## v0.8.0: Policy Distribution

### Purpose
Support immutable signed policy bundles.

### Required Capabilities
- bundle manifest
- checksum verification
- signature verification
- schema compatibility check
- wrapper compatibility check
- activation rules
- rollback guidance

### Exit Criteria
- invalid bundles are rejected
- active policy provenance is recorded
- runtime silent policy mutation is prohibited
- rollback procedure is documented

## v0.9.0: Observability and Enterprise Evidence

### Purpose
Make AEGIS operationally investigable.

### Required Capabilities
- structured audit records
- operational logs
- trace or request IDs
- SIEM-friendly field names
- audit export guidance
- secret-safe logging

### Exit Criteria
- every material decision produces audit evidence
- audit records include execution and policy provenance
- logs do not expose secrets
- investigation flow is documented

## v0.10.0: Deployment Reference

### Purpose
Provide practical deployment patterns.

### Required Contents
- local development guide
- container guidance
- Kubernetes reference
- GitOps policy deployment guidance
- configuration examples
- operational failure guidance

### Exit Criteria
- gateway can run locally
- container deployment is documented
- policy bundle deployment is documented
- deployment preserves invariants

## v0.11.0: Orchestrator Integration Reference

### Purpose
Demonstrate integration with multiple AI orchestration approaches.

### Required Contents
- generic HTTP integration
- reference finite state machine
- pending approval handling
- deterministic resume example
- replay without re-planning example

### Exit Criteria
- integration obeys ToolCallRequest and ToolCallResponse
- pending state dehydrates execution
- resumed execution replays stored action
- no integration bypasses the gateway

## v0.12.0: Production Hardening Candidate

### Purpose
Prepare AEGIS for production-oriented evaluation.

### Required Capabilities
- concurrency controls
- idempotency enforcement
- backup and restore guidance
- high availability guidance
- migration guidance
- release process
- compatibility matrix

### Exit Criteria
- race conditions are tested where applicable
- exactly-once behavior is validated
- recovery guidance exists
- release process is documented

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