# Changelog

All notable changes to AEGIS are documented in this file.

This changelog follows the Keep a Changelog structure and is governed by the AEGIS Definition of Done. Future releases shall update this file when repository behavior, documentation, schemas, policy contracts, or release artifacts change.

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
