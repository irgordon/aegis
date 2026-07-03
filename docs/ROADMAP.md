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
- release readiness before scope expansion

## Current Repository State

AEGIS now has a working local Gateway MVP.

For a new reader, this means the repository can prove the gateway decision path locally: it reads a structured request, verifies a local policy bundle, evaluates simple policy, returns allowed, denied, or pending, records audit evidence, and can append a local audit record.

The Phase 3 local governed execution foundation is complete after reclassification. The local runtime can dispatch and execute the built-in read-only `health.check` wrapper after policy allows it, execute the local sandbox mutation wrapper `sandbox.note.write`, pass a safe local development credential handle for that sandbox mutation, return structured error reports, expose lifecycle state, optionally append a local execution state log, inspect state evidence, and generate bounded recovery plans.

It does not yet execute real external actions, replay execution, approval workflow, production credential providers, or HTTP service behavior. Those concerns now belong to later phases instead of the completed local execution foundation.

Phase 4 has a Tauri shell and Slint operator surface. The CLI remains a support surface for validation, inspection, testing, and automation. The UI scaffold renders fixture-backed sample evidence and can request fixed live `health.check` backend evidence through a narrow read-only IPC command. It does not submit arbitrary gateway requests, execute mutation wrappers, inspect live state logs, generate live recovery plans, define broad IPC command surfaces, or provide authority.

`v0.4.0` is complete, tagged, and pushed as a local-only, pre-alpha, source-oriented release.

Post-`v0.4.0` distribution work produced the first public downloadable Developer Preview in `v0.4.1`, starting with unsigned, not-notarized macOS archives and a combined `SHA256SUMS` manifest. The next implementation work should verify developer download and portable launch behavior, then expand validation toward later platforms.

For contributors, the backlog has been reorganized around improving the public Developer Preview. Completed Phase 2, Phase 3, and Phase 4 foundation work is no longer an active task list. Active work remains in Phase 5 developer distribution.

For engineers and architects, execution remains the primary architectural concern. The UI remains non-authoritative. It renders backend evidence; it does not evaluate policy, authorize execution, inject credentials, dispatch wrappers, or invent lifecycle or recovery state.

## Phase 0: Governance Baseline

### Status
Complete.

### Objective
Established the repository as a documentation-driven engineering project.

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

### Status
Complete.

### Objective
Defined the stable protocol contracts used by orchestrators, gateways, policy engines, wrappers, and audit systems.

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
Implemented the minimum local gateway capable of receiving requests, validating schema, verifying a local policy bundle, evaluating simple local policy, returning deterministic decisions, emitting audit evidence, and optionally persisting local audit records.

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
Implemented governed local execution under policy, authorization, credential, audit, and state boundaries.

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
- replay and recovery execution move to a later phase
- approval workflow moves to a later phase
- production credential providers move to a later phase
- HTTP service, deployment, observability, and plugin architecture move to a later phase

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
Prepared backend evidence and documentation for a graphical operator surface without giving the UI authority.

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
Complete for `v0.4.0`.

### Objective
Rendered governed execution evidence in a non-authoritative graphical desktop operator surface.

### Deliverables
- Tauri desktop shell with Slint graphical UI layer
- sample execution timeline from fixture evidence
- sample status cards from fixture evidence
- normalized error cards from fixture evidence
- sample recovery inspection and recovery planning cards from fixture evidence
- minimal IPC data bridge
- live read-only runtime evidence rendering

### Phase 4 Progression
Completed `v0.4.0` foundation:

- static Tauri shell with Slint landing window
- fixture-backed Slint execution timeline, status cards, normalized error card, and recovery inspection and planning cards
- narrow read-only `get_health_check_evidence` IPC command
- live backend `health.check` evidence rendering for current status cards and timeline fields
- executable v0.4.0 release validation gate
- v0.4.0 local-only release tag

Completed post-`v0.4.0` distribution planning sequence:

1. Defined the post-v0.4.0 distribution plan.
2. Selected the first downloadable artifact targets.
3. Added the draft artifact workflow and checksum validation after maintainers approved implementation scope.
4. Started Phase 5 developer distribution after draft artifacts were portable and checksums verified.

### Exit Criteria
- UI displays runtime state without owning policy decisions
- UI consumes backend evidence and cannot bypass gateway execution logic
- UI renders sample evidence before live IPC
- live evidence rendering is read-only
- graphical timelines, status cards, and error cards preserve backend meaning
- sample recovery inspection and recovery plan labels do not imply recovery or replay execution

## Release Track: Minimum Usable Local Release

### Status
Complete for `v0.4.0`.

