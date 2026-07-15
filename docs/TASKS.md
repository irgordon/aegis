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

## Current Release Truth

- Latest published release: `v0.4.1 Developer Preview`
- Current development target: `v0.4.2 Developer Preview Refresh`
- Active engineering phase: `Phase 5 Developer Distribution`
- Active repository priority: `P1 Complete Phase 5 Developer Distribution`

The immutable `v0.4.1` artifacts contain two macOS archives and a combined `SHA256SUMS`. They do not contain the later bundled request fixture, conventional CLI help, or corrected desktop identity now present on the development branch.

## Completed P0 Work: Repository Truth

P0 paused release expansion until the repository had one authoritative
project, release, and version truth.

| Task | Status |
| --- | --- |
| Record the Release Truth architecture decision | complete |
| Add the Release Truth invariant | complete |
| Add the machine-readable release-truth record | complete |
| Define the unified product version policy | complete |
| Reconcile release-sensitive documentation | complete |
| Reconcile roadmap, phasemap, and task status | complete |
| Align Cargo, Tauri, changelog, and UI development versions | complete |
| Reject duplicate and conflicting task states | complete |
| Add release-truth checks to repository verification | complete |
| Add desktop validation to normal CI | complete |
| Complete P0 validation and exit review | complete |

## Active P1 Work: Complete Phase 5 Developer Distribution

`v0.4.0` is complete, tagged, and pushed as a local-only source release.

P1 proceeds in strict order. First, validate and publish the bounded macOS
`v0.4.2 Developer Preview Refresh` outcome. Then validate Windows x64 and Linux
x64 artifacts as the separate `v0.5.0` outcome. Release versions remain
outcomes rather than phase containers.

The `v0.4.2` tag and GitHub Release are prohibited until the draft artifact
evidence gate passes. The immutable `v0.4.1` tag and release must not change.

The draft artifact workflow is on `origin/main`. Manual workflow reviews produced inspectable macOS workflow artifacts. The combined `SHA256SUMS` manifest now covers both macOS draft archives and verifies successfully in workflow artifacts.

The environment-coupling audit found a release-blocking desktop artifact issue: live evidence resolved the policy bundle through a GitHub runner source path embedded at build time. Source changes now prefer an artifact-relative bundled policy bundle. Artifact-level verification confirmed the blocker is resolved in the rerun artifacts.

Artifact-level workflow review confirms release binary path remapping is resolved for runtime portability. One Tauri-generated desktop context string remains as deferred release hygiene.

The historical draft GitHub Release workflow was manual-only, required the
existing `v0.4.1` tag and a matching checkout, created or updated a draft
prerelease for maintainer review, and refused to modify a non-draft release.
The current-development workflow preserves those safeguards and targets a new
`v0.4.2` tag.

The first public GitHub Release exists for `v0.4.1`. It remains a prerelease Developer Preview. It contains the two macOS archives and the combined `SHA256SUMS` manifest. Local checksum verification passed before and after publication.

Current-development artifact workflows stage a safe `health.check` request fixture at `examples/health-check-request.json`. The published `v0.4.1` artifacts do not contain that fixture. It targets `v0.4.2`.

The gateway now treats `--help` and `-h` as conventional help requests. Help output points first-time users to the bundled `health.check` smoke test and returns success instead of a structured invalid-arguments error.

The desktop first-run review is complete on the current development branch. The desktop launches, communicates governed evidence, distinguishes current `v0.4.2` development from the latest `v0.4.1` release, and uses a neutral no-error state when live health-check evidence contains no error report.

A direct workflow dispatch from the annotated `v0.4.1` tag exposed a tag-fetch conflict in the workflow guard. The successful draft release run used `main` at the exact same commit as the `v0.4.1` tag, and the workflow guard verified the checkout matched the tag.

The draft release tag guard on `main` now avoids broad tag fetching, resolves the annotated `v0.4.1` tag target commit, and compares that target to `HEAD`. This preserves fail-closed tag verification for future release tags.

Live verification from the existing `v0.4.1` tag remains blocked because GitHub ran the workflow definition stored at the already-created tag. That tag still contains the older broad tag-fetch guard. Fixing that exact tag would require moving or recreating `v0.4.1`, which release rules prohibit. This historical caveat does not affect the published Developer Preview artifacts. The hardened guard applies to future tags created after this change.

Phase 5 asks:

```text
How can another developer download, verify, launch, and evaluate AEGIS without needing the maintainer's machine or source checkout?
```

Every new Phase 5 task should identify whether it completes a distribution capability, improves developer usability, removes a distribution blocker, improves artifact quality, or reduces release engineering debt.

Tasks that satisfy none should be deferred until Phase 6 or later.

Release impact:

- [ ] Completes a distribution capability
- [ ] Improves developer usability
- [ ] Removes a distribution blocker
- [ ] Improves artifact quality
- [ ] Reduces release engineering debt

