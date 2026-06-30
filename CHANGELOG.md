# Changelog

All notable changes to AEGIS are documented in this file.

This changelog follows the Keep a Changelog structure and is governed by the AEGIS Definition of Done. Future releases shall update this file when repository behavior, documentation, schemas, policy contracts, or release artifacts change.

## [0.2.24] - 2026-06-30

### Changed

- Recorded Slint with Tauri as the intended future graphical UI direction.
- Clarified that the CLI remains a support surface and the backend remains authoritative for UI evidence.

## [0.2.23] - 2026-06-29

### Added

- Added UI evidence contract for future Tauri operator feedback, status cards, timelines, recovery planning display, and normalized error rendering.

### Changed

- Updated documentation maps and Phase 3 task tracking for UI-renderable backend evidence.

## [0.2.22] - 2026-06-29

### Changed

- Reviewed `/docs/wiki/` for language, tone, voice, grammar, accuracy, invariant alignment, and cognitive load.
- Clarified wiki wording for credential injection, future UI evidence consumption, fail-closed error conditions, and read-only recovery planning.
- Updated Phase 3 task tracking for wiki language and accuracy review.

## [0.2.21] - 2026-06-29

### Added

- Added `/docs/wiki/` as an AEGIS knowledge base covering overview, execution flow, major components, policy, authorization, credentials, wrappers, audit, state, recovery, errors, future UI feedback, architectural decisions, and contributor guidance.

### Changed

- Updated documentation governance to list the wiki as an explanatory knowledge base, not an authoritative replacement for governed documents.
- Updated Phase 3 task tracking for wiki formalization.

## [0.2.20] - 2026-06-29

### Changed

- Documented the Tauri graphical UI foundation and visual feedback model.
- Clarified that the CLI is a support surface, not the primary operator experience.
- Updated planning documents so Phase 3 preserves UI-ready backend evidence and Phase 4 owns Tauri UI implementation.

## [0.2.19] - 2026-06-29

### Added

- Added recovery plan invariant tests for terminal, corrupted, failed-closed, audit-failed, inspection-failed, deterministic, and secret-safe planning evidence.

### Fixed

- Fixed recovery plan handling for unknown recoverability status and inconsistent terminal status so ambiguous internal reports fail closed.

## [0.2.18] - 2026-06-29

### Added

- Added read-only recovery plan generation from inspected execution state evidence.
- Added `--plan-recovery` CLI mode for bounded future recovery classifications.
- Added recovery plan tests for terminal, corrupted, audit-failed, non-terminal, inspection-failed, and read-only behavior.

### Changed

- Updated audit documentation and Phase 3 task tracking for recovery planning.

## [0.2.17] - 2026-06-29

### Added

- Added recovery inspection invariant tests for corrupted, duplicated, reordered, terminal-state, and mixed state evidence.

### Fixed

- Fixed recovery inspection handling for inconsistent request, tool, and wrapper references within one execution.

## [0.2.16] - 2026-06-29

### Added

- Added read-only execution recovery inspection for local execution state logs.
- Added `--inspect-state` CLI mode for grouped lifecycle inspection without policy loading or wrapper execution.
- Added recovery inspection tests for terminal classification, recoverability classification, malformed state records, ordering errors, and read-only CLI behavior.

### Changed

- Updated audit documentation and Phase 3 task tracking for execution state inspection.

## [0.2.15] - 2026-06-29

### Added

- Added a repository code audit report for dead, stale, and legacy implementation paths.

### Fixed

- Corrected sandbox wrapper context evidence so credential injection is marked required when `sandbox.note.write` executes.
- Removed stale gateway scaffold status text and an unused execution state reference wrapper.
- Aligned architecture and roadmap documentation with current Phase 3 runtime capabilities.

## [0.2.14] - 2026-06-29

### Added

- Added a local development credential injection boundary with safe credential handle references.
- Added runtime, audit, and state evidence for local credential injection.
- Added credential injection tests for sandbox mutation, handle mismatch failures, secret absence, and wrapper boundary enforcement.

### Changed

- Updated architecture, trust boundary, audit, and task documentation for local credential handle injection.

## [0.2.13] - 2026-06-29

