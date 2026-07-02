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

Two Phase 5 release workflows now exist.

`.github/workflows/draft-artifacts.yml` builds inspection-only workflow artifacts.

The draft artifact workflow is manually triggered. It builds macOS draft archives, generates one combined `SHA256SUMS` manifest from the produced archives, and uploads GitHub Actions workflow artifacts only.

The draft archives stage the local development policy bundle needed by fixed desktop health-check evidence. This avoids depending on the build machine source checkout for the UI evidence path.

Combined checksum support is verified in workflow artifacts. The combined `SHA256SUMS` manifest covers both macOS draft archives from the latest review run.

The draft artifact workflow does not create a GitHub Release, upload release assets, tag `v0.4.1`, sign artifacts, notarize artifacts, create installers, or auto-update anything.

`.github/workflows/draft-github-release.yml` can create or update a draft GitHub Release for maintainer review.

The draft GitHub Release workflow is manually triggered from the existing `v0.4.1` tag ref. It rebuilds the macOS archives, generates and verifies one combined `SHA256SUMS`, requires the existing `v0.4.1` tag, refuses to modify a non-draft release, and attaches only the expected archives plus `SHA256SUMS`.

It does not publish the release, create or move tags, sign artifacts, notarize artifacts, create installers, or auto-update anything.

The draft GitHub Release workflow should be verified in a separate workflow run before any public publishing step.

A future publishing workflow should:

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
| `v0.4.1` | Selected first downloadable developer-preview target and active Phase 5 distribution milestone. |
| `v0.5.0` | Developer experience after downloadable evaluation works. |
| `v0.6.0` | Production distribution after developer-preview artifacts are proven. |
| Later | Signed, notarized, installer-based, and broader user-facing artifacts. |

Phase 5 now focuses on developer distribution. The first downloadable artifact target remains `v0.4.1`.

## Open Decisions

- Should checksum manifests be signed in `v0.4.1` or a later version?
- What validation evidence should be reviewed before a draft release is published?
- Which later platform should follow macOS after Stage 1 is verified?

## Recommended Next Tasks

1. `test(release): Verify draft GitHub Release`
2. `docs(release): Draft v0.4.1 developer-preview release notes`
3. `test(release): Verify developer download and portable launch`
4. `chore(release): Publish first unsigned developer-preview build`

Keep installers, signing, notarization, auto-update, production credentials, replay execution, and approval workflow out of Phase 5.
