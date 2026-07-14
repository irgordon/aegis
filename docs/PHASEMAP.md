# AEGIS
# Phasemap v1.0

## Purpose

This document defines AEGIS engineering maturity gates and records release outcomes without binding a phase to a version number.

The roadmap describes direction.

The phasemap defines engineering maturity gates.

Release versions provide evidence of bounded outcomes achieved within or across those gates.

## Phase and Release Principle

Engineering phases communicate maturity and sequencing.

Release versions communicate validated outcomes. A phase may span multiple releases, and a release does not automatically close a phase.

## Current Release Truth

- Latest published release: `v0.4.1 Developer Preview`
- Current development target: `v0.4.2 Developer Preview Refresh`
- Active engineering phase: `Phase 5 Developer Distribution`
- Active repository priority: `P0 Repository Truth`

## Current Phase Model

| Phase | Name | Status | Purpose | Current Tasks | Deferred Tasks |
| --- | --- | --- | --- | --- | --- |
| 0 | Governance Baseline | Complete | Establish documentation-driven governance. | None. | None. |
| 1 | Contracts and Architecture Foundation | Complete | Define schemas, examples, and compatibility contracts. | None. | None. |
| 2 | Local Gateway MVP | Complete | Prove local governed request-to-response behavior. | None. | None. |
| 3 | Governed Execution Engine | Complete for local built-in execution foundation. | Prove safe local wrapper execution under policy, authorization, credential, audit, and state boundaries. | None. | Replay execution, approval workflow, and production credential providers. |
| 3.5 | UI-Ready Evidence and Documentation | Complete | Make backend evidence understandable and renderable by a future UI. | None. | Live UI rendering and IPC. |
| 4 | Graphical Operator Surface | Complete for local release. | Render backend evidence in a non-authoritative Tauri plus Slint desktop UI. | None. | HTTP service, platform deployment, replay execution, approval workflow, and production credential providers. |
| 5 | Developer Distribution | Active | Deliver portable, verifiable Developer Preview artifacts. | Complete P0 Repository Truth, publish `v0.4.2`, then validate Windows x64 and Linux x64. | Installers, signing, notarization, auto-update, replay execution, approval workflow, and production credentials. |
| 6 | Developer Experience | Not started | Improve evaluation, launch, troubleshooting, and read-only evidence review after distribution works. | None. | Replay execution, approval workflow, production credentials, and production distribution. |
| 7 | Production Distribution | Not started | Add signed or clearly bounded production-style distribution after developer-preview artifacts are proven. | None. | Runtime expansion and platform hardening. |
| 8 | Runtime and Platform Expansion | Not started | Add recovery, approval, credential provider, service, deployment, and operational maturity. | None. | Post-1.0 ecosystem tracks. |

## Engineering Gate Summary

| Phase | Entry Criteria | Exit Criteria |
| --- | --- | --- |
| 5 | Phase 4 local operator surface and release evidence are complete. | Declared Developer Preview platforms are portable, verifiable, documented, and release-governed. |
| 6 | Developer distribution works without maintainer-specific knowledge. | Evaluation, launch, troubleshooting, and read-only evidence review are clear without adding runtime authority. |
| 7 | Developer Preview distribution is stable enough for platform trust work. | Production-style distribution controls are documented, validated, and do not alter runtime governance. |
| 8 | Developer and production distribution boundaries are stable. | Runtime recovery, approval, credential provider, service, deployment, and operational controls are tested, documented, and reproducible. |

## Release Outcome Summary

| Version | Outcome | Status |
| --- | --- | --- |
| `v0.4.1` | First public macOS Developer Preview. | Published historical release. |
| `v0.4.2` | Developer Preview Refresh with repository truth and post-tag first-run improvements. | Current development target. |
| `v0.5.0` | Windows x64 and Linux x64 Developer Preview. | Planned Phase 5 outcome. |

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
Render governed execution evidence in a non-authoritative graphical desktop operator surface and establish the minimum usable local release.

### Status
Complete for local-only `v0.4.0`.

### Required Capabilities
- Tauri desktop shell with Slint graphical UI layer
- static Tauri + Slint landing scaffold
- Slint-rendered sample execution timeline from fixture evidence
- Slint-rendered sample status cards from fixture evidence
- Slint-rendered normalized error cards from fixture evidence
- Slint-rendered sample recovery inspection and recovery planning cards from fixture evidence
- documented minimum usable local release path
- minimal IPC data bridge
- live read-only runtime evidence rendering
- executable release validation gate
- formal release readiness review
- annotated `v0.4.0` tag pushed to origin

