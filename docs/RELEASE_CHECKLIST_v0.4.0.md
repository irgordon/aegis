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

## Required Capability Checks

Before tagging, confirm these capabilities work or are accurately represented:

- [ ] Desktop app launches locally.
- [ ] Static/sample UI evidence renders in the desktop app.
- [ ] Fixed live `health.check` backend evidence renders in the desktop app.
- [ ] Local gateway binary runs.
- [ ] `health.check` wrapper executes under policy allow.
- [ ] `sandbox.note.write` wrapper executes under policy allow and sandbox containment.
- [ ] Local policy bundle verification succeeds for the included local development bundle.
- [ ] Local policy evaluation produces bounded decisions.
- [ ] Execution authorization is required before wrapper execution.
- [ ] Credential boundary and safe local credential handle evidence are present where required.
- [ ] `audit.jsonl` output is created when requested.
- [ ] `state.jsonl` output is created when requested.
- [ ] Recovery inspection runs from a state log.
- [ ] Recovery plan generation runs from a state log.
- [ ] Structured errors are emitted for failure paths.
- [ ] Build and launch documentation is accurate.

Do not add unimplemented capabilities to satisfy this checklist.

## Desktop GUI Checks

Confirm the graphical operator surface:

- [ ] Tauri shell builds.
- [ ] Slint UI builds.
- [ ] Desktop app launches.
- [ ] UI shows AEGIS identity.
- [ ] UI shows pre-alpha, local-only, live-evidence, and sample-fallback status.
- [ ] UI renders fixed live `health.check` evidence when available.
- [ ] UI renders sample execution timeline.
- [ ] UI renders sample status cards.
- [ ] UI renders sample normalized error card.
- [ ] UI renders sample recovery inspection card.
- [ ] UI renders sample recovery plan card.
- [ ] UI states that backend evidence drives the UI.
- [ ] UI states that the authority boundary is in the backend.
- [ ] UI does not imply arbitrary live backend control.
- [ ] UI does not imply replay or recovery execution.

Do not require installer generation, code signing, auto-update, or production packaging for `v0.4.0`.

## Local Gateway Checks

Confirm the local gateway:

- [ ] Gateway binary runs.
- [ ] Local policy bundle verifies.
- [ ] `schemas/examples/valid/HealthCheckRequest.json` works.
- [ ] `schemas/examples/valid/SandboxNoteWriteRequest.json` works.
- [ ] `sandbox.note.write` writes only under the supplied sandbox directory.
- [ ] Malformed requests fail closed.
- [ ] Denied requests do not execute wrappers.
- [ ] Pending requests do not execute wrappers.
- [ ] Unsupported wrappers fail closed.

Keep this gate practical. Broad fuzzing, enterprise validation, remote trust, and production hardening are not part of `v0.4.0`.

## Evidence Checks

Confirm evidence behavior:

- [ ] `audit.jsonl` is created when requested.
- [ ] `state.jsonl` is created when requested.
- [ ] Audit and state logs remain separate files.
- [ ] Audit evidence does not contain secrets.
- [ ] State evidence does not contain secrets.
- [ ] Runtime stdout is structured JSON.
- [ ] Structured error reports include `code`, `severity`, `message`, `reason`, `next_action`, and `location`.
- [ ] Generated repo-local audit and state logs are removed before commit.

Secret-like material must not appear in runtime output, audit logs, state logs, UI fixtures, screenshots, or documentation.

## Recovery Inspection and Planning Checks

Confirm recovery evidence behavior:

- [ ] `--inspect-state` reads `state.jsonl` read-only.
- [ ] `--inspect-state` groups executions by `execution_id`.
- [ ] `--inspect-state` reports last known state.
- [ ] `--inspect-state` reports terminal status.
- [ ] `--inspect-state` reports recoverability status.
- [ ] `--plan-recovery` produces bounded recovery plan outcomes.
- [ ] Completed executions are not recoverable.
- [ ] Failed-closed executions are not recoverable.
- [ ] `audit_failed` is an audit-retry candidate only.
- [ ] `candidate_for_future_replay` means future evaluation only.
- [ ] Recovery planning does not replay, resume, or mutate state.

Do not imply recovery execution, replay execution, or audit retry execution exists.

## Documentation Checks

Verify documentation clearly explains:

- [ ] what AEGIS is
- [ ] how to build the local gateway
- [ ] how to launch the desktop app
- [ ] how to run `health.check`
- [ ] how to run `sandbox.note.write`
- [ ] how to write audit and state logs
- [ ] how to inspect state logs
- [ ] how to generate a recovery plan
- [ ] what the UI currently does
- [ ] what the UI does not yet do
- [ ] what is deferred beyond `v0.4.0`

Check these files at minimum:

- [ ] `README.md`
- [ ] `docs/RELEASE_PATH.md`
- [ ] `docs/ROADMAP.md`
- [ ] `docs/PHASEMAP.md`
- [ ] `docs/TASKS.md`
- [ ] `docs/wiki/README.md`
- [ ] `docs/wiki/10-contributor-guide.md`
- [ ] `docs/UI_EVIDENCE_CONTRACT.md`

Do not rewrite these files unless a release-readiness contradiction is found.

## Validation Commands

Run the full release validation gate:

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

Do not claim release readiness unless these commands pass.

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

- [ ] replay execution
- [ ] recovery execution
- [ ] audit retry execution
- [ ] approval workflow
- [ ] production credential providers
- [ ] vault integration
- [ ] cloud identity
- [ ] remote logging
- [ ] SIEM export
- [ ] database persistence
- [ ] HTTP/service deployment
- [ ] installer generation
- [ ] enterprise packaging
- [ ] code signing
- [ ] auto-update
- [ ] distributed state
- [ ] plugin architecture
- [ ] external integrations

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
- sample UI implies replay or recovery execution exists
- secret-like material appears in runtime output, audit, state, UI fixture, or docs
- worktree is not clean

## Tagging Preconditions

Do not create a tag from this checklist task.

Tagging may happen only after:

- [ ] all validation commands pass
- [ ] manual verification passes
- [ ] release blockers are absent
- [ ] `CHANGELOG.md` has the release entry
- [ ] release limitations are documented
- [ ] working tree is clean
- [ ] maintainer explicitly approves tagging

## Final Readiness Decision

Complete this section only after validation and manual verification.

| Field | Value |
| --- | --- |
| Ready to tag | yes/no |
| Maintainer |  |
| Date |  |
| Validation gate passed | yes/no |
| Manual verification passed | yes/no |
| Release blockers absent | yes/no |
| Notes |  |