### Changed

- Adopted the 4MAT README structure.
- Added README communication standard to DOCUMENTATION.md.

## [0.2.12] - 2026-06-29

### Added

- Added execution state log invariant tests for pending decisions, invalid bundles, checksum mismatch, invalid signatures, wrapper lookup, authorization, credential boundary, terminal states, lifecycle ordering, known state names, and audit/state separation.

### Fixed

- Ensured pending policy decisions transition to the existing fail-closed terminal lifecycle state instead of stopping at policy evaluation.

### Changed

- Updated Phase 3 task tracking for execution state log invariant hardening.

## [0.2.11] - 2026-06-29

### Added

- Added optional append-only local JSONL execution state logging with `--state-log`.
- Added lifecycle transition records using the existing execution state model.
- Added fail-closed structured errors for execution state log write failures.
- Added execution state log tests for successful, denied, malformed, wrapper failure, audit failure, append, invalid path, JSON validity, and secret-free paths.

### Changed

- Updated audit documentation to distinguish audit logs from execution state logs.
- Updated Phase 3 task tracking for durable local execution state evidence.

## [0.2.10] - 2026-06-29

### Added

- Added built-in local L1 `sandbox.note.write` wrapper execution.
- Added sandbox path containment, idempotency, authorization, and credential boundary gates for local mutation.
- Added sandbox mutation evidence to runtime output and audit records.
- Added fail-closed sandbox wrapper tests for unsafe paths, missing idempotency, missing sandbox directory, policy denial, pending policy, and credential mismatch.

### Changed

- Updated the local development policy bundle to allow `sandbox.note.write`.
- Updated trust boundary, policy, audit, and task documentation for controlled local L1 mutation.

## [0.2.9] - 2026-06-29

### Added

- Added credential class boundary validation before wrapper dispatch.
- Added explicit wrapper credential requirements for governed execution.
- Added credential boundary evidence to runtime output and audit records.
- Added fail-closed credential boundary tests.

### Changed

- Updated trust boundary and audit documentation for credential class evidence.
- Updated Phase 3 task tracking for credential class boundary work.

## [0.2.8] - 2026-06-29

### Added

- Added execution authorization boundary between policy evaluation and wrapper dispatch.
- Added authorization evidence to runtime output and audit records.
- Added fail-closed authorization tests for wrapper, version, capability, scope, denied, and pending paths.

### Changed

- Updated trust boundary and audit documentation for execution authorization evidence.
- Updated Phase 3 task tracking for execution authorization boundary work.

## [0.2.7] - 2026-06-29

### Changed

- Added repository-wide documentation stability guidance for Phase 3.
- Clarified that CHANGELOG.md is the primary record of routine implementation progress.

## [0.2.6] - 2026-06-28

### Changed

- Normalized README.md as the stable public project entry point.
- Clarified documentation ownership rules for README.md and implementation progress.
- Updated architecture, roadmap, coding style, and task tracking to reflect the current Phase 3 baseline.

## [0.2.5] - 2026-06-28

### Added

- Added deterministic in-memory execution lifecycle state model.
- Added lifecycle evidence to runtime output and audit records.
- Added lifecycle transition tests for success, fail-closed, wrapper failure, and audit failure paths.

### Changed

- Updated architecture and audit documentation for execution lifecycle evidence.
- Updated Phase 3 task tracking for execution lifecycle modeling.

## [0.2.4] - 2026-06-28

### Added

- Added built-in local L0 `health.check` wrapper execution.
- Added wrapper execution evidence to runtime output and audit records.
- Added fail-closed tests for wrapper dispatch and execution failures.

### Changed

- Updated local development policy bundle to allow `health.check`.
- Updated Phase 3 task tracking for the first governed wrapper execution path.

## [0.2.3] - 2026-06-28

### Added

- Added structured gateway error reports with plain-language messages, reasons, and next actions.
- Added tests for fail-closed error reporting across request, policy, wrapper, audit, and runtime failures.

### Changed

- Updated local runtime output and audit records with bounded safe error evidence.
- Updated audit, API, and coding-style documentation for structured error reporting.
- Updated Phase 3 task tracking for structured error reporting.

