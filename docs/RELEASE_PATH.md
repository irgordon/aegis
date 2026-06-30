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
- see sample evidence clearly in the UI
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
- structured error output
- validation and launch instructions

## User-Visible Capabilities

A user can launch the desktop app and see sample execution evidence.

A user can run the local gateway from the CLI with the included local policy bundle.

A user can execute the built-in `health.check` wrapper.

A user can execute the `sandbox.note.write` wrapper against a local sandbox directory.

A user can produce local `audit.jsonl` and `state.jsonl` evidence.

A user can inspect a state log.

A user can generate a read-only recovery plan from inspected state evidence.

The desktop UI does not yet run live backend actions.

The desktop UI does not yet load live audit or state logs.

The desktop UI currently renders sample evidence only.

## Current Implemented Capabilities

The repository currently implements the local runtime, local policy bundle verification, local policy evaluation, governed built-in wrappers, structured errors, audit evidence, state evidence, recovery inspection, recovery planning, and a static Tauri plus Slint UI scaffold.

The UI currently renders sample timeline, status, normalized error, recovery inspection, and recovery planning evidence.

The current implementation is enough to define a minimum usable local release path, but not enough to tag it without completing the checklist below.

## Required Before Tagging

Complete only these tasks before tagging the minimum usable release:

- document build and launch commands
- verify local gateway commands in this release path
- verify the desktop scaffold build command
- verify the sample UI launch command
- confirm all validation gates pass
- confirm generated local logs and sandbox files are not committed
- confirm README still points readers to `docs/`
- confirm release notes state local-only pre-alpha limitations

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

The minimum usable release validation gate is:

```bash
python3 scripts/verify.py

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test

cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml

git diff --check
git status --short --branch
```

Do not tag the release unless the validation gate passes.

## Release Candidate Checklist

Use `docs/RELEASE_CHECKLIST_v0.4.0.md` as the concrete readiness checklist before tagging.

- [ ] Repository verification passes.
- [ ] Local gateway formatting, clippy, and tests pass.
- [ ] Desktop formatting, clippy, tests, and check pass.
- [ ] `health.check` command succeeds with local policy bundle.
- [ ] `sandbox.note.write` command succeeds against a local sandbox directory.
- [ ] Audit JSONL evidence is created locally.
- [ ] State JSONL evidence is created locally.
- [ ] Recovery inspection command returns structured JSON.
- [ ] Recovery planning command returns structured JSON.
- [ ] Desktop app launches locally.
- [ ] UI distinguishes fixed live health-check evidence from sample fallback evidence.
- [ ] README remains short and points to `docs/`.
- [ ] Changelog records the release.
- [ ] No generated logs, sandbox files, or build artifacts are staged.

## Out-of-Scope for Minimum Release

The minimum release does not include arbitrary gateway execution from the UI, mutation execution from the UI, user-selected policy bundles, live audit or state log loading, live recovery execution, HTTP service behavior, production packaging, installer generation, signing, auto-update, production credentials, approval workflow, replay execution, recovery execution, audit retry execution, external integrations, or enterprise hardening.

## Recommended Version Target

Recommended target:

```text
v0.4.0 Minimum Usable Local Release
```

Reason:

`v0.3.0` is already assigned to the completed local Governed Execution Engine foundation. The minimum usable release depends on the Phase 4 desktop shell and fixture-backed UI evidence, so `v0.4.0` is the clearest version target.

Do not create the tag as part of release planning.

## Risks

- The desktop UI combines fixed live health-check evidence with sample fallback evidence, so fallback labels must remain clear.
- Local logs are development artifacts and are not production audit storage.
- Recovery planning is read-only guidance and may be mistaken for recovery execution.
- The local credential handle is safe development evidence, not a real credential provider.
- Without packaging, users must launch from source.

## Next Tasks

Recommended next tasks:

1. Complete `docs/RELEASE_CHECKLIST_v0.4.0.md`.
2. Verify the release commands end to end and record any command corrections.
3. Add a short local release note for `v0.4.0`.
4. Run the full validation gate on a clean checkout.
5. Launch the desktop app locally and confirm fixed live evidence and labeled sample fallback behavior.
6. Tag `v0.4.0` only after the release candidate checklist passes and maintainer approval is explicit.
