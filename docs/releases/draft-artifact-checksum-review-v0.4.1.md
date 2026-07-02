# AEGIS v0.4.1 Draft Artifact Checksum Review

## Summary

Recommendation: PASS.

The `Draft Release Artifacts` workflow was re-run after commit `0fcdde2db1f1a93da3590f9cd7c39810687417aa`.

The run produced one combined draft artifact bundle named `draft-artifacts-v0.4.1`.

That bundle contains both macOS draft archives and one combined `SHA256SUMS` manifest. The manifest covers both archives and verifies successfully.

These artifacts are GitHub Actions workflow artifacts for inspection only. They are not GitHub Release assets.

## Workflow Run

| Field | Value |
| --- | --- |
| Workflow | `Draft Release Artifacts` |
| Run ID | `28552699446` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/28552699446` |
| Branch | `main` |
| Commit | `0fcdde2db1f1a93da3590f9cd7c39810687417aa` |
| Result | `success` |
| Started | `2026-07-01T22:45:27Z` |
| Completed | `2026-07-01T22:57:27Z` |
| Duration | about 12 minutes |

Job results:

| Job | Result |
| --- | --- |
| `Draft macOS artifact (macos-arm64)` | success |
| `Draft macOS artifact (macos-x64)` | success |
| `Combined draft artifact checksums` | success |

## Combined Artifact Bundle

The combined workflow artifact `draft-artifacts-v0.4.1` contains exactly the expected release-facing files:

```text
SHA256SUMS
aegis-v0.4.1-macos-arm64.tar.gz
aegis-v0.4.1-macos-x64.tar.gz
```

No `SHA256SUMS.sig` file was produced. Checksum signing remains deferred.

## SHA256SUMS Verification

The combined manifest includes exactly two archive entries:

```text
fb3997ada53cd6d0adfae2c74df04641dc65f6058b574979556a8516c47aad23  aegis-v0.4.1-macos-arm64.tar.gz
9073f117f9594acd48f2793643d27e9fe4553f3d36c1b98852759194d13ad788  aegis-v0.4.1-macos-x64.tar.gz
```

Local verification passed:

```text
aegis-v0.4.1-macos-arm64.tar.gz: OK
aegis-v0.4.1-macos-x64.tar.gz: OK
```

The manifest covers produced archive files only. It does not include extracted files, binaries, directories, or missing artifacts.

## Portability Regression Check

Both archives still contain the bundled local development policy bundle:

```text
policy-bundles/local-dev/manifest.yaml
policy-bundles/local-dev/gateway_policy.yaml
policy-bundles/local-dev/risk_matrix.yaml
policy-bundles/local-dev/checksums/SHA256SUMS
policy-bundles/local-dev/signatures/public.pem
policy-bundles/local-dev/signatures/SHA256SUMS.sig
```

No extracted archive contained:

- `.git`
- `.github`
- `target`
- `node_modules`
- Cargo registry caches
- `.env`
- private-key-like files
- local temp folders

No maintainer-local path, private key, GitHub token, API key, or committed secret was found in extracted files.

A later path-remapping review confirmed the release desktop binaries no longer contain `CARGO_MANIFEST_DIR` or the source-tree development fallback string. DA-ECA-002 is resolved for runtime portability with residual Tauri-generated desktop context metadata deferred.

The produced binaries have the expected architectures:

| Binary | Architecture |
| --- | --- |
| `macos-arm64/bin/aegis-gateway` | Mach-O arm64 |
| `macos-arm64/desktop/aegis-desktop` | Mach-O arm64 |
| `macos-x64/bin/aegis-gateway` | Mach-O x86_64 |
| `macos-x64/desktop/aegis-desktop` | Mach-O x86_64 |

## Release Boundary Verification

Verified:

- no `v0.4.1` tag exists on origin
- no `v0.4.1` GitHub Release exists
- no public release assets exist
- no signing was performed
- no notarization was performed
- no installers were produced
- no auto-update metadata was produced

## Issue Status

| Issue | Status | Notes |
| --- | --- | --- |
| Combined `SHA256SUMS` | Resolved | Artifact evidence confirms one combined manifest covers both macOS draft archives and verifies successfully. |
| DA-ECA-001 | Resolved | The bundled policy bundle remains present, and artifact-relative policy resolution remains intact. |
| DA-ECA-002 | Resolved for runtime portability; residual metadata deferred | Workflow run `28607528253` confirms no runtime-relevant source path leakage remains. One Tauri-generated desktop context string remains as release hygiene. |
| DA-ECA-003 | Resolved | Artifact README and content manifest remain present in both archives. |

## Recommendation

PASS.

The combined checksum manifest is artifact-verified. The draft artifact workflow now produces a release-facing checksum shape suitable for the next planning step.

This does not make `v0.4.1` published or release-ready. GitHub Release publishing, public release assets, tags, signing, notarization, installers, and auto-update remain out of scope.

## Required Follow-Up

1. Keep residual Tauri-generated source metadata tracked as release hygiene.
2. Prepare publishing work only in a separate approved task.
