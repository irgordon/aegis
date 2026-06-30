# AEGIS Release Path

## Purpose

This document defines the minimum usable local release path for AEGIS.

The goal is not production readiness. The goal is a small local-only release that a real user can build, launch, understand, and use safely.

The minimum usable release should be small enough to ship and safe enough to explain.

A feature that is not required for a local usable release should not block the local release.

## Minimum Usable Release

The minimum usable release is a pre-alpha, local-only, developer-oriented AEGIS build.

It must let a user:

- launch the desktop app
- run the local gateway with current supported wrappers
- produce audit and state evidence
- run recovery inspection and recovery planning
- see fixed live health-check evidence and sample fallback evidence clearly in the UI
- understand what is implemented and what is not

It must not claim production readiness, enterprise readiness, or live UI authority.

## Target Release Scope

The release includes:

- Tauri plus Slint desktop shell
- fixture-backed desktop evidence rendering
- local gateway binary
- local development policy bundle
- built-in `health.check` wrapper
- built-in `sandbox.note.write` wrapper
- local policy bundle verification
- local policy evaluation
- execution authorization
- credential class boundary
- safe local credential handle boundary
- optional local JSONL audit log
- optional local JSONL state log
- read-only recovery inspection
- read-only recovery planning
- fixed read-only live `health.check` UI evidence
- structured error output
- validation and launch instructions

## User-Visible Capabilities

A user can launch the desktop app and see fixed live health-check evidence with labeled sample fallback evidence.

A user can run the local gateway from the CLI with the included local policy bundle.

A user can execute the built-in `health.check` wrapper.

A user can execute the `sandbox.note.write` wrapper against a local sandbox directory.

A user can produce local `audit.jsonl` and `state.jsonl` evidence.

A user can inspect a state log.

A user can generate a read-only recovery plan from inspected state evidence.

The desktop UI does not run arbitrary live backend actions.

The desktop UI does not yet load live audit or state logs.

The desktop UI currently renders fixed live `health.check` evidence and labeled sample fallback evidence.

## Current Implemented Capabilities

The repository currently implements the local runtime, local policy bundle verification, local policy evaluation, governed built-in wrappers, structured errors, audit evidence, state evidence, recovery inspection, recovery planning, and a Tauri plus Slint UI scaffold.

The UI currently renders fixed live health-check evidence for current status and timeline fields, plus labeled sample normalized error, recovery inspection, and recovery planning evidence.

The current implementation satisfies the minimum usable local release path.

`v0.4.0` is complete after maintainer approval, release validation, annotated tag creation, main branch push, and tag push.

## Required Before Tagging

Complete only these tasks before tagging the minimum usable release:

- document build and launch commands
- verify local gateway commands in this release path
- verify the desktop scaffold build command
- verify the desktop launch command and fixed live health-check evidence boundary
- confirm all validation gates pass
- confirm generated local logs and sandbox files are not committed
- confirm README still points readers to `docs/`
- confirm release notes state local-only pre-alpha limitations
- obtain maintainer approval
- create the annotated `v0.4.0` tag
- push `main`
- push only the `v0.4.0` tag

Do not add replay execution, approval workflow, production credentials, HTTP service behavior, packaging, signing, or installer generation to this release gate.

## Explicitly Deferred

The minimum usable release explicitly defers:

- replay execution
- recovery execution
- audit retry execution
- approval workflow
- production credential providers
- vault integration
- cloud identity
- remote logging
- SIEM export
- database persistence
- HTTP or service deployment
- enterprise packaging
- installer generation
- code signing
- auto-update
- distributed state
- plugin architecture
- external integrations
- production hardening

## Build and Launch Path

Validate repository governance:

```bash
python3 scripts/verify.py
```

Validate the local gateway crate:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Validate the desktop crate:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

Run `health.check` locally:

```bash
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  --state-log state.jsonl \
  schemas/examples/valid/HealthCheckRequest.json
```

Run `sandbox.note.write` locally:

```bash
mkdir -p /tmp/aegis-sandbox

cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  --state-log state.jsonl \
  --sandbox-dir /tmp/aegis-sandbox \
  schemas/examples/valid/SandboxNoteWriteRequest.json
```

Inspect and plan recovery from a state log:

```bash
cargo run --quiet --bin aegis-gateway -- --inspect-state state.jsonl
cargo run --quiet --bin aegis-gateway -- --plan-recovery state.jsonl
```

Launch the desktop app:

```bash
cargo run --manifest-path src-tauri/Cargo.toml
```

Generated `audit.jsonl`, `state.jsonl`, and sandbox files are local development artifacts. Do not commit them.

## Validation Gate

The minimum usable release validation gate is executable:

```bash
bash scripts/validate-v0.4.0-release.sh
```

The script runs the existing repository, Rust, desktop, UI, gateway smoke, recovery inspection, recovery planning, desktop launch, and cleanliness checks in the documented order.

Do not tag the release unless the validation gate passes on a clean worktree.

## Release Candidate Checklist

Use `docs/RELEASE_CHECKLIST_v0.4.0.md` as the concrete readiness and final local release checklist.

- [x] `bash scripts/validate-v0.4.0-release.sh` passes.
- [x] UI distinguishes fixed live health-check evidence from sample fallback evidence.
- [x] README remains short and points to `docs/`.
- [x] Changelog records the release.
- [x] No generated logs, sandbox files, or build artifacts are staged.
- [x] Maintainer approval is recorded.
- [x] Annotated `v0.4.0` tag is created and pushed.
- [x] GitHub Release publishing and distribution artifacts remain deferred.

## Out-of-Scope for Minimum Release

The minimum release does not include arbitrary gateway execution from the UI, mutation execution from the UI, user-selected policy bundles, live audit or state log loading, live recovery execution, HTTP service behavior, production packaging, installer generation, signing, auto-update, production credentials, approval workflow, replay execution, recovery execution, audit retry execution, external integrations, or enterprise hardening.

## Recommended Version Target

Recommended target:

```text
v0.4.0 Minimum Usable Local Release
```

Reason:

`v0.3.0` is already assigned to the completed local Governed Execution Engine foundation. The minimum usable release depends on the Phase 4 desktop shell and fixture-backed UI evidence, so `v0.4.0` is the clearest version target.

The final local release task creates the annotated `v0.4.0` tag after maintainer approval and release validation.

## Risks

- The desktop UI combines fixed live health-check evidence with sample fallback evidence, so fallback labels must remain clear.
- Local logs are development artifacts and are not production audit storage.
- Recovery planning is read-only guidance and may be mistaken for recovery execution.
- The local credential handle is safe development evidence, not a real credential provider.
- Without packaging, users must launch from source.

## Final Release Closure

`v0.4.0` closes when:

1. Maintainer approval is recorded.
2. The full validation gate passes on a clean checkout.
3. The final release-status documents are committed.
4. The annotated `v0.4.0` tag is created.
5. `main` is pushed to origin.
6. The `v0.4.0` tag is pushed to origin.

GitHub Release publishing, downloadable binaries, installers, packaging, signing, notarization, auto-update, and distribution workflow remain deferred.

## Post-v0.4.0 Distribution Planning

After `v0.4.0`, distribution work should follow `docs/RELEASE_DISTRIBUTION_PLAN.md`.

The plan keeps `v0.4.0` source-only and defines a staged path toward GitHub Release artifacts, checksums, platform builds, signing, notarization, and installers.

The first downloadable artifact targets are selected in `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md`.

Downloadable artifacts remain future work.