## [0.2.2] - 2026-06-28

### Added

- Added wrapper dispatcher and execution boundary types.
- Added wrapper dispatcher contract tests for matched dispatch, bounded execution modes, and fail-closed wrapper failures.

### Changed

- Updated Phase 3 task tracking for wrapper dispatcher and execution boundary work.

## [0.2.1] - 2026-06-28

### Changed

- Established Phase 3 engineering principles.
- Updated architecture guidance to prioritize useful execution and reduced cognitive load.
- Reorganized Phase 3 planning around Execute, Govern, Recover, and Prove.

## [0.2.0] - 2026-06-28

### Changed

- Completed Phase 2 Gateway MVP.
- Completed Phase 2 exit review and v0.2.0 readiness alignment.
- Realigned roadmap around executable runtime development.
- Established Phase 3 priorities.
- Updated ROADMAP.md, PHASEMAP.md, TASKS.md, and README.md to reflect the post-Phase-2 repository state.

## [0.1.21] - 2026-06-28

### Added

- Added durable append-only local audit logging.
- Added JSONL audit persistence.
- Added audit persistence tests.

### Changed

- Updated the local runtime to optionally persist audit records.
- Updated audit documentation for local JSONL persistence.
- Updated Phase 2 task tracking for local audit logging.

## [0.1.20] - 2026-06-28

### Added

- Added local policy and risk matrix evaluation for verified policy bundles.
- Added fail-closed tests for missing, malformed, ambiguous, and unsupported policy state.
- Added policy evaluation evidence to runtime output and audit records.

### Changed

- Updated local development policy bundle fixture.
- Updated policy documentation for verified local policy evaluation.
- Updated Phase 2 task tracking for local policy evaluation.

## [0.1.19] - 2026-06-28

### Added

- Added cryptographic signature verification for local policy bundle checksum metadata.
- Added fail-closed tests for missing, malformed, and invalid policy bundle signatures.

### Changed

- Updated local policy bundle fixture signature metadata.
- Simplified README.md to clearly state pre-alpha status and current repository scope.
- Updated POLICY_DISTRIBUTION.md with checksum and signature verification behavior.
- Updated Phase 2 task tracking for policy bundle authenticity verification.

## [0.1.18] - 2026-06-28

### Added

- Added real SHA-256 checksum verification for local policy bundles.
- Added fail-closed tests for missing and mismatched policy bundle checksums.

### Changed

- Updated local policy bundle fixture checksum metadata.
- Updated README.md with checksum verification behavior and local bundle maintenance instructions.
- Updated policy distribution documentation to distinguish checksum verification from signature verification.
- Updated Phase 2 task tracking for policy bundle integrity verification.

## [0.1.17] - 2026-06-28

### Added

- Added local policy bundle loader and verifier.
- Added local development policy bundle fixture.
- Added policy bundle verification tests.
- Added runtime output propagation for verified policy bundle identity.

### Changed

- Updated README.md with policy bundle-backed local runtime usage.
- Updated Phase 2 task tracking for policy bundle verification.

## [0.1.16] - 2026-06-28

### Added

- Added a local Gateway MVP runtime entrypoint.
- Added structured JSON output containing gateway response and audit evidence.
- Added local runtime contract tests.

### Changed

- Updated README.md with local Gateway MVP usage.
- Updated Phase 2 task tracking for executable gateway behavior.

## [0.1.15] - 2026-06-28

### Added

- Added approval token contract models without approval workflow.
- Added audit evidence propagation for approval context references.
- Added approval token contract tests.

### Changed

- Updated Phase 2 task tracking for approval token contract modeling.

## [0.1.14] - 2026-06-28

### Added

- Added execution identity contract models without execution ID generation.
- Added audit evidence propagation for execution identity references.
- Added execution identity contract tests.

### Changed

- Updated Phase 2 task tracking for execution identity contract modeling.

## [0.1.13] - 2026-06-28

### Added

- Added wrapper configuration contract models without wrapper execution.
- Added audit evidence propagation for wrapper configuration references.
- Added wrapper configuration contract tests.

### Changed

- Updated Phase 2 task tracking for wrapper configuration contract modeling.

