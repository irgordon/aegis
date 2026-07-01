# AEGIS First Downloadable Artifact Targets

## Decision Summary

The first downloadable developer-preview release should be `v0.4.1`.

It should publish archive-based artifacts through a future draft GitHub Release after maintainer review.

Before publishing exists, `.github/workflows/draft-artifacts.yml` builds draft macOS archive artifacts for inspection as GitHub Actions workflow artifacts only.

Stage 1 should target macOS arm64 and macOS x64.

The preferred artifact scope is one archive per platform containing the desktop app and the gateway binary when practical.

The archive should also include the local development policy bundle required by fixed desktop health-check evidence.

The release remains pre-alpha, local-only, unsigned, and not production-ready.

## Baseline

`v0.4.0` is complete, tagged, and pushed.

It remains source-only.

It has no downloadable artifacts, installers, packages, signed artifacts, notarization, auto-update, or GitHub Release assets.

This decision does not change `v0.4.0`.

## Target Version

Decision: use `v0.4.1` for the first downloadable developer-preview artifact set.

Rationale:

- `v0.4.0` is complete and must not be expanded.
- The first downloadable build is packaging and distribution work, not a major runtime capability.
- A `v0.4.1` developer preview preserves the `v0.4.0` baseline while making AEGIS easier to try.
- `v0.5.0` should remain available for larger capability work unless maintainers deliberately rebaseline the phasemap.

## Target Platforms

Use a staged platform rollout.

| Stage | Platforms | Status |
| --- | --- | --- |
| 1 | macOS arm64, macOS x64 | Selected first. |
| 2 | Windows x64 | Planned after Stage 1 is stable. |
| 3 | Linux x64 | Planned after Stage 1 is stable. |

Reason:

- Current active validation and desktop launch work happens on macOS.
- Starting with macOS lowers initial distribution risk.
- Windows and Linux should follow after artifact layout, naming, checksum, and release review are stable.

Do not claim support for a platform until its build and validation path exists.

## Artifact Contents

Preferred scope: desktop app plus gateway binary in one archive per platform.

This matches the `v0.4.0` minimum usable local release:

- the desktop app is the operator surface
- the gateway binary supports local validation and smoke testing
- the bundled local development policy bundle lets fixed health-check evidence run without a source checkout
- both are useful for a developer preview

If bundling both is not practical in the first implementation pass, use desktop app first and document the gateway binary as a source-build fallback.

## Artifact Formats

Use archive files, not installers.

Selected formats:

| Platform | Format |
| --- | --- |
| macOS arm64 | `.tar.gz` |
| macOS x64 | `.tar.gz` |
| Windows x64 | `.zip` |
| Linux x64 | `.tar.gz` |

Deferred formats:

- `.dmg`
- `.pkg`
- `.msi`
- `.exe` installer
- `.AppImage`
- `.deb`
- `.rpm`
- package repositories
- auto-update metadata

## Artifact Naming

Use one artifact per platform when practical:

```text
aegis-v0.4.1-macos-arm64.tar.gz
aegis-v0.4.1-macos-x64.tar.gz
aegis-v0.4.1-windows-x64.zip
aegis-v0.4.1-linux-x64.tar.gz
SHA256SUMS
```

If desktop and gateway must ship separately, use:

```text
aegis-desktop-v0.4.1-macos-arm64.tar.gz
aegis-gateway-v0.4.1-macos-arm64.tar.gz
```

Prefer one combined platform archive unless implementation proves it is unsafe or confusing.

## Checksum Requirements

A checksum is a file fingerprint.

Every downloadable artifact must have a SHA-256 checksum.

All checksums must be listed in `SHA256SUMS`.

`SHA256SUMS` must be published with the draft GitHub Release.

Checksum generation must happen after artifact creation.

Release notes must explain how to verify checksums.

Do not require `SHA256SUMS.sig` for the first developer-preview release unless signing is deliberately scheduled.

Signed checksum manifests are future work.

## Draft Workflow Status

The draft artifact workflow exists for inspection only.

It:

- runs manually through GitHub Actions
- targets macOS arm64 and macOS x64 archives
- includes the gateway binary and desktop binary output when the build succeeds
- stages the local development policy bundle needed for fixed health-check evidence
- generates one combined `SHA256SUMS` manifest for produced archives
- uploads GitHub Actions workflow artifacts only

Combined checksum support is verified in workflow artifacts. The combined `SHA256SUMS` manifest covers both macOS draft archives from the latest review run.

It does not create a GitHub Release, publish release assets, tag `v0.4.1`, sign artifacts, notarize artifacts, create installers, or auto-update anything.

## Release Notes Requirements

The first downloadable release notes must state:

- release version
- target platforms
- artifact names
- checksum verification steps
- build validation summary
- known limitations
- pre-alpha warning
- unsigned artifact warning
- no installer warning
- no auto-update warning

Release notes must not claim production readiness, enterprise readiness, normal installability, or platform trust.

## User Warning Requirements

Future GitHub Release text must include this warning:

```text
Developer preview. Unsigned artifacts. Local-only. Not production-ready.
Not enterprise-hardened. No installer. No auto-update.
Validate SHA-256 checksums before use.
```

Use plain language near the top of the release body.

## Deferred Items

The first downloadable developer preview does not include:

- GitHub Release auto-publishing
- installers
- `.dmg`
- `.pkg`
- `.msi`
- `.exe` installers
- `.AppImage`
- `.deb`
- `.rpm`
- package repositories
- code signing
- macOS notarization
- Windows code signing
- Linux package signing
- signed checksum manifests
- auto-update
- production credential providers
- runtime behavior changes
- UI authority changes

## Validation Requirements

The future artifact workflow must verify:

- `python3 scripts/verify.py` passes
- release validation gate or successor gate passes
- desktop crate builds
- gateway binary builds
- UI scaffold tests pass
- artifacts are generated from a clean tag
- artifact names match this document
- `SHA256SUMS` includes every published artifact
- release notes include required warnings
- artifacts and logs contain no secrets

Do not implement these checks in this task.

## Rationale

This decision favors the smallest useful downloadable release.

It keeps `v0.4.0` closed and source-only.

It avoids pulling installer, signing, notarization, and auto-update work into the first artifact pass.

It gives the next implementation task enough detail to create a draft build workflow without guessing.

## Open Follow-Up Decisions

- Can the desktop app and gateway binary be bundled cleanly in one archive per platform?
- Which exact macOS app bundle layout should be used inside the archive?
- Should Stage 1 include both macOS architectures in the first workflow pass or one at a time?
- Should checksum manifests be signed in `v0.4.1` or a later version?
- What validation evidence should be attached to the draft GitHub Release?

## Next Implementation Tasks

1. `ci(release): Add draft artifact build workflow` - complete
2. `test(release): Validate artifact naming and checksum generation`
3. `docs(release): Draft v0.4.1 developer-preview release notes`
4. `chore(release): Publish first unsigned developer-preview build`

Do not begin implementation until maintainers approve the workflow scope.
