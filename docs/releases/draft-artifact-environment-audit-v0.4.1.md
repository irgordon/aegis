# AEGIS v0.4.1 Draft Artifact Environment Coupling Audit

## Summary

The `Draft Release Artifacts` workflow artifacts were downloaded, extracted, and inspected.

The archive contents are small and match the intended draft shape. Checksums verify. No maintainer username, maintainer project path, private key, token value, or committed secret was found.

The artifacts are not portable enough for release publishing yet.

The desktop binary embeds the GitHub runner source path through `CARGO_MANIFEST_DIR` and uses it to resolve the local policy bundle. That means live desktop evidence can depend on a path that exists only in the build environment.

Recommendation: FAIL.

This failure does not mean a GitHub Release was published. It means the current draft artifacts should not be promoted into release assets until the path dependency is removed.

Post-audit fix status: source changes now prefer an artifact-relative bundled policy bundle and stage the local development policy bundle into draft artifacts. The artifact-level re-run in `docs/releases/draft-artifact-portability-rerun-v0.4.1.md` confirms DA-ECA-001 is resolved.

DA-ECA-002 fix status: source and workflow changes now keep the source-tree policy fallback out of release desktop builds and configure draft release builds with Rust path remapping plus debuginfo stripping. Artifact-level verification in `docs/releases/draft-artifact-path-remap-review-v0.4.1.md` confirms the gateway binaries have no scanned high-risk path markers and the desktop binaries no longer contain `CARGO_MANIFEST_DIR` or `../examples/policy-bundles/local-dev`. One Tauri-generated desktop context string still contains `/Users/runner/work/aegis/aegis/src-tauri`. That residual metadata is deferred release hygiene, not a runtime portability blocker.

## Artifact Source

| Field | Value |
| --- | --- |
| Workflow | `Draft Release Artifacts` |
| Run ID | `28484349169` |
| Run URL | `https://github.com/irgordon/aegis/actions/runs/28484349169` |
| Branch | `main` |
| Commit | `9e4afa392b936d0c9bc603b90649235df5c9c196` |
| Result | `success` |
| Inspection path | `/tmp/aegis-artifact-coupling-audit-28484349169` |

The temporary inspection path was used only to download and extract workflow artifacts. It is not evidence of shipped runtime coupling.

## Files Inspected

Downloaded workflow artifacts:

- `draft-macos-arm64-v0.4.1`
- `draft-macos-x64-v0.4.1`

Archives inspected:

- `aegis-v0.4.1-macos-arm64.tar.gz`
- `aegis-v0.4.1-macos-x64.tar.gz`

Repository files reviewed:

- `.github/workflows/draft-artifacts.yml`
- `docs/releases/artifact-readme-v0.4.1.md`
- `docs/releases/draft-artifact-workflow-review-v0.4.1.md`
- `docs/FIRST_DOWNLOADABLE_ARTIFACTS.md`
- `docs/RELEASE_DISTRIBUTION_PLAN.md`
- `docs/wiki/10-contributor-guide.md`
- `scripts/`
- `src-tauri/src/main.rs`

## Archive Tree Review

Each archive contained only:

```text
README.md
ARTIFACT-CONTENTS.md
bin/aegis-gateway
desktop/aegis-desktop
```

No extracted archive contained:

- `.git`
- `.github`
- `target`
- `node_modules`
- build caches
- Cargo registry caches
- `.env`
- private-key-like files
- local temp folders
- absolute-path mirror directories

## Local Path and Secret Marker Review

The extracted Markdown files had no matches for maintainer paths or secret markers.

Binary string scans found GitHub-hosted runner source paths such as:

```text
/Users/runner/.cargo/registry/...
/Users/runner/work/aegis/aegis/src-tauri
```

These are not maintainer-local paths and are not secret values. They are still environment markers inside draft binaries.

The desktop binary also contains:

```text
../examples/policy-bundles/local-dev
```

That relative segment is joined to the embedded `CARGO_MANIFEST_DIR` path at runtime.

Generic words such as `token`, `secret`, `username`, and `password` appeared inside dependency or browser-runtime strings. No actual credential value, token value, private key, API key, or secret material was found.

## Binary Metadata Review

| Binary | Architecture | Dynamic library summary |
| --- | --- | --- |
| `macos-arm64/bin/aegis-gateway` | Mach-O arm64 | `/usr/lib/libSystem.B.dylib` |
| `macos-x64/bin/aegis-gateway` | Mach-O x86_64 | `/usr/lib/libSystem.B.dylib` |
| `macos-arm64/desktop/aegis-desktop` | Mach-O arm64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |
| `macos-x64/desktop/aegis-desktop` | Mach-O x86_64 | macOS system frameworks including WebKit, AppKit, Foundation, and libSystem |

No non-system dynamic library dependency was observed in the `otool -L` output.

## Runtime Assumption Review

The gateway binary can be inspected from the archive. Normal governed gateway use still requires a request file and a verified local policy bundle.

The desktop binary has a stronger problem. The fixed live evidence path uses:

```rust
PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/policy-bundles/local-dev")
```

In the GitHub Actions build, that compiles to a path under:

```text
/Users/runner/work/aegis/aegis/src-tauri
```

