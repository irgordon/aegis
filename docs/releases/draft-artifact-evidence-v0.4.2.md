# AEGIS v0.4.2 Draft Artifact Evidence Review

## Summary

Review result: **BLOCKED**.

The `v0.4.2` draft artifacts pass download, checksum, extraction, archive
content, gateway help, gateway smoke-test, desktop process launch, binary
metadata, path-remapping, and release-boundary checks.

The remaining release gate is visual desktop evidence. The review host was
accessed remotely while its display session was asleep or logged out. Both
desktop binaries started and stayed running without terminal errors, but macOS
screen capture returned a black display and exposed no inspectable application
window. The review therefore cannot claim that the desktop visibly renders the
required release identity and neutral no-error state.

This is an environment blocker, not an observed artifact failure. A PASS is
still required before creating the immutable `v0.4.2` tag or any draft GitHub
Release.

## Review Scope

This review evaluates the non-publishing workflow artifacts as a new evaluator
would. It does not create or move a tag, create or update a GitHub Release,
publish assets, sign artifacts, notarize binaries, or create installers.

## Workflow Evidence

| Field | Value |
| --- | --- |
| Workflow | `Draft Release Artifacts` |
| Run ID | `29374158498` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/29374158498` |
| Event | `workflow_dispatch` |
| Branch | `main` |
| Commit | `f0b7585253a845f988d9730bfebba38488d21846` |
| Result | `success` |
| Completed | `2026-07-14T23:01:13Z` |

Job results:

| Job | Result |
| --- | --- |
| `Draft macOS artifact (macos-arm64)` | success |
| `Draft macOS artifact (macos-x64)` | success |
| `Combined draft artifact checksums` | success |

## Review Host

| Field | Value |
| --- | --- |
| Host architecture | macOS arm64 |
| Operating system | macOS 26.5.2, build 25F84 |
| x64 compatibility | Rosetta available and exercised |
| Repository state | clean `main` at the workflow commit |
| Active GitHub identity | `irgordon` |

Artifacts were downloaded to an isolated temporary directory outside the
repository.

## Download and Checksum Evidence

The run exposed the expected workflow artifacts:

- `draft-macos-arm64-v0.4.2`
- `draft-macos-x64-v0.4.2`
- `draft-artifacts-v0.4.2`

The combined artifact contained both archives and one `SHA256SUMS` manifest.

```text
6d351953ff3f33a9c54e3aca089a7f1619efca479fca25e03c663c2c009d0da6  aegis-v0.4.2-macos-arm64.tar.gz
cf808bff8cb6ab657036cb57a663ff3016c39f80269c3a412d688bb60ef715f1  aegis-v0.4.2-macos-x64.tar.gz
```

Combined verification passed:

```text
aegis-v0.4.2-macos-arm64.tar.gz: OK
aegis-v0.4.2-macos-x64.tar.gz: OK
```

The per-platform downloads were byte-identical to the archive copies included
in the combined workflow artifact.

## Archive Content Evidence

Both archives extracted cleanly and contained:

```text
README.md
ARTIFACT-CONTENTS.md
bin/aegis-gateway
desktop/aegis-desktop
examples/health-check-request.json
policy-bundles/local-dev/manifest.yaml
policy-bundles/local-dev/gateway_policy.yaml
policy-bundles/local-dev/risk_matrix.yaml
policy-bundles/local-dev/checksums/SHA256SUMS
policy-bundles/local-dev/signatures/public.pem
policy-bundles/local-dev/signatures/SHA256SUMS.sig
```

The policy bundle checksum manifest verified in both extracted archives. The
two archives contained identical health-check fixtures and policy checksum
manifests.

No extracted archive contained `.git`, `.github`, `target`, `node_modules`,
`.env`, private-key-like files, or absolute-path mirror directories.

The bundled request is a parameter-free L0 `health.check` request. It does not
request mutation, approval, replay, credentials, or recovery execution.

## Binary Evidence

| Binary | Metadata | Runtime result |
| --- | --- | --- |
| arm64 gateway | Mach-O arm64, executable | help and smoke test passed |
| x64 gateway | Mach-O x86_64, executable | help and smoke test passed through Rosetta |
| arm64 desktop | Mach-O arm64, executable | process started and remained running |
| x64 desktop | Mach-O x86_64, executable | process started and remained running through Rosetta |

The gateway binaries depend only on `/usr/lib/libSystem.B.dylib`. The desktop
binaries depend only on macOS system libraries and frameworks.

Both gateway help commands returned success and showed:

- conventional `-h` and `--help` behavior
- the archive-only smoke-test command
- unsigned Developer Preview warnings
- the archive README as the documentation entry point

Both archive-only smoke tests returned success and satisfied all checked
conditions:

- response status `allowed`
- wrapper result `healthy`
- policy bundle verification `verified`
- Ed25519 signature status `signature_verified`
- wrapper status `executed`
- lifecycle state `completed`
- audit status `allowed`
- no credential required

## Portability and Path-Remapping Evidence

The previously accepted portability conditions remain true:

- neither gateway binary contains a scanned high-risk path marker
- neither desktop binary contains `CARGO_MANIFEST_DIR`
- neither desktop binary contains `../examples/policy-bundles/local-dev`
- neither desktop binary contains `examples/policy-bundles/local-dev`
- both desktop binaries contain the intended artifact-relative
  `policy-bundles/local-dev` path
- each desktop binary contains one known Tauri-generated
  `/Users/runner/work/aegis/aegis/src-tauri` context string

The remaining Tauri context string is the previously accepted residual release
hygiene. It is not a runtime policy bundle dependency and is not described as
perfect binary cleanliness.

## Desktop Visual Evidence

Supporting binary inspection confirmed that both desktop artifacts contain the
required labels:

- `Current development: v0.4.2`
- `Development branch`
- `Latest release v0.4.1`
- `No live error reported`
- `The latest health-check evidence did not include an error report.`
- `Unsigned`
- `Not notarized`
- `Not production-ready`

The static UI integrity tests also cover these labels and the neutral no-error
state. These checks prove that the intended strings are compiled into both
artifacts, but they do not prove that the window visibly renders them.

Visual confirmation is blocked because the remote display session is asleep or
logged out. Brief wake activity did not restore an inspectable WindowServer
surface. Screen captures remained black even though the desktop processes were
running.

Result: desktop process launch PASS; visible first-screen evidence BLOCKED by
the review environment.

## Annotated-Tag Guard Evidence

Static release-workflow validation passed all 15 tests. The checks prove that
the draft release workflow:

- is manual-only
- requires an existing `v0.4.2` tag
- fetches only the exact tag when needed
- compares the peeled tag commit to `HEAD`
- avoids broad or forced tag fetching
- does not create, move, or push tags
- refuses to modify a non-draft release
- creates only a draft prerelease when all later gates pass

The current negative precondition also behaves fail-closed:

- no local `v0.4.2` tag exists
- no remote `v0.4.2` tag exists
- the remote tag lookup exits non-zero
- no `v0.4.2` GitHub Release exists

The tag-to-`HEAD` equality check cannot be exercised until a maintainer creates
the tag after this evidence review passes. That live check remains part of the
separate release task.

The immutable `v0.4.1` tag remains at annotated tag object
`332e0f9c983d47b5bfabf8caa1cc2a541c6f0bf2`, peeling to commit
`2bdb0704b1541e344528408e76a028287bdc9336`. The only GitHub Release remains
the published `v0.4.1` prerelease.

## Gate Results

| Criterion | Result |
| --- | --- |
| Download both macOS archives and combined checksums | PASS |
| Verify both archive checksums | PASS |
| Extract both archives | PASS |
| Confirm bundled smoke-test fixture and policy bundle | PASS |
| Run archive-only gateway smoke test | PASS on arm64 and x64 |
| Confirm gateway help behavior | PASS on arm64 and x64 |
| Launch desktop binary | PASS at process boundary on arm64 and x64 |
| Confirm visible desktop release identity | BLOCKED by inactive GUI session |
| Confirm visible neutral no-error state | BLOCKED by inactive GUI session |
| Verify accepted portability and path-remapping conditions | PASS |
| Verify no tag or release was created | PASS |
| Validate annotated-tag guard without creating a tag | PASS for static and negative preconditions |

## Review Result

**BLOCKED**.

Artifact integrity and nonvisual runtime evidence pass. The release gate remains
closed because visual desktop evidence is explicitly required and unavailable
from the current remote GUI session.

Do not create the `v0.4.2` tag or draft GitHub Release from this result.

## Required Follow-Up

1. Log into an active local macOS graphical session with the display on.
2. Launch the extracted arm64 desktop artifact from its archive root.
3. Capture and inspect the first screen for current-development identity,
   latest-release identity, and the neutral no-error state.
4. Update this review to PASS only if the visible evidence matches the compiled
   labels and no new blocker appears.
5. Begin tag and draft-release work only in a separate approved task after PASS.

GitHub Actions Node runtime warnings and the `macos-latest` runner migration are
tracked separately as bounded release-workflow maintenance. They do not affect
this artifact evidence result.
