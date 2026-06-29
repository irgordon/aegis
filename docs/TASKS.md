# AEGIS
# Tasks v1.0

## Purpose

This document tracks the governed task backlog for AEGIS.

Tasks translate the roadmap and phasemap into reviewable work items. This document does not override the operating doctrine, product requirements, architecture, invariants, or acceptance criteria.

## Status Values

Task status values are bounded:

- `planned`: documented but not started
- `in_progress`: actively being worked
- `blocked`: cannot proceed until a dependency is resolved
- `complete`: finished and validated for its phase

No other status values should be used unless this document is updated.

## Phase 0: Governance Baseline

Objective: establish the documentation-driven governance foundation before implementation begins.

| Task | Status |
| --- | --- |
| Create README.md | complete |
| Create OPERATING_DOCTRINE.md | complete |
| Create PRD.md | complete |
| Create ARCHITECTURE.md | complete |
| Create INVARIANTS.md | complete |
| Create ARCHITECTURAL_PRINCIPLES.md | complete |
| Create CODING_STYLE.md | complete |
| Create DOCUMENTATION.md | complete |
| Create USER_FLOWS.md | complete |
| Create ACCEPTANCE_CRITERIA.md | complete |
| Create ROADMAP.md | complete |
| Create PHASEMAP.md | complete |
| Create VALIDATION.md | complete |
| Create SECURITY_MODEL.md | complete |
| Create THREAT_MODEL.md | complete |
| Create TRUST_BOUNDARIES.md | complete |
| Create POLICY_ENGINE.md | complete |
| Create POLICY_DISTRIBUTION.md | complete |
| Create AUDIT_LOGGING.md | complete |
| Create ORCHESTRATOR_FSM_CONTRACT.md | complete |
| Create API_SPEC.md | complete |
| Create RUNTIME_EVIDENCE.md | complete |
| Create TEST_STRATEGY.md | complete |
| Create ADR.md | complete |
| Create RELEASE_PROCESS.md | complete |
| Create required repository directories | complete |
| Validate README documentation links | complete |
| Validate required governance document presence | complete |

## Phase 1: Protocol and Schema Foundation

Objective: define stable protocol contracts used by orchestrators, gateways, policy engines, wrappers, and audit systems.

| Task | Status |
| --- | --- |
| Review existing schema file names and extensions | complete |
| Move or mirror governed schemas into root-level `schemas/` if approved | complete |
| Finalize ToolCallRequest schema | complete |
| Finalize ToolCallResponse schema | complete |
| Finalize AuditRecord schema | complete |
| Finalize PolicyBundleManifest schema | complete |
| Finalize ApprovalRequest schema | complete |
| Finalize ExecutionState schema | complete |
| Add valid and invalid schema examples | complete |
| Add schema validation command or script | complete |
| Align API_SPEC.md with finalized schemas | complete |
| Add repository verification script | complete |
| Add CHANGELOG.md | complete |
| Add COMPATIBILITY.md | complete |

## Phase 2: Gateway MVP

Objective: implement the minimum gateway path after protocol contracts are stable.

| Task | Status |
| --- | --- |
| Select initial implementation language and runtime | complete |
| Create gateway entrypoint | complete |
| Add minimal gateway entrypoint boundary | complete |
| Add local gateway runtime entrypoint | complete |
| Add durable append-only local audit logging | complete |
| Add policy bundle loader and verifier | complete |
| Add real policy bundle checksum verification | complete |
| Add policy bundle signature verification | complete |
| Add local policy and risk matrix evaluation | complete |
| Add idempotency contract model | complete |
| Add wrapper configuration contract models | complete |
| Add execution identity contract models | complete |
| Add approval token contract models | complete |
| Implement request validation | complete |
| Add schema-backed Rust request and response models | complete |
| Add schema-backed gateway validation pipeline | complete |
| Implement policy decision interface | complete |
| Add policy decision adapter interface | complete |
| Implement deterministic response mapping | complete |
| Implement basic audit record creation | in_progress |
| Add minimal audit record builder | complete |
| Enforce deny-by-default behavior | in_progress |
| Add deny-by-default unsupported tool handling | complete |
| Add allowed and denied path tests | in_progress |

## Phase 3: Policy Engine

Objective: create declarative policy evaluation independent of wrappers and orchestration logic.

| Task | Status |
| --- | --- |
| Define gateway_policy.yaml format | planned |
| Define risk_matrix.yaml format | planned |
| Implement capability class handling | planned |
| Implement policy validation | planned |
| Capture policy provenance in decisions | planned |
| Add deterministic policy tests | planned |

## Phase 4: Security Wrappers

Objective: implement enforcement wrappers after gateway and policy contracts exist.

| Task | Status |
| --- | --- |
| Define wrapper interface | planned |
| Implement task-scoped authorization wrapper | planned |
| Implement permission isolation wrapper | planned |
| Implement credential injection wrapper | planned |
| Implement HITL approval verification wrapper | planned |
| Add wrapper failure tests | planned |

## Phase 5: Durable State and Replay

Objective: support long-running workflows, approval waits, crash recovery, and deterministic replay.

| Task | Status |
| --- | --- |
| Define execution state model | planned |
| Implement pending approval persistence | planned |
| Implement replay token handling | planned |
| Implement idempotency locks | planned |
| Implement exactly-once checks | planned |
| Add replay validation tests | planned |

## Governance Maintenance Tasks

| Task | Status |
| --- | --- |
| Keep TASKS.md updated as work completes | planned |
| Add ADR entries for architecture-impacting decisions | planned |
| Update acceptance criteria when behavior changes | planned |
| Re-run governance validation before implementation milestones | planned |
