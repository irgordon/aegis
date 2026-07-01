# AEGIS v0.4.1 Draft Artifact Portability Re-run

## Summary

Recommendation: PASS WITH FIXES.

The `Draft Release Artifacts` workflow was re-run after commit `f21782126a021a9d7b3071bd0f92f9a8fc7edbd7`.

The new macOS arm64 and macOS x64 draft artifacts include the local development policy bundle under `policy-bundles/local-dev`. The desktop resolver now prefers that artifact-relative policy bundle before using the source-tree development fallback.

DA-ECA-001 is resolved at the artifact level. The desktop health-check evidence path no longer requires the GitHub runner source checkout as the primary policy bundle location.

Two follow-ups remain before publishing work:

- Strip or remap source/debug paths in developer-preview binaries where practical.
- Produce one combined `SHA256SUMS` manifest across all produced draft archives.

These workflow artifacts are inspection evidence only. They are not GitHub Release assets.

## Workflow Run

| Field | Value |
| --- | --- |
| Workflow | `Draft Release Artifacts` |
| Run ID | `28548645224` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/28548645224` |
| Branch | `main` |
| Commit | `f21782126a021a9d7b3071bd0f92f9a8fc7edbd7` |
| Result | `success` |
| Started | `2026-07-01T21:22:52Z` |
| Completed | `2026-07-01T21:34:29Z` |
| Duration | about 11 minutes 37 seconds |

Both jobs passed:

- `Draft macOS artifact (macos-arm64)`
- `Draft macOS artifact (macos-x64)`

## Artifact Results

Downloaded workflow artifacts:

| Workflow artifact | Archive | Result |
| --- | --- | --- |
| `draft-macos-arm64-v0.4.1` | `aegis-v0.4.1-macos-arm64.tar.gz` | present |
| `draft-macos-x64-v0.4.1` | `aegis-v0.4.1-macos-x64.tar.gz` | present |

Each workflow artifact included a platform-local `SHA256SUMS` file.

## Archive Tree Review

Both archives contain the expected draft structure:

```text
README.md
ARTIFACT-CONTENTS.md
bin/aegis-gateway
desktop/aegis-desktop
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
- absolute-path mirror directories

## Policy Bundle Review

The required runtime policy bundle files are present in both platform archives:

- `manifest.yaml`
- `gateway_policy.yaml`
- `risk_matrix.yaml`
- `checksums/SHA256SUMS`
- `signatures/public.pem`
- `signatures/SHA256SUMS.sig`

Private signing material is not present.

The archive `README.md` and `ARTIFACT-CONTENTS.md` both state that the artifact contains the bundled local development policy bundle used for fixed health-check evidence.

## Checksum Verification

Checksum verification passed for both platform artifact packages:

```text
aegis-v0.4.1-macos-arm64.tar.gz: OK
aegis-v0.4.1-macos-x64.tar.gz: OK
```

The known combined-checksum issue remains open. Each matrix artifact still has its own `SHA256SUMS`; a future release-facing workflow should produce one combined manifest across all produced archives.

## Local Path and Secret Marker Review

No maintainer-local path, private key, token value, API key, or committed secret was found.

The desktop binaries still contain GitHub runner source paths and `../examples/policy-bundles/local-dev` strings. These now appear as debug/build metadata and the development fallback path, not as the primary artifact policy bundle path.

The gateway binaries did not show policy bundle path markers.

Generic words such as `password`, `secret`, and `token` appear inside dependency or browser-runtime strings. Exact secret material checks found no private key, token assignment, API key assignment, or credential value.

## Binary Metadata Review

| Binary | Architecture | Dynamic library summary |
| --- | --- | --- |
| `macos-arm64/bin/aegis-gateway` | Mach-O arm64 | `/usr/lib/libSystem.B.dylib` |
| `macos-x64/bin/aegis-gateway` | Mach-O x86_64 | `/usr/lib/libSystem.B.dylib` |
| `macos-arm64/desktop/aegis-desktop` | Mach-O arm64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |
| `macos-x64/desktop/aegis-desktop` | Mach-O x86_64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |

No non-system dynamic library dependency was observed in `otool -L` output.

## Runtime Resolution Assessment

The new artifact layout supports artifact-relative policy bundle resolution.

The desktop resolver checks:

1. the policy bundle next to the extracted artifact root at `policy-bundles/local-dev`
2. the source-tree development fallback under `../examples/policy-bundles/local-dev`

Because the archives now include `policy-bundles/local-dev`, the fixed desktop health-check evidence path does not require `/Users/runner/work/...`, `/Users/...`, or a source checkout for bundled policy material.

The source fallback remains useful for local development. It should not be treated as the primary artifact runtime path.

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
| DA-ECA-001 | Resolved | Artifact evidence confirms the policy bundle is packaged and preferred through artifact-relative resolution. |
| DA-ECA-002 | Deferred | Runner/source debug paths remain in binaries. They do not appear to control runtime policy bundle resolution. |
| DA-ECA-003 | Resolved | Artifact README and content manifest now state that the bundled policy bundle is included. |
| Combined `SHA256SUMS` | Open follow-up | Per-platform checksum manifests still exist. A combined manifest remains future work. |

## Recommendation

PASS WITH FIXES.

The source-path runtime blocker is resolved. Do not start GitHub Release publishing until the remaining release-facing follow-ups are addressed or explicitly accepted:

- strip or remap source/debug paths where practical
- consolidate draft artifact checksums into one manifest

## Required Follow-Up

1. `ci(release): Consolidate draft artifact checksums`
2. `ci(release): Strip or remap draft binary source paths`
3. Re-run artifact review before promoting workflow artifacts into GitHub Release assets.
