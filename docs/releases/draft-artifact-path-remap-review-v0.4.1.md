# AEGIS v0.4.1 Draft Artifact Path Remap Review

## Summary

Recommendation: PASS.

The `Draft Release Artifacts` workflow was re-run after commit `b8213b45f402f053110bd57b89f9a1eec3c7466d`.

The new artifacts preserve the portable archive layout, verify through the combined `SHA256SUMS` manifest, and no longer expose runtime-relevant source policy fallback strings.

The gateway binaries have no scanned high-risk path markers. The desktop binaries still contain one Tauri-generated GitHub runner `src-tauri` context string. That string is residual metadata, not a runtime policy bundle dependency.

DA-ECA-002 is resolved for runtime portability. Residual Tauri toolchain metadata remains deferred release hygiene.

These artifacts are GitHub Actions workflow artifacts for inspection only. They are not GitHub Release assets.

## Workflow Run

| Field | Value |
| --- | --- |
| Workflow | `Draft Release Artifacts` |
| Run ID | `28607528253` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/28607528253` |
| Branch | `main` |
| Commit | `b8213b45f402f053110bd57b89f9a1eec3c7466d` |
| Result | `success` |
| Started | `2026-07-02T17:00:04Z` |
| Completed | `2026-07-02T17:13:13Z` |
| Duration | about 13 minutes 9 seconds |

Job results:

| Job | Result |
| --- | --- |
| `Draft macOS artifact (macos-arm64)` | success |
| `Draft macOS artifact (macos-x64)` | success |
| `Combined draft artifact checksums` | success |

## Artifact Results

The combined workflow artifact `draft-artifacts-v0.4.1` contained:

```text
SHA256SUMS
aegis-v0.4.1-macos-arm64.tar.gz
aegis-v0.4.1-macos-x64.tar.gz
```

The per-platform workflow artifacts also contained their expected platform archives.

## Checksum Verification

Combined checksum verification passed:

```text
aegis-v0.4.1-macos-arm64.tar.gz: OK
aegis-v0.4.1-macos-x64.tar.gz: OK
```

The combined manifest covers produced archive files only. No checksum signature was produced. Signed checksum manifests remain deferred.

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
- `.env`
- private-key-like files
- local temp folders
- absolute-path mirror directories

## Binary Metadata Review

| Binary | Architecture | Dynamic library summary |
| --- | --- | --- |
| `macos-arm64/bin/aegis-gateway` | Mach-O arm64 | `/usr/lib/libSystem.B.dylib` |
| `macos-x64/bin/aegis-gateway` | Mach-O x86_64 | `/usr/lib/libSystem.B.dylib` |
| `macos-arm64/desktop/aegis-desktop` | Mach-O arm64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |
| `macos-x64/desktop/aegis-desktop` | Mach-O x86_64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |

No non-system dynamic library dependency was observed in `otool -L` output.

## Path Marker Review

Scanned markers:

```text
/home/runner/work
/Users/
Users/godzilla
godzilla
Documents/Projects
CARGO_MANIFEST_DIR
../examples/policy-bundles/local-dev
examples/policy-bundles/local-dev
/private/tmp
/var/folders
/mnt/data
```

Results:

| Binary | Result |
| --- | --- |
| `macos-arm64/bin/aegis-gateway` | no high-risk markers |
| `macos-x64/bin/aegis-gateway` | no high-risk markers |
| `macos-arm64/desktop/aegis-desktop` | one `/Users/runner/work/aegis/aegis/src-tauri` string |
| `macos-x64/desktop/aegis-desktop` | one `/Users/runner/work/aegis/aegis/src-tauri` string |

The desktop binaries did not contain `CARGO_MANIFEST_DIR`, `../examples/policy-bundles/local-dev`, or `examples/policy-bundles/local-dev`.

The desktop binaries do contain `policy-bundles/local-dev`, which is the intended artifact-relative policy bundle path.

The desktop binaries also contain remapped dependency metadata under `~/.cargo/...`. That is toolchain-generated metadata and not a maintainer-local or GitHub workspace path.

## Runtime Portability Assessment

The artifact layout remains portable for the fixed health-check evidence path:

1. The archives include `policy-bundles/local-dev`.
2. The desktop resolver prefers the artifact-relative policy bundle.
3. The source-tree policy fallback string is absent from release desktop binaries.
4. `CARGO_MANIFEST_DIR` is absent from the scanned release binaries.
5. The gateway binaries have no scanned high-risk path markers.

The remaining `/Users/runner/work/aegis/aegis/src-tauri` string appears once in each desktop binary as Tauri-generated context metadata. It does not point to the policy bundle, does not include the source-tree fallback path, and does not appear to control runtime policy resolution.

## Release Boundary Verification

Verified:

- no `v0.4.1` tag exists on origin
- no `v0.4.1` GitHub Release exists
- no public release assets exist
- no signing was performed
- no notarization was performed
- no installers were produced
- no auto-update metadata was produced

## DA-ECA-002 Status

Resolved for runtime portability; residual toolchain metadata deferred.

The previous runtime concern was source-path dependence. Fresh artifact evidence shows the release binaries no longer contain the source-tree policy fallback string or `CARGO_MANIFEST_DIR`, and the archive contains the artifact-relative policy bundle.

The remaining Tauri desktop context string is release hygiene, not a runtime portability blocker.

## Recommendation

PASS.

The draft artifacts satisfy the path-remapping review for runtime portability. Do not claim perfect binary cleanliness. Continue to describe the artifacts as unsigned, not notarized, developer-preview workflow artifacts.

## Required Follow-Up

1. Keep residual Tauri-generated source metadata tracked as release hygiene.
2. Continue publishing work only in a separate approved task.
3. Keep GitHub Release publishing, public release assets, tags, signing, notarization, installers, and auto-update out of scope until that work is explicitly scheduled.