### Objective
Created the smallest local-only AEGIS release that a user can build, launch, understand, and use safely.

### Scope
- launch the Tauri plus Slint desktop app
- run the local gateway with the included policy bundle
- execute `health.check`
- execute `sandbox.note.write` against a local sandbox directory
- write local audit and state JSONL evidence
- inspect state evidence
- generate read-only recovery plans
- explain limitations plainly

### Deferred
- replay execution
- approval workflow
- production credential providers
- HTTP or service deployment
- enterprise packaging
- installer generation
- signing
- auto-update
- external integrations

### Reference
See `docs/RELEASE_PATH.md`.

Post-`v0.4.0` distribution planning is tracked in `docs/RELEASE_DISTRIBUTION_PLAN.md`.

The selected first artifact targets are tracked in `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md`.

### Release Governance

Phase progression now follows release readiness rather than feature accumulation.

After `v0.4.0`, every new release task should answer:

```text
Does this preserve v0.4.0's safety boundaries while moving AEGIS toward a validated downloadable release?
```

If the answer is no, defer the work.

Downloadable artifact publishing is gated by validation for every release. Signing, notarization, installers, and auto-update remain future work until deliberately scheduled.

## Phase 5: Developer Distribution

### Objective
Deliver the first downloadable developer-preview release without weakening the governance properties established in earlier phases.

The primary Phase 5 question is:

```text
How can another developer download, verify, launch, and evaluate AEGIS without needing the maintainer's machine or source checkout?
```

This phase is about distribution engineering. It is not a new governance phase.

### Deliverables
Completed:

- stripped or remapped debug/source build paths where practical
- draft GitHub Release workflow
- draft GitHub Release verification
- first downloadable developer-preview release
- clear unsigned developer-preview warnings
- SHA-256 checksum verification instructions

Remaining:

- cross-platform artifact validation
- developer download verification
- portable launch verification

### Exit Criteria
- a developer can download AEGIS from GitHub
- downloaded artifacts verify successfully
- desktop application launches without a source checkout
- gateway launches without a source checkout
- artifacts are portable, reproducible, explainable, verifiable, and disposable
- artifacts are unsigned and clearly identified as developer previews
- no maintainer-specific environment is required
- release validation remains deterministic
- release governance remains intact

### Non-Goals
- installers
- signing
- notarization
- auto-update
- production credentials
- replay execution
- approval workflow
- enterprise deployment
- cloud distribution
- plugin ecosystem
- database backends

## Phase 6: Developer Experience

### Objective
Improve the experience of evaluating AEGIS after the first downloadable developer preview exists.

This phase should make the product easier to understand, launch, inspect, and troubleshoot without adding new execution authority.

### Deliverables
- clearer first-run and launch guidance
- read-only audit, state, recovery inspection, and recovery plan views where they reduce evaluation friction
- developer-focused troubleshooting notes
- artifact validation notes based on real download testing
- improved local examples and evidence walkthroughs

### Exit Criteria
- a new developer can evaluate the downloaded artifact without maintainer-specific knowledge
- UI and CLI evidence remain consistent
- read-only views do not imply authority or execution
- troubleshooting guidance is clear and current
- no production deployment, replay execution, approval workflow, or credential provider work is introduced

## Phase 7: Production Distribution

### Objective
Move from unsigned developer-preview archives toward normal platform distribution.

This phase should improve trust and installability after the developer-preview path is proven.

### Deliverables
- signed checksum manifests where scheduled
- code signing plan and implementation
- macOS notarization plan and implementation
- installer or app bundle packaging where scheduled
- auto-update decision and implementation only if explicitly approved
- production distribution documentation

### Exit Criteria
- artifacts are signed or clearly documented as unsigned
- notarization status is explicit
- installer behavior is documented and validated if installers exist
- release assets remain reproducible and verifiable
- distribution changes do not alter gateway authority, policy behavior, or runtime governance

## Phase 8: Runtime and Platform Expansion

### Objective
Expand runtime governance and platform capabilities after developer distribution and production distribution boundaries are stable.

### Deliverables
- replay eligibility report
- replay dry-run plan
- constrained replay execution
- audit retry path
- recovery execution guardrails
- approval workflow boundary
- approval evidence and state persistence
- production credential provider boundary
- provider compatibility checks
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
- replay uses stored intent and does not call the planning layer
- recovery actions preserve fail-closed behavior
- audit retry is bounded and traceable
- pending actions do not execute before valid approval
- approvals are attributable, scoped, and auditable
- credential providers do not expose secrets to agents, stdout, audit logs, state logs, or UI
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