If no box is checked, defer the work until Phase 6 or later.

| Task | Status |
| --- | --- |
| Strip or remap source paths in developer-preview binaries | complete |
| Verify release binary source path reduction in workflow artifacts | complete |
| Validate artifact naming and checksum generation | complete |
| Add draft GitHub Release workflow | complete |
| Verify draft GitHub Release | complete |
| Harden direct annotated-tag dispatch for future draft release workflow tags | complete |
| Developer download verification | complete |
| Portable launch verification | complete |
| Re-run desktop first-screen validation from an active local display | complete |
| Distinguish current development from latest release in desktop identity | complete |
| Improve desktop no-error empty state | complete |
| Add desktop next-step cue for the gateway smoke test | complete |
| Add artifact-only gateway smoke-test request or guidance | complete |
| Improve gateway first-run help output | complete |
| Draft v0.4.1 developer-preview release notes | complete |
| GitHub Release publishing | complete |
| Publish first unsigned developer-preview build | complete |
| Validate v0.4.2 release truth against main | complete |
| Build v0.4.2 macOS arm64 draft artifact | complete |
| Build v0.4.2 macOS x64 draft artifact | complete |
| Verify v0.4.2 bundled health-check fixture | complete |
| Verify v0.4.2 desktop identity and neutral empty state | blocked |
| Verify v0.4.2 gateway help output | complete |
| Verify v0.4.2 combined SHA256SUMS | complete |
| Verify v0.4.2 portable launch from extracted artifacts | complete |
| Verify v0.4.2 annotated-tag guard | complete |
| Record v0.4.2 artifact evidence | blocked |
| Publish immutable v0.4.2 without modifying v0.4.1 | planned |
| Validate v0.5.0 Windows x64 artifacts | planned |
| Validate v0.5.0 Linux x64 artifacts | planned |
| Complete Phase 5 exit review | planned |

The [v0.4.2 draft artifact evidence](releases/draft-artifact-evidence-v0.4.2.md)
records a BLOCKED result because the remote macOS display session could not
provide the required visual desktop evidence. Tagging and publication remain
prohibited.

### P1 Exit Gate

- `v0.4.2` is published and verified only after its artifact evidence passes
- every platform declared for Phase 5 has validated artifacts
- a new evaluator can download, verify, extract, launch, and run the bundled
  smoke test
- Phase 5 is marked complete only after the exit review passes

### Bounded Release Workflow Maintenance

These tasks are tracked separately from artifact evidence and are not current
`v0.4.2` release blockers.

| Task | Status |
| --- | --- |
| Upgrade GitHub Actions away from Node 20 runtimes | in_progress |
| Validate the macos-latest runner migration and choose bounded labels | in_progress |
| Enforce approved workflow actions and runner labels | in_progress |

The [GitHub Actions maintenance review](releases/github-actions-maintenance-review.md)
records the selected action majors, runner labels, security review, artifact
boundaries, and release-candidate isolation rule.

Completed inputs for Phase 5:

- `v0.4.0` local source release
- post-`v0.4.0` distribution plan
- first downloadable artifact target decision
- draft artifact workflow
- artifact portability audit
- desktop artifact policy bundle path fix
- combined `SHA256SUMS` manifest
- artifact-level checksum verification
- release binary source path remapping and debuginfo stripping
- artifact-level path-remapping verification
- draft GitHub Release workflow and static boundary tests
- draft release asset-name validation before release creation
- `v0.4.1` annotated tag
- `v0.4.1` GitHub Release with expected macOS archives and `SHA256SUMS`
- local checksum verification of draft release assets
- hardened annotated-tag dispatch guard on `main` for future draft GitHub Release workflow tags
- published `v0.4.1` Developer Preview GitHub Release
- post-publish checksum verification of unchanged `v0.4.1` assets
- developer download and portable launch review for public `v0.4.1` assets
- current-development artifact-only `health.check` gateway smoke-test fixture and guidance targeting `v0.4.2`
- current-development gateway `--help` and `-h` output targeting `v0.4.2`
- desktop first-run review comparing public `v0.4.1` with current development
- current-development desktop identity and neutral no-error state targeting `v0.4.2`

Deferred from Phase 5:

```text
Installers, signing, notarization, auto-update, production credentials,
replay execution, approval workflow, enterprise deployment, cloud distribution,
plugin ecosystem, database backends, Windows ARM64, and Linux ARM64 remain
deferred.
```

## Planned P2-P5 Work

### P2: Deliver Phase 6 Developer Experience

| Task | Status |
| --- | --- |
| Improve first-run guidance from evaluation evidence | planned |
| Add clear troubleshooting paths | planned |
| Add read-only audit and state evidence views | planned |
| Add read-only recovery inspection and recovery plan views | planned |
| Complete the UI Integrity Review before broader UI work | planned |
| Improve navigation only where evaluation evidence proves confusion | planned |
| Verify the desktop remains evidence-only | planned |