### Exit Criteria
- minimum usable local release checklist passes
- local gateway commands for `health.check`, `sandbox.note.write`, audit evidence, state evidence, recovery inspection, and recovery planning are documented and verified
- desktop app launch command is documented and verified
- UI displays runtime state without owning policy decisions
- UI consumes backend evidence and cannot bypass gateway execution logic
- UI renders sample evidence before live IPC
- live evidence rendering is read-only
- graphical timelines, status cards, and error cards preserve backend meaning
- sample recovery inspection and recovery plan labels do not imply recovery or replay execution
- release remains local-only, source-oriented, and pre-alpha
- downloadable artifacts, installers, signing, notarization, and GitHub Release publishing are deferred

## v0.4.x: Post-v0.4.0 Distribution Planning

### Purpose
Plan a safe path from source-only local release to downloadable developer-preview artifacts.

### Status
Complete as Phase 5 input.

### Required Planning Outputs
- release distribution plan
- target platform candidates
- staged artifact types
- artifact naming convention
- checksum and integrity requirements
- signing and notarization sequencing
- GitHub Release workflow outline
- open decisions for first downloadable artifacts
- first downloadable artifact target decision

### Exit Criteria
- `docs/RELEASE_DISTRIBUTION_PLAN.md` exists
- `v0.4.0` remains source-only and unchanged in scope
- target platforms are planned, not promised
- future artifacts require SHA-256 checksums before publication
- signing, notarization, installers, and auto-update remain deferred until scheduled
- no release automation or artifact publishing is implemented by the planning task
- first downloadable developer-preview version is selected as `v0.4.1`
- first platform stage is selected as macOS arm64 and macOS x64
- first artifact format is archive-based, not installer-based
- draft GitHub Release publishing mode is selected for future implementation
- manual draft artifact workflow builds GitHub Actions workflow artifacts only

## v0.4.1: Developer Distribution

### Purpose
Deliver and verify the first downloadable Developer Preview.

This milestone should let another developer download, verify, launch, and evaluate AEGIS without the maintainer machine or a source checkout.

### Status
Published historical release.

### Required Capabilities
Completed:

- stripped or remapped debug/source build paths where practical
- draft GitHub Release workflow
- draft GitHub Release verification
- first downloadable developer-preview release
- clear unsigned developer-preview warnings
- checksum verification instructions
- developer download verification
- portable launch verification

Not included in the immutable release:

- bundled `health.check` request fixture
- conventional gateway `--help` output
- corrected desktop Developer Preview identity

Those improvements exist on the current development branch and target `v0.4.2`.

### Exit Criteria
- AEGIS can be downloaded from GitHub
- artifact SHA-256 checksums verify
- desktop application launches from extracted artifact contents
- gateway launches from extracted artifact contents
- no local source checkout is required
- no maintainer-specific environment is required
- artifacts are portable, reproducible, explainable, verifiable, and disposable
- artifacts are unsigned and clearly identified as developer previews
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

## v0.4.2: Developer Preview Refresh

### Purpose
Publish the post-`v0.4.1` first-run improvements under the Release Truth invariant.

### Status
Current development target after P0 Repository Truth closes.

### Required Capabilities
- reconciled latest-release and current-development documentation
- unified release-facing product version identity
- bundled safe `health.check` request fixture
- conventional gateway `--help` output
- explicit current-development desktop identity
- desktop validation in normal CI
- direct annotated-tag dispatch verification

### Exit Criteria
- P0 Repository Truth is complete
- public documentation describes the release artifact accurately
- package, desktop, artifact, tag, and changelog versions agree
- checksums verify
- gateway and desktop launch from extracted artifacts
- the immutable `v0.4.1` release remains unchanged

## v0.5.0: Windows and Linux Developer Preview

### Purpose
Add Windows x64 and Linux x64 Developer Preview artifacts after the refreshed macOS path is stable.

### Status
Planned Phase 5 release outcome.

### Required Capabilities
- Windows x64 archive build and validation
- Linux x64 archive build and validation
- platform-specific artifact smoke tests
- combined checksum and release-note coverage
- portable gateway and desktop launch evidence

### Exit Criteria
- Windows x64 and Linux x64 artifacts are reproducible and verifiable
- artifacts do not require a source checkout or maintainer machine
- platform limitations are explicit
- release governance remains intact

## Phase 6: Developer Experience

### Purpose
Improve the experience of evaluating AEGIS after the first downloadable developer preview exists.

### Required Capabilities
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

### Purpose
Move from unsigned developer-preview archives toward normal platform distribution.

### Required Capabilities
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

### Purpose
Expand runtime governance and platform capabilities after developer distribution and production distribution boundaries are stable.

### Required Capabilities
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
