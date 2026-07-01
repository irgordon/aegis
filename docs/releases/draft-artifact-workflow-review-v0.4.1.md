# AEGIS v0.4.1 Draft Artifact Workflow Review

## Summary

Recommendation: PASS WITH FIXES.

The manual `Draft Release Artifacts` workflow ran successfully from `main`.

It produced macOS arm64 and macOS x64 draft archives as GitHub Actions workflow artifacts only.

No GitHub Release, release asset, `v0.4.1` tag, signing, notarization, installer, or auto-update behavior was created.

One issue should be fixed before publishing work begins: checksum manifests are generated per platform artifact. A future release-facing workflow should produce one `SHA256SUMS` manifest that lists every produced archive.

Follow-up portability status: the rerun documented in `docs/releases/draft-artifact-portability-rerun-v0.4.1.md` confirms the desktop policy bundle source-path blocker is resolved at the artifact level. The combined checksum follow-up remains open.

## Workflow Run

| Field | Value |
| --- | --- |
| Workflow | Draft Release Artifacts |
| Run ID | `28484349169` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/28484349169` |
| Event | `workflow_dispatch` |
| Branch | `main` |
| Commit | `9e4afa392b936d0c9bc603b90649235df5c9c196` |
| Result | success |
| Started | 2026-07-01 00:13:22 UTC |
| Completed | 2026-07-01 00:23:43 UTC |
| Duration | about 10 minutes 21 seconds |

Both jobs passed:

- `Draft macOS artifact (macos-arm64)`
- `Draft macOS artifact (macos-x64)`

## Artifact Results

GitHub Actions workflow artifacts:

| Workflow artifact | Expected archive | Result |
| --- | --- | --- |
| `draft-macos-arm64-v0.4.1` | `aegis-v0.4.1-macos-arm64.tar.gz` | present |
| `draft-macos-x64-v0.4.1` | `aegis-v0.4.1-macos-x64.tar.gz` | present |

Each workflow artifact also included a `SHA256SUMS` file.

These are workflow artifacts for inspection only. They are not public release assets.

## Archive Contents

Both archives contain the expected draft structure:

```text
aegis-v0.4.1-macos-<arch>/
  README.md
  ARTIFACT-CONTENTS.md
  bin/aegis-gateway
  desktop/aegis-desktop
```

The binaries were inspected locally:

- macOS arm64 archive contains Mach-O arm64 `aegis-gateway` and `aegis-desktop`.
- macOS x64 archive contains Mach-O x86_64 `aegis-gateway` and `aegis-desktop`.

No `.git` directory, build cache, `target` directory, private key file, local temp file, or unrelated repository tree was found in the extracted archives.

No secret-like strings were found in the extracted text files.

## Checksum Verification

Local checksum verification passed:

```text
aegis-v0.4.1-macos-arm64.tar.gz: OK
aegis-v0.4.1-macos-x64.tar.gz: OK
```

Observed checksum files:

```text
bb51755b870d8b749c71e485ea97495662ceb2001b7980a735f58f56619d0f97  aegis-v0.4.1-macos-arm64.tar.gz
c701b48f3667b22d15a861db8c8fd857eb40c9397c668f70a7cfe5a90486c682  aegis-v0.4.1-macos-x64.tar.gz
```

Issue: each matrix artifact has its own `SHA256SUMS`. Before release publishing work, the workflow should produce one combined `SHA256SUMS` manifest for all produced archives.

## Release Boundary Verification

Verified:

- no `v0.4.1` tag exists on origin
- no `v0.4.1` GitHub Release exists
- no GitHub Releases are listed for the repository
- artifacts were uploaded only as GitHub Actions workflow artifacts
- no signing or notarization was performed
- no installer artifacts were produced
- no auto-update metadata was produced

## Issues Found

| Issue | Severity | Notes |
| --- | --- | --- |
| Per-platform checksum manifests | Low | Useful for inspection, but a future release-facing workflow should generate one combined `SHA256SUMS` manifest covering all produced archives. |

No release-boundary violation was found.

## Recommendation

PASS WITH FIXES.

The workflow proves that AEGIS can build draft macOS developer-preview artifact candidates without publishing a GitHub Release.

Before publishing work begins, add a small workflow follow-up to produce and verify one combined `SHA256SUMS` manifest across all produced archives.

## Next Steps

1. Add a focused follow-up: `ci(release): Consolidate draft artifact checksums`.
2. Re-run the draft workflow review after the checksum manifest shape is fixed.
3. Continue to keep GitHub Release publishing, tags, signing, notarization, installers, and auto-update out of scope until a separate publishing task is approved.