## [0.1.12] - 2026-06-28

### Added

- Added a minimal policy decision adapter interface for validated gateway requests.
- Added policy adapter contract tests for allowed, denied, pending, and fail-closed adapter outcomes.

### Changed

- Updated Phase 2 task tracking for policy adapter interface work.

## [0.1.11] - 2026-06-28

### Added

- Added typed idempotency contract models for caller-supplied gateway idempotency context.
- Added gateway idempotency tests for L1, L2, and L3 mutation-capable request evidence.

### Changed

- Updated audit evidence details to reference supplied idempotency context where applicable.
- Updated Phase 2 task tracking for idempotency contract modeling.

## [0.1.10] - 2026-06-28

### Added

- Added UI-DESIGN.md to define required UI integrity review standards for future frontend work.

### Changed

- Updated AGENTS.md and DOCUMENTATION.md to reference UI design review requirements.

## [0.1.9] - 2026-06-28

### Added

- Added a minimal internal Gateway entrypoint boundary.
- Added entrypoint contract tests for malformed, unsupported, allowed, denied, and pending request outcomes.

### Changed

- Updated Phase 2 task tracking for gateway entrypoint coordination.

## [0.1.8] - 2026-06-28

### Added

- Added schema-backed gateway validation pipeline for ToolCallRequest JSON input.
- Added fail-closed denial evidence for malformed and unsupported request validation.

### Changed

- Updated Phase 2 task tracking for request validation pipeline work.

## [0.1.7] - 2026-06-28

### Changed

- Hardened ARCHITECTURE.md execution semantics for identity binding, approval liveness, policy bundle pinning, wrapper determinism, replay fidelity, and idempotency.

## [0.1.6] - 2026-06-28

### Added

- Added deny-by-default handling for unsupported tools using an explicit allowlist.
- Added gateway safety tests for unsupported-tool denial responses and audit evidence.

### Changed

- Updated Phase 2 task tracking for deny-by-default unsupported tool handling.

## [0.1.5] - 2026-06-28

### Changed

- Aligned DOCUMENTATION.md terminology with ARCHITECTURE.md.
- Clarified policy bundle contents and top-level documentation-related directories.

## [0.1.4] - 2026-06-28

### Added

- Added minimal Rust audit record builder for gateway decisions.
- Added audit contract tests for allowed, denied, and pending gateway responses.

### Changed

- Updated Phase 2 task tracking for audit evidence construction.

## [0.1.3] - 2026-06-28

### Added

- Added deterministic mapping from explicit policy decisions to ToolCallResponse values.
- Added gateway contract tests for allowed, denied, and pending response mapping.

### Changed

- Updated Phase 2 task tracking for policy decision and response mapping work.

## [0.1.2] - 2026-06-28

### Added

- Added schema-backed Rust request and response models.
- Added Rust fixture tests for valid and invalid ToolCallRequest and ToolCallResponse examples.

### Changed

- Updated Phase 2 task tracking for request and response contract work.

## [0.1.1] - 2026-06-28

### Added

- Accepted the Rust Gateway MVP runtime decision.
- Added the initial Rust Gateway MVP crate scaffold.
- Added gateway contract tests for bounded response states.
- Added Rust formatting, linting, and test validation to CI.

### Changed

- Updated repository status for Phase 2 Gateway MVP scaffolding.
- Updated Phase 2 task state for runtime selection and gateway entrypoint scaffolding.

## [0.1.0] - 2026-06-28

### Governance Foundation

### Added

- Operating Doctrine
- PRD
- Architecture
- Invariants
- Architectural Principles
- Documentation Governance
- Coding Style
- User Flows
- Acceptance Criteria
- Roadmap
- Phasemap
- Validation
- Security Model
- Threat Model
- Trust Boundaries
- Policy Engine
- Policy Distribution
- Audit Logging
- Orchestrator FSM Contract
- API Specification
- Runtime Evidence
- Test Strategy
- ADR
- Release Process
- Tasks

### Changed

- README expanded
- Governance hierarchy completed

## [0.0.0] - 2026-06-28

### Initial repository

### Added

- Repository created
- Initial README
- Project identity established