Exit gate: a new evaluator can understand, launch, inspect, and troubleshoot
AEGIS independently, and the UI remains evidence-only.

### P3: Deliver Phase 7 Production Distribution

| Task | Status |
| --- | --- |
| Sign checksum manifests | planned |
| Add code signing | planned |
| Add macOS notarization | planned |
| Add approved application bundles or installers | planned |
| Decide whether auto-update belongs in scope | planned |
| Keep signing material outside the repository | planned |
| Add reproducible release checks | planned |
| Validate installer and upgrade behavior | planned |

Exit gate: distributed artifacts are signed, verifiable, platform-appropriate,
and installable without changing gateway authority.

### P4: Deliver Phase 8 Runtime Expansion

Document each contract before implementation and preserve this delivery order.

| Task | Status |
| --- | --- |
| Add approval persistence | planned |
| Add replay eligibility and dry-run evaluation | planned |
| Add bounded audit retry and replay | planned |
| Add production credential providers | planned |
| Add HTTP service and configuration boundaries | planned |
| Add operational observability | planned |
| Add extension and orchestrator integration | planned |
| Add trust and policy distribution | planned |
| Add high-availability capability | planned |
| Complete security fuzz load and compatibility validation | planned |

Exit gate: runtime expansion remains contract-first, deterministic,
fail-closed, and unavailable as a UI or transport authority bypass.

### P5: Stabilize v1.0

| Task | Status |
| --- | --- |
| Freeze public schemas and compatibility rules | planned |
| Close known operational and compatibility gaps | planned |
| Verify reproducible releases for v1.0 | planned |
| Complete security fuzz load upgrade and recovery testing | planned |
| Document v1.0 known limitations | planned |
| Complete the formal v1.0 readiness review | planned |

Exit gate: public contracts are stable, deployment and recovery paths are
proven, and v1.0 has no unresolved truth drift.

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

Status: complete for local built-in execution foundation.

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
| Add local credential injection boundary | complete |
| Add durable local execution state log | complete |
| Harden execution state log invariants | complete |
| Add execution recovery inspection | complete |
| Harden recovery inspection invariants | complete |
| Add recovery plan generation | complete |
| Harden recovery plan invariants | complete |

## Phase 3.5: UI-Ready Evidence and Documentation

Objective: prepare backend evidence and documentation for graphical rendering without granting UI authority.

Status: complete.

| Task | Status |
| --- | --- |
| Formalize docs/wiki knowledge base | complete |
| Review docs/wiki language and accuracy | complete |
| Define UI-renderable evidence contract | complete |
| Record Slint with Tauri UI direction | complete |

## Phase 4: Graphical Operator Surface

Objective: render backend evidence in a non-authoritative Tauri plus Slint desktop operator surface.

Status: complete for `v0.4.0`.

| Task | Status |
| --- | --- |
| Add Tauri shell with Slint UI scaffold | complete |
| Render execution timeline from sample evidence | complete |
| Render sample status cards from fixture evidence | complete |
| Render normalized error cards from sample evidence | complete |
| Harden sample evidence rendering invariants | complete |
| Render sample recovery inspection and planning cards | complete |
| Define minimum usable local release path | complete |
| Add v0.4.0 release readiness checklist | complete |
| Add executable v0.4.0 release validation script | complete |
| Define minimal IPC data bridge | complete |
| Render live read-only runtime evidence | complete |
| Harden read-only IPC evidence boundary | complete |
| Run v0.4.0 readiness review | complete |
| Define v0.4.0 visual design guidance | complete |
| Apply v0.4.0 visual design system | complete |
| Complete v0.4.0 visual readability review | complete |
| Prepare v0.4.0 release candidate | complete |
| Maintainer approval to tag v0.4.0 | complete |
| Create v0.4.0 tag | complete |
| Push v0.4.0 tag | complete |
| Finalize v0.4.0 local release | complete |
| Define post-v0.4.0 distribution plan | complete |
| Select first downloadable artifact targets | complete |
| Add draft artifact build workflow | complete |
| Refresh wiki for post-v0.4.0 distribution work | complete |
| Push local draft artifact workflow commit after workflow-scope authentication | complete |
| Review draft artifact workflow run | complete |
| Consolidate draft artifact checksum manifest | complete |
| Rerun draft artifact workflow and verify combined checksum manifest | complete |
| Render audit, state, recovery inspection, and recovery plan views read-only after v0.4.0 | planned |

## Governance Maintenance Tasks

| Task | Status |
| --- | --- |
| Keep TASKS.md updated as work completes | planned |
| Add ADR entries for architecture-impacting decisions | planned |
| Update acceptance criteria when behavior changes | planned |
| Re-run governance validation before implementation milestones | planned |
