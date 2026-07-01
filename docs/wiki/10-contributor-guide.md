# AEGIS
# Contributor Guide

## What Is This?

This page gives contributors a practical path through the repository.

It does not replace `AGENTS.md`, `docs/CODING_STYLE.md`, or `docs/TASKS.md`.

The wiki is explanatory. `docs/OPERATING_DOCTRINE.md`, `docs/INVARIANTS.md`, `docs/ARCHITECTURE.md`, and task-specific technical documents remain authoritative.

## Before Changing Files

Read the current task prompt and then follow the repository reading order in `AGENTS.md`.

For most runtime work, start with:

1. `docs/OPERATING_DOCTRINE.md`
2. `docs/INVARIANTS.md`
3. `docs/CODING_STYLE.md`
4. `docs/ROADMAP.md`
5. `docs/PHASEMAP.md`
6. `docs/TASKS.md`
7. relevant architecture or runtime documents

If documentation and code disagree, stop and resolve the documentation gap first.

## Where Changes Usually Belong

| Work type | Likely location |
| --- | --- |
| request or response contracts | `src/gateway/`, `schemas/`, `docs/API_SPEC.md` |
| policy bundle verification | `src/policy/bundle.rs` |
| policy evaluation | `src/policy/evaluator.rs` |
| authorization or credential boundaries | `src/auth/` |
| wrapper dispatch | `src/gateway/wrapper.rs` |
| built-in wrappers | `src/wrappers/` |
| runtime coordination | `src/runtime/local.rs` |
| audit evidence and JSONL audit persistence | `src/audit/` |
| lifecycle and state logs | `src/state/` |
| recovery inspection or planning | `src/state/` |
| structured errors | `src/error.rs` |
| desktop shell and static Slint UI scaffold | `src-tauri/` |
| draft artifact workflows | `.github/workflows/` |
| local policy fixtures | `examples/policy-bundles/local-dev/` |
| request examples | `schemas/examples/` |
| tests | `tests/` |
| task tracking | `docs/TASKS.md` |
| release history | `CHANGELOG.md` |

## Validation

Run the validation required by the task.

For Rust runtime changes, the normal validation set is:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git diff --check
git status --short --branch
```

For documentation-only changes, Rust validation is unnecessary unless Rust files changed.

For desktop scaffold changes, also run:

```bash
cargo check --manifest-path src-tauri/Cargo.toml
```

## Current Release and Distribution Status

`v0.4.0` is closed.

Do not retroactively expand it.

It remains local-only, pre-alpha, source-oriented, and has no published release assets, installers, packaging, signing, notarization, or auto-update.

Post-`v0.4.0` distribution work targets `v0.4.1` as the first planned downloadable developer-preview release.

Initial planned platforms are macOS arm64 and macOS x64.

Draft artifacts should remain GitHub Actions workflow artifacts only until GitHub Release publishing is deliberately added.

Commit `3ab2874` added the draft artifact workflow and is now on `origin/main`.

The first manual workflow run passed and produced macOS workflow artifacts for inspection. Review found one follow-up before publishing work: produce one combined `SHA256SUMS` manifest for all draft archives.

## GitHub Workflow Scope

Changes under `.github/workflows` require a GitHub token with `workflow` scope. If a push is rejected because the token lacks workflow scope, do not rewrite release work to avoid the workflow file. Refresh authentication with workflow scope, then push the existing commit.

## Current Runtime Rules

Contributors should preserve these rules:

- malformed or unsupported requests fail closed
- policy bundle verification happens before policy evaluation
- policy evaluation happens before authorization
- denied and pending policy decisions do not authorize execution
- every wrapper execution passes through the credential boundary
- credential injection runs only when a wrapper requires and is allowed to receive a safe local handle reference
- wrappers do not self-authorize
- local credential handles are safe references, not secrets
- mutation wrappers require stronger gates than read-only wrappers
- audit and state logs remain separate
- recovery inspection and planning are read-only
- state logs are lifecycle evidence, not replay state
- future UI uses backend evidence and does not invent authority

## Adding or Changing Wrappers

Every wrapper must:

- declare a wrapper name
- declare a wrapper version
- declare an explicit credential requirement
- accept execution authorization context
- accept credential injection context when required
- return bounded output
- avoid secrets in output
- produce testable execution evidence

Mutation wrappers must also prove:

- idempotency context is present
- write scope is explicit and narrow
- path containment is enforced when filesystem paths are involved
- denied or pending policy decisions do not mutate state

## Updating Documentation

Documentation stability is active.

Do not update the README for ordinary implementation progress.

Use:

- `CHANGELOG.md` for release history
- `docs/TASKS.md` for task state
- narrow technical documents when behavior changes
- this wiki when explanatory guidance needs to become easier to follow

Do not use the wiki to create new requirements that are not present in governed documents.

## Before Committing

Check:

- the change belongs to the current phase
- no speculative code was added
- no secrets or secret-looking values were introduced
- fail-closed behavior is tested
- UI-renderable evidence remains structured
- recovery and replay wording does not imply execution exists before it is implemented
- validation passes
- changelog and task tracking are updated when required
