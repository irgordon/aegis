# AEGIS Release Distribution Plan

## Scope

This document defines the post-`v0.4.0` distribution path for AEGIS.

It is a planning document. It does not publish releases, create public downloadable assets, add installers, sign artifacts, or notarize app bundles.

The first concrete artifact target decision is recorded in `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md`.

## Current Baseline

`v0.4.0` is complete.

`v0.4.0` is:

- local-only
- pre-alpha
- developer-oriented
- source-oriented
- not production-ready
- not enterprise-hardened

`v0.4.0` has no GitHub Release assets, installers, packages, signed artifacts, notarized app bundles, or auto-update support.

Do not retroactively expand `v0.4.0`.

## Distribution Goals

The next distribution step should make AEGIS easier to try without requiring a full local development setup.

Distribution should:

- publish downloadable artifacts through GitHub Releases
- preserve release validation before publishing
- include clear pre-alpha warnings
- avoid implying production readiness
- avoid implying enterprise support
- avoid bypassing governance, tests, or release review

## Non-Goals

This plan does not implement:

- GitHub Release publishing
- release artifacts
- cross-platform binaries
- installers
- packaging scripts
- signing
- notarization
- auto-update
- release workflow automation
- runtime behavior
- UI behavior
- schema, policy, approval, credential, wrapper, audit, state, or recovery changes

## Release Channels

AEGIS should use staged release channels.

| Channel | Purpose | Status |
| --- | --- | --- |
| Source release | Tag and source checkout only. | `v0.4.0` complete. |
| Pre-alpha downloadable build | Unsigned downloadable developer-preview artifacts with checksums and strong warnings. | First target selected in `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md`. |
| Unsigned developer preview | Early artifact channel for maintainers and technical evaluators. | Draft workflow artifacts exist for inspection only. |
| Signed/notarized release | Downloadable artifacts with platform trust controls. | Deferred. |
| Installer-based release | Installer packages for normal user installation. | Deferred. |

## Target Platforms

Planned platform targets:

- macOS arm64
- macOS x64
- Windows x64
- Linux x64

Optional future targets:

- Linux arm64
- Windows arm64

No platform is supported until its build, validation, checksum, and release notes path exists.

The first selected rollout starts with macOS arm64 and macOS x64. Windows x64 and Linux x64 follow after the Stage 1 artifact path is stable.

## Artifact Types

An artifact is a downloadable file attached to a release.

Early pre-alpha artifacts may include:

- compressed desktop app bundles
- compressed gateway binaries where appropriate
- release notes
- validation summary
- `SHA256SUMS`

Later artifacts may include:

- macOS `.dmg`
- Windows `.msi` or `.exe` installer
- Linux `.AppImage`
- Linux `.deb`
- Linux `.rpm`
- signed binaries
- notarized macOS app
- auto-update metadata

Installer formats, signing, notarization, and auto-update metadata are deferred until deliberately scheduled.

## Artifact Naming

Use a consistent naming pattern:

```text
aegis-v<version>-<platform>-<arch>.<ext>
```

Examples:

```text
aegis-v0.4.1-macos-arm64.tar.gz
aegis-v0.4.1-macos-x64.tar.gz
aegis-v0.4.1-windows-x64.zip
aegis-v0.4.1-linux-x64.tar.gz
SHA256SUMS
SHA256SUMS.sig
```

Exact extensions should be selected when the first artifact target is chosen.

For the first developer preview, `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md` selects one combined archive per platform when practical.

## Checksums and Integrity

A checksum is a fingerprint for a file. If the file changes, the fingerprint changes.

Every published artifact must have a SHA-256 checksum.

Checksums must be included in a release checksum manifest named `SHA256SUMS`.

The checksum manifest should be attached to the GitHub Release.

Future work should sign the checksum manifest before broader user-facing distribution.

Do not publish unsigned binaries without clear pre-alpha warnings.

## Signing and Notarization

Signing proves who produced an artifact.

Notarization is an Apple review step that lets macOS recognize an app as checked by Apple.

Signing and notarization are deferred unless deliberately scheduled.

Unsigned artifacts may be acceptable only for an early pre-alpha developer preview if they are clearly labeled.

Signed and notarized artifacts are required before broader user-facing distribution.

macOS notarization should exist before AEGIS claims normal macOS installability.

Windows code signing should exist before AEGIS claims trusted Windows distribution.

Linux package signing should exist before package repository distribution.

Do not store signing keys, certificates, or secrets in this repository.

## GitHub Release Workflow

A draft artifact build workflow now exists at `.github/workflows/draft-artifacts.yml`.

The workflow is manually triggered. It builds macOS draft archives, generates one combined `SHA256SUMS` manifest from the produced archives, and uploads GitHub Actions workflow artifacts only.

The draft archives stage the local development policy bundle needed by fixed desktop health-check evidence. This avoids depending on the build machine source checkout for the UI evidence path.

Combined checksum support is implemented in the workflow source. Artifact-level verification is pending a new workflow run.

It does not create a GitHub Release, upload release assets, tag `v0.4.1`, sign artifacts, notarize artifacts, create installers, or auto-update anything.

A future GitHub Release workflow should:

1. Trigger on an approved version tag.
2. Run release validation.
3. Build platform artifacts.
4. Generate `SHA256SUMS`.
5. Upload artifacts to the matching GitHub Release.
6. Attach release notes.
7. Fail closed if validation fails.
8. Avoid publishing partial releases.

Manual guardrails still apply:

- maintainer approval before tagging
- no forced tags
- no publishing from a dirty state
- no release if validation fails
- release notes must match the tag

## Validation Requirements

Distribution builds must require:

- `python3 scripts/verify.py`
- the current release validation gate, such as `bash scripts/validate-v0.4.0-release.sh` or its successor
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- desktop crate validation
- UI scaffold tests
- artifact checksum verification
- clean working tree before tag

Future release gates should be versioned or renamed when the release target changes.

## Security Boundaries

Downloadable artifacts do not add runtime authority.

The UI remains an operator surface, not an authority boundary.

Downloadability does not imply production readiness.

Unsigned builds must be clearly labeled.

CI logs must not expose secrets.

Signing keys must never be committed to the repository.

No release workflow may bypass policy, validation, tests, or release review.

## Version Sequencing

Recommended sequence:

| Version | Distribution posture |
| --- | --- |
| `v0.4.0` | Source-only local release. Complete. |
| `v0.4.1` | Selected first packaging-only developer-preview target. |
| `v0.5.0` | First downloadable cross-platform developer-preview target only if maintainers reassign the next milestone from recovery to distribution. |
| Later | Signed, notarized, installer-based, and broader user-facing artifacts. |

Because `docs/PHASEMAP.md` currently assigns `v0.5.0` to recovery and replay execution, the first downloadable artifact target is `v0.4.1` unless maintainers deliberately rebaseline the phasemap.

## Open Decisions

- Can the desktop app and gateway binary be bundled cleanly in one archive per platform?
- Should Stage 1 build both macOS architectures in one workflow pass or one at a time?
- Should checksum manifests be signed in `v0.4.1` or a later version?
- Should distribution ship desktop-only artifacts, gateway-only artifacts, or both?

## Recommended Next Tasks

1. `ci(release): Add draft artifact build workflow` - complete
2. `test(release): Validate artifact naming and checksum generation`
3. `docs(release): Draft v0.4.1 developer-preview release notes`
4. `chore(release): Publish first unsigned developer-preview build`

Do not begin these tasks until maintainers approve the first distribution target and version.
