# AEGIS v0.4.0 Release Readiness Checklist

## Purpose

This checklist defines what must be true before tagging `v0.4.0`.

It turns the minimum usable local release path into a practical release gate.

It does not create release automation, packaging, signing, installer generation, or publishing.

## Release Identity

Release target:

```text
v0.4.0 Minimum Usable Local Release
```

Release identity:

- local-only
- pre-alpha
- developer-oriented
- not production-ready
- not enterprise-hardened

Do not tag `v0.4.0` if release notes, README text, UI text, or documentation imply production or enterprise readiness.

## Final Local Release Status

`v0.4.0` is approved for local-only release closure.

Maintainer approval was received in the final release task on 2026-06-30.

Release validation passed before tagging.

- [x] Release notes are complete.
- [x] `CHANGELOG.md` contains an explicit `v0.4.0` section.
- [x] Visual readability review passed.
- [x] Release validation gate passes.
- [x] Maintainer explicitly approves tagging.
- [x] `v0.4.0` tag is created from the final release-status commit.
- [x] `v0.4.0` tag is pushed to origin.
- [x] `main` is pushed to origin.
- [x] GitHub Release publishing is deferred.
- [x] No binaries, installers, packages, signatures, or release assets are published.

## Required Capability Checks

Before tagging, confirm these capabilities work or are accurately represented:

- [x] Desktop app launches locally.
- [x] Static/sample UI evidence renders in the desktop app.
- [x] Fixed live `health.check` backend evidence renders in the desktop app.
- [x] Local gateway binary runs.
- [x] `health.check` wrapper executes under policy allow.
- [x] `sandbox.note.write` wrapper executes under policy allow and sandbox containment.
- [x] Local policy bundle verification succeeds for the included local development bundle.
- [x] Local policy evaluation produces bounded decisions.
- [x] Execution authorization is required before wrapper execution.
- [x] Credential boundary and safe local credential handle evidence are present where required.
- [x] `audit.jsonl` output is created when requested.
- [x] `state.jsonl` output is created when requested.
- [x] Recovery inspection runs from a state log.
- [x] Recovery plan generation runs from a state log.
- [x] Structured errors are emitted for failure paths.
- [x] Build and launch documentation is accurate.

Do not add unimplemented capabilities to satisfy this checklist.

## Desktop GUI Checks

Confirm the graphical operator surface:

- [x] Tauri shell builds.
- [x] Slint UI builds.
- [x] Desktop app launches.
- [x] UI shows AEGIS identity.
- [x] UI shows pre-alpha, local-only, live-evidence, and sample-fallback status.
- [x] UI renders fixed live `health.check` evidence when available.
- [x] UI read-only IPC boundary tests pass.
- [x] UI IPC accepts no arbitrary request, wrapper, bundle, filesystem path, recovery, replay, approval, or credential input.
- [x] UI renders sample execution timeline.
- [x] UI renders sample status cards.
- [x] UI renders sample normalized error card.
- [x] UI renders sample recovery inspection card.
- [x] UI renders sample recovery plan card.
- [x] UI states that backend evidence drives the UI.
- [x] UI states that the authority boundary is in the backend.
- [x] UI does not imply arbitrary live backend control.
- [x] UI does not imply replay or recovery execution.
- [x] The v0.4.0 desktop UI is visually clear enough that a first-time user can understand the screen purpose, current state, and next available information in under 10 seconds.

Do not require installer generation, code signing, auto-update, or production packaging for `v0.4.0`.

## Local Gateway Checks

Confirm the local gateway:

- [x] Gateway binary runs.
- [x] Local policy bundle verifies.
- [x] `schemas/examples/valid/HealthCheckRequest.json` works.
- [x] `schemas/examples/valid/SandboxNoteWriteRequest.json` works.
- [x] `sandbox.note.write` writes only under the supplied sandbox directory.
- [x] Malformed requests fail closed.
- [x] Denied requests do not execute wrappers.
- [x] Pending requests do not execute wrappers.
- [x] Unsupported wrappers fail closed.

Keep this gate practical. Broad fuzzing, enterprise validation, remote trust, and production hardening are not part of `v0.4.0`.

## Evidence Checks

Confirm evidence behavior:

- [x] `audit.jsonl` is created when requested.
- [x] `state.jsonl` is created when requested.
- [x] Audit and state logs remain separate files.
- [x] Audit evidence does not contain secrets.
- [x] State evidence does not contain secrets.
- [x] Runtime stdout is structured JSON.
- [x] Structured error reports include `code`, `severity`, `message`, `reason`, `next_action`, and `location`.
- [x] Generated repo-local audit and state logs are removed before commit.

Secret-like material must not appear in runtime output, audit logs, state logs, UI fixtures, screenshots, or documentation.

## Recovery Inspection and Planning Checks

Confirm recovery evidence behavior:

