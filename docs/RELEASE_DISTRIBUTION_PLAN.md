# AEGIS Release Distribution Plan

## Scope

This document defines the post-`v0.4.0` distribution path for AEGIS.

It began as a planning document. The first planned public Developer Preview has now been published as `v0.4.1`.

This document still does not add installers, sign artifacts, notarize app bundles, or change runtime behavior.

**Latest published release:** `v0.4.1 Developer Preview`.

**Current development target:** `v0.4.2 Developer Preview Refresh`.

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

`v0.4.1` is the latest published Developer Preview.

`v0.4.1` is:

- publicly downloadable from GitHub Releases
- prerelease
- unsigned
- not notarized
- archive-based
- checksum-verified with `SHA256SUMS`
- not production-ready
- not enterprise-hardened

The immutable `v0.4.1` artifacts do not include the later request fixture, conventional CLI help, or corrected desktop identity. Those current-development improvements target `v0.4.2`.

## Distribution Goals

The distribution path should make AEGIS easier to try without requiring a full local development setup.

Distribution should:

- publish downloadable artifacts through GitHub Releases
- preserve release validation before publishing
- include clear Developer Preview warnings
- avoid implying production readiness
- avoid implying enterprise support
- avoid bypassing governance, tests, or release review

## Non-Goals

This plan does not implement:

- cross-platform binaries
- installers
- packaging scripts
- signing
- notarization
- auto-update
- release workflow automation beyond the current manual Developer Preview path
- runtime behavior
- UI behavior
- schema, policy, approval, credential, wrapper, audit, state, or recovery changes

## Release Channels

AEGIS should use staged release channels.

| Channel | Purpose | Status |
| --- | --- | --- |
| Source release | Tag and source checkout only. | `v0.4.0` complete. |
| Developer Preview download | Unsigned downloadable developer-preview artifacts with checksums and strong warnings. | `v0.4.1` published as a prerelease. |
| Unsigned developer preview | Early artifact channel for maintainers and technical evaluators. | Public for macOS arm64 and macOS x64 through GitHub Releases. |
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

Early Developer Preview artifacts may include:

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
aegis-v0.5.0-windows-x64.zip
aegis-v0.5.0-linux-x64.tar.gz
SHA256SUMS
SHA256SUMS.sig
```

The macOS names describe the published release. The Windows and Linux names
describe the planned `v0.5.0` cross-platform outcome and are not published
assets.

Exact extensions should be selected when the first artifact target is chosen.

For the first developer preview, `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md` selects one combined archive per platform when practical.

## Checksums and Integrity

A checksum is a fingerprint for a file. If the file changes, the fingerprint changes.

Every published artifact must have a SHA-256 checksum.

Checksums must be included in a release checksum manifest named `SHA256SUMS`.

The checksum manifest should be attached to the GitHub Release.

Future work should sign the checksum manifest before broader user-facing distribution.

Do not publish unsigned binaries without clear Developer Preview warnings.

## Signing and Notarization

Signing proves who produced an artifact.

Notarization is an Apple review step that lets macOS recognize an app as checked by Apple.

Signing and notarization are deferred unless deliberately scheduled.

Unsigned artifacts may be acceptable only for an early Developer Preview if they are clearly labeled.

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

The current-development draft artifact workflow targets `v0.4.2`. It does not create a GitHub Release, upload release assets, create or move tags, sign artifacts, notarize artifacts, create installers, or auto-update anything.

`.github/workflows/draft-github-release.yml` can create or update a draft GitHub Release for maintainer review.

The current-development draft GitHub Release workflow is manually triggered from a maintainer-created `v0.4.2` tag ref. It rebuilds the macOS archives, generates and verifies one combined `SHA256SUMS`, requires the existing tag, refuses to modify a non-draft release, and attaches only the expected archives plus `SHA256SUMS`.

It does not publish the release, create or move tags, sign artifacts, notarize artifacts, create installers, or auto-update anything.

The historical `v0.4.1` draft GitHub Release workflow was verified before the first public publishing step. The current-development workflow targets a new immutable `v0.4.2` tag and must be verified again before publication.

`v0.4.1` is now published as a public prerelease Developer Preview. The release contains the expected two macOS archives and one combined `SHA256SUMS` manifest.

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
| `v0.4.1` | First public downloadable Developer Preview. Published as a prerelease with macOS archives and `SHA256SUMS`. |
| `v0.4.2` | Developer Preview Refresh under Release Truth governance. |
| `v0.5.0` | Windows x64 and Linux x64 Developer Preview outcome. |
| Later | Signed, notarized, installer-based, and broader user-facing artifacts. |

P1 Complete Phase 5 Developer Distribution is active. It first validates the
macOS-only `v0.4.2` refresh and publishes it only after artifact evidence
passes, then validates Windows x64 and Linux x64 as the separate `v0.5.0`
outcome. Windows ARM64 and Linux ARM64 remain deferred. Phase 6 remains an
engineering phase without a reserved version.

## Open Decisions

- Should checksum manifests be signed in a later version?
- Which later platform should follow macOS after Stage 1 is verified?

## Recommended Next Tasks

1. `chore(release): Validate v0.4.2 Developer Preview Refresh`
2. `test(release): Record v0.4.2 artifact evidence`
3. `test(release): Validate Windows x64 artifact path for v0.5.0`
4. `test(release): Validate Linux x64 artifact path for v0.5.0`

Keep installers, signing, notarization, auto-update, production credentials, replay execution, and approval workflow out of Phase 5.