The archive does not include `examples/policy-bundles/local-dev`. A user who unpacks the artifact outside the GitHub runner workspace will not have that path. Live desktop evidence may therefore fail or fall back because the policy bundle cannot be found.

The health-check request is embedded with `include_str!`, so the request fixture itself is not the problem.

## Workflow Assumption Review

The workflow is present on `origin/main`.

The workflow remains:

- manual through `workflow_dispatch`
- non-publishing
- workflow-artifact only
- limited to `actions/upload-artifact`
- not tied to maintainer-local paths
- not creating a GitHub Release
- not creating a tag
- not signing
- not notarizing
- not creating installers

The workflow uses GitHub-hosted macOS runner paths and standard macOS runner tools. No maintainer username, maintainer home directory, Homebrew-only setup, or unpublished local file was found in the workflow.

## Documentation Assumption Review

The checked-in artifact README gives the correct release warnings:

- unsigned
- not notarized
- local-only
- pre-alpha
- not production-ready
- archive, not installer
- checksum should be validated

Before this audit, the artifact README did not clearly state what runtime material was inside the archive. The checked-in artifact README used for later draft artifacts now identifies the bundled local development policy bundle and keeps request fixtures out of the archive.

Existing distribution documents still correctly state:

- `v0.4.0` is complete and source-only
- `v0.4.1` is not tagged
- `v0.4.1` is not published
- workflow artifacts are not GitHub Release assets
- no public downloads exist
- no installers exist
- no signing or notarization exists
- the combined `SHA256SUMS` manifest is verified in workflow artifacts

After this audit, the checked-in artifact README and draft workflow were updated to include the local development policy bundle needed for fixed health-check evidence. The inspected workflow run did not contain that fix.

## Findings

### DA-ECA-001

Severity: P1

Location:

- `src-tauri/src/main.rs`
- `desktop/aegis-desktop`

Evidence:

The desktop live evidence path resolves the policy bundle from `env!("CARGO_MANIFEST_DIR")`. Binary strings show `/Users/runner/work/aegis/aegis/src-tauri` and `../examples/policy-bundles/local-dev` in the draft desktop binaries.

Impact:

The desktop artifact can depend on the GitHub runner source path for live backend evidence. A user unpacking the archive elsewhere will not have that path or the policy bundle. This blocks draft GitHub Release publishing.

Recommended fix:

Resolve the policy bundle from a packaged resource path or include the required local development policy bundle as an explicit artifact resource. The runtime must not depend on the build machine source directory.

Fix status:

Resolved by artifact-level evidence in workflow run `28548645224`.

### DA-ECA-002

Severity: P2

Location:

- `bin/aegis-gateway`
- `desktop/aegis-desktop`

Evidence:

Binary strings include GitHub runner source paths under `/Users/runner/.cargo/registry/...` and `/Users/runner/work/aegis/aegis/...`.

Impact:

These paths do not appear to control runtime behavior, but they are environment markers in draft binaries. They should be removed or remapped before broader distribution.

Recommended fix:

Use a release build configuration that strips or remaps source paths in binaries where practical.

Fix status:

Resolved for runtime portability. Workflow run `28607528253` confirms the gateway binaries have no scanned high-risk path markers. The desktop binaries no longer contain `CARGO_MANIFEST_DIR`, `../examples/policy-bundles/local-dev`, or `examples/policy-bundles/local-dev`. One Tauri-generated desktop context string still contains `/Users/runner/work/aegis/aegis/src-tauri`; this is tracked as residual release hygiene, not a runtime policy dependency.

### DA-ECA-003

Severity: P2

Location:

- `README.md` inside the inspected archives
- `docs/releases/artifact-readme-v0.4.1.md`

Evidence:

The inspected archive README did not state clearly whether the archive contained runtime policy material or request fixtures.

Status:

Resolved for the rerun artifacts. The archive README and content manifest now state that the bundled local development policy bundle is included.

Impact:

Operators could mistake the draft archive for a self-contained runnable package.

Recommended fix:

Keep the artifact README explicit that these are inspection artifacts with a bundled local development policy bundle, no request fixtures, no installers, no signing, and no production configuration. This repository wording has been clarified for future draft artifact runs.

## Classification

Original audit classification: environment-coupled artifact.

Current follow-up classification: portable for runtime policy bundle resolution with residual toolchain metadata deferred.

The original inspected artifacts depended on a build-environment source path for live desktop evidence. Later artifacts include the required `policy-bundles/local-dev` bundle and no longer contain the source-tree policy fallback in release desktop binaries.

## Required Fixes Before Publishing

The original runtime portability blockers have follow-up artifact evidence:

1. The desktop binary uses the bundled policy bundle path.
2. The staged `policy-bundles/local-dev` directory contains only runtime policy verification files.
3. The combined `SHA256SUMS` manifest verifies in workflow artifacts.
4. DA-ECA-002 is resolved for runtime portability.

Residual Tauri-generated source metadata remains deferred release hygiene and should not be described as perfect binary cleanliness.

## Recommendation

Original recommendation: FAIL.

The release boundary still held. No release was published and no public assets were created.

Current follow-up recommendation: PASS for runtime portability.

The artifact portability boundary now holds for the fixed health-check evidence path. GitHub Release publishing, public release assets, tags, signing, notarization, installers, and auto-update remain separate future work.