- [x] `--inspect-state` reads `state.jsonl` read-only.
- [x] `--inspect-state` groups executions by `execution_id`.
- [x] `--inspect-state` reports last known state.
- [x] `--inspect-state` reports terminal status.
- [x] `--inspect-state` reports recoverability status.
- [x] `--plan-recovery` produces bounded recovery plan outcomes.
- [x] Completed executions are not recoverable.
- [x] Failed-closed executions are not recoverable.
- [x] `audit_failed` is an audit-retry candidate only.
- [x] `candidate_for_future_replay` means future evaluation only.
- [x] Recovery planning does not replay, resume, or mutate state.

Do not imply recovery execution, replay execution, or audit retry execution exists.

## Documentation Checks

Verify documentation clearly explains:

- [x] what AEGIS is
- [x] how to build the local gateway
- [x] how to launch the desktop app
- [x] how to run `health.check`
- [x] how to run `sandbox.note.write`
- [x] how to write audit and state logs
- [x] how to inspect state logs
- [x] how to generate a recovery plan
- [x] what the UI currently does
- [x] what the UI does not yet do
- [x] what is deferred beyond `v0.4.0`

Check these files at minimum:

- [x] `README.md`
- [x] `docs/RELEASE_PATH.md`
- [x] `docs/ROADMAP.md`
- [x] `docs/PHASEMAP.md`
- [x] `docs/TASKS.md`
- [x] `docs/wiki/README.md`
- [x] `docs/wiki/10-contributor-guide.md`
- [x] `docs/UI_EVIDENCE_CONTRACT.md`

Do not rewrite these files unless a release-readiness contradiction is found.

## Validation Commands

Run the executable release validation gate:

```bash
bash scripts/validate-v0.4.0-release.sh
```

This script runs repository validation, Rust validation, desktop validation, UI tests, gateway smoke tests, recovery inspection, recovery planning, desktop launch checking, and final repository cleanliness checks in order.

Do not claim release readiness unless this script passes on a clean worktree.

## Manual Verification

Run the gateway health check:

```bash
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  --state-log state.jsonl \
  schemas/examples/valid/HealthCheckRequest.json
```

Run sandbox note write:

```bash
mkdir -p /tmp/aegis-sandbox

cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  --state-log state.jsonl \
  --sandbox-dir /tmp/aegis-sandbox \
  schemas/examples/valid/SandboxNoteWriteRequest.json
```

Run recovery inspection:

```bash
cargo run --quiet --bin aegis-gateway -- --inspect-state state.jsonl
```

Run recovery planning:

```bash
cargo run --quiet --bin aegis-gateway -- --plan-recovery state.jsonl
```

Launch the desktop app:

```bash
cargo run --manifest-path src-tauri/Cargo.toml
```

Only mark desktop launch complete if the app actually opens.

Stop the desktop process cleanly after verification.

Remove generated local `audit.jsonl`, `state.jsonl`, and sandbox files before committing.

## Deferred Work Confirmation

Confirm these are not part of `v0.4.0`:

- [x] replay execution
- [x] recovery execution
- [x] audit retry execution
- [x] approval workflow
- [x] production credential providers
- [x] vault integration
- [x] cloud identity
- [x] remote logging
- [x] SIEM export
- [x] database persistence
- [x] HTTP/service deployment
- [x] installer generation
- [x] enterprise packaging
- [x] code signing
- [x] auto-update
- [x] distributed state
- [x] plugin architecture
- [x] external integrations

If any of these appear to be required for the release, stop and revisit the release scope instead of expanding `v0.4.0`.

## Release Blockers

Any of these block tagging:

- validation command failure
- desktop app cannot build
- desktop app cannot launch
- local gateway cannot run current supported wrappers
- audit or state evidence cannot be produced
- recovery inspection or recovery planning is broken
- README or release docs imply production readiness
- UI implies live backend authority
- UI IPC accepts arbitrary execution input
- sample UI implies replay or recovery execution exists
- secret-like material appears in runtime output, audit, state, UI fixture, or docs
- worktree is not clean

## Tagging Preconditions

Tagging may happen only after:

- [x] all validation commands pass
- [x] manual verification passes
- [x] release blockers are absent
- [x] `CHANGELOG.md` has the release entry
- [x] release limitations are documented
- [x] working tree is clean
- [x] maintainer explicitly approves tagging

## Final Readiness Decision

| Field | Value |
| --- | --- |
| Ready to tag | yes |
| Maintainer | local maintainer approval recorded from final release task |
| Date | 2026-06-30 |
| Validation gate passed | yes |
| Manual verification passed | yes, through the executable release validation gate |
| Release blockers absent | yes |
| Tag status | annotated `v0.4.0` created from the final release-status commit and pushed to origin |
| Main branch status | pushed to origin |
| Publishing status | GitHub Release publishing deferred |
| Asset status | no binaries, installers, packages, signatures, or release assets published |
| Notes | `v0.4.0` remains local-only, pre-alpha, developer-oriented, and source-oriented. Distribution planning is deferred. |
