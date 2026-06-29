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

## Active Phase 3 Work

Phase 3 work is grouped by the purpose it serves.

### Execute

| Task | Status |
| --- | --- |
| Add local sandbox L1 mutation wrapper | complete |
| Add broader mutation-capable execution path | planned |

### Govern

| Task | Status |
| --- | --- |
| Add credential injection boundary | planned |
| Add approval workflow boundary | planned |

### Recover

| Task | Status |
| --- | --- |
| Add durable execution state | planned |
| Add replay and recovery behavior | planned |

### Prove

| Task | Status |
| --- | --- |
| Add execution evidence | planned |
| Add replay evidence | planned |
| Add governed execution integration tests | planned |

Completed phase history remains below for traceability and repository verification.

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

Objective: implement the minimum local gateway path after protocol contracts are stable.

Status: complete.

| Task | Status |
| --- | --- |
| Complete local Rust Gateway MVP | complete |
| Validate request and response contracts in Rust | complete |
| Verify local policy bundle structure, checksums, and signatures | complete |
| Evaluate local policy and risk matrix decisions | complete |
| Emit structured JSON response and audit evidence | complete |
| Persist optional append-only local JSONL audit records | complete |
| Complete Phase 2 exit review and v0.2.0 readiness check | complete |

## Phase 3: Governed Execution Engine

Objective: execute real AI actions safely under governance.

Status: in progress.

Completed foundation work:

| Task | Status |
| --- | --- |
| Add wrapper dispatcher | complete |
| Add wrapper execution boundary | complete |
| Execute local L0 health.check wrapper | complete |
| Execute local sandbox L1 mutation wrapper | complete |
| Add execution lifecycle state machine | complete |
| Add structured error reporting | complete |
| Add execution authorization boundary | complete |
| Add credential class boundary | complete |

## Phase 4: Platform Capabilities

Objective: expose and operate the governed execution engine through platform boundaries after runtime behavior is stable.

| Task | Status |
| --- | --- |
| Add HTTP API boundary | planned |
| Add service deployment model | planned |
| Add runtime configuration model | planned |
| Add operational observability | planned |
| Add plugin or wrapper extension architecture | planned |
| Add orchestrator integration references | planned |
| Add desktop UI only after runtime behavior is stable | planned |

## Phase 5: Production Hardening

Objective: prepare AEGIS for production-oriented evaluation.

| Task | Status |
| --- | --- |
| Add production PKI or trust distribution | planned |
| Add remote policy distribution | planned |
| Add high-availability deployment guidance | planned |
| Add performance and load testing | planned |
| Complete security review | planned |
| Add fuzz testing for critical parsers and boundaries | planned |
| Add compatibility guarantees | planned |
| Add release engineering | planned |
| Add operational documentation | planned |

## Governance Maintenance Tasks

| Task | Status |
| --- | --- |
| Keep TASKS.md updated as work completes | planned |
| Add ADR entries for architecture-impacting decisions | planned |
| Update acceptance criteria when behavior changes | planned |
| Re-run governance validation before implementation milestones | planned |
