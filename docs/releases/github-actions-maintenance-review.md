# GitHub Actions Maintenance Review

## Purpose

This review records the bounded GitHub Actions dependency and runner-image
maintenance performed outside the frozen `v0.4.2` artifact candidate.

The work removes Node 20 action runtimes, pins stable runner labels, preserves
artifact and release boundaries, and adds static regression checks. It does not
add product capability or authorize tagging or publication.

## Scope Result

The maintenance scope is acceptable because it reduces release engineering
debt without changing gateway, desktop, policy, archive assembly, checksum,
tagging, or publication behavior.

The following files remain outside scope:

- runtime and desktop source
- public release claims
- release artifact contents
- roadmap and phasemap
- `config/release-truth.json`

## Upstream Review

Official upstream sources reviewed on 2026-07-14:

- [actions/checkout v7.0.0](https://github.com/actions/checkout/releases/tag/v7.0.0)
- [actions/checkout v7 definition](https://raw.githubusercontent.com/actions/checkout/v7/action.yml)
- [actions/setup-python v6.0.0](https://github.com/actions/setup-python/releases/tag/v6.0.0)
- [actions/setup-python v6 definition](https://raw.githubusercontent.com/actions/setup-python/v6/action.yml)
- [actions/upload-artifact v7.0.0](https://github.com/actions/upload-artifact/releases/tag/v7.0.0)
- [actions/upload-artifact v7 definition](https://raw.githubusercontent.com/actions/upload-artifact/v7/action.yml)
- [actions/download-artifact v8.0.0](https://github.com/actions/download-artifact/releases/tag/v8.0.0)
- [actions/download-artifact v8 definition](https://raw.githubusercontent.com/actions/download-artifact/v8/action.yml)
- [GitHub-hosted runner image labels](https://github.com/actions/runner-images)
- [dtolnay/rust-toolchain usage](https://github.com/dtolnay/rust-toolchain)

The selected GitHub actions use Node 24 and require runner version `2.327.1`
or later where documented. AEGIS uses GitHub-hosted runners, which GitHub
maintains, rather than self-hosted runners with an unknown runner version.

## Action Audit

| Workflow | Action | Uses | Before | Selected | Runtime |
| --- | --- | ---: | --- | --- | --- |
| `validate.yml` | `actions/checkout` | 3 | `v4` | `v7` | Node 24 |
| `validate.yml` | `actions/setup-python` | 1 | `v5` | `v6` | Node 24 |
| `validate.yml` | `dtolnay/rust-toolchain` | 2 | `stable` | `stable` | Composite shell action |
| `draft-artifacts.yml` | `actions/checkout` | 1 | `v4` | `v7` | Node 24 |
| `draft-artifacts.yml` | `actions/setup-python` | 1 | `v5` | `v6` | Node 24 |
| `draft-artifacts.yml` | `dtolnay/rust-toolchain` | 1 | `stable` | `stable` | Composite shell action |
| `draft-artifacts.yml` | `actions/upload-artifact` | 2 | `v4` | `v7` | Node 24 |
| `draft-artifacts.yml` | `actions/download-artifact` | 1 | `v4` | `v8` | Node 24 |
| `draft-github-release.yml` | `actions/checkout` | 2 | `v4` | `v7` | Node 24 |
| `draft-github-release.yml` | `actions/setup-python` | 1 | `v5` | `v6` | Node 24 |
| `draft-github-release.yml` | `dtolnay/rust-toolchain` | 1 | `stable` | `stable` | Composite shell action |
| `draft-github-release.yml` | `actions/upload-artifact` | 1 | `v4` | `v7` | Node 24 |
| `draft-github-release.yml` | `actions/download-artifact` | 1 | `v4` | `v8` | Node 24 |

`dtolnay/rust-toolchain@stable` remains unchanged because upstream documents
the action revision as the Rust toolchain selector and recommends this exact
form for current stable Rust. It is not a Node-based GitHub action.

## Breaking Changes Reviewed

### Checkout v7

The v7 fork-protection change affects unsafe checkout from `pull_request_target`
or `workflow_run`. AEGIS uses neither trigger. The existing `fetch-depth: 0`
input remains only on the draft release job that needs tag history.

Every checkout now sets `persist-credentials: false`. No workflow performs an
authenticated Git push. The release job uses its bounded `github.token`
directly through `gh` only after tag and asset validation.

### Setup Python v6

The Node 24 runtime and minimum runner requirement were reviewed. The existing
`python-version: 3.12` input remains unchanged and supported.

### Upload Artifact v7

Version 7 adds ESM packaging and optional direct, unarchived uploads. AEGIS
sets `archive: true`, preserving the existing workflow-artifact container. It
also makes the existing defaults explicit:

- `overwrite: false`
- `include-hidden-files: false`
- `if-no-files-found: error`

Artifact names and upload paths are unchanged. Executable permissions remain
inside the AEGIS tar archives rather than relying on workflow-artifact ZIP
metadata.

### Download Artifact v8

Version 8 adds ESM packaging, direct-download handling, and fail-closed digest
verification. AEGIS keeps decompression enabled and makes the integrity
boundary explicit:

- `skip-decompress: false`
- `digest-mismatch: error`

The workflows do not use `github-token`, `repository`, or `run-id`; downloads
remain limited to artifacts from the current workflow run.

## Runner Audit

| Workflow job | Before | Selected | Architecture reason |
| --- | --- | --- | --- |
| Governance validation | `ubuntu-latest` | `ubuntu-24.04` | Stable x64 validation image |
| Gateway validation | `ubuntu-latest` | `ubuntu-24.04` | Stable x64 validation image |
| Desktop validation | `macos-latest` | `macos-15` | Stable arm64 image |
| Draft arm64 artifact | `macos-latest` | `macos-15` | Native arm64 build |
| Draft x64 artifact | `macos-latest` | `macos-15-intel` | Native x64 build |
| Combined draft checksums | `macos-latest` | `macos-15` | Explicit stable macOS utility image |
| Draft release arm64 asset | `macos-latest` | `macos-15` | Native arm64 build |
| Draft release x64 asset | `macos-latest` | `macos-15-intel` | Native x64 build |
| Draft GitHub Release | `macos-latest` | `macos-15` | Explicit stable macOS utility image |

GitHub documents that `macos-15` is arm64 and `macos-15-intel` is x64. GitHub
also documents that `-latest` migration is gradual and may expose workflows to
different images during the migration window. Explicit OS labels remove that
alias migration risk, although GitHub still updates each selected image
regularly.

## Permissions Review

- All workflows retain top-level `contents: read`.
- Only `draft-github-release` retains job-level `contents: write`.
- No PAT, repository secret, OIDC permission, package permission, deployment
  permission, issue permission, or pull-request write permission was added.
- No workflow uses `pull_request_target` or `workflow_run`.
- All checkout credentials are removed after checkout rather than persisted.

## Artifact and Publication Boundaries

The following semantics remain unchanged:

- manual-only draft artifact and draft release triggers
- per-platform artifact names
- combined artifact name
- archive paths and archive contents
- tar-based executable permission preservation
- combined `SHA256SUMS` generation and verification
- exact existing-tag and tag-to-`HEAD` guards
- draft prerelease behavior
- refusal to modify a published release
- absence of automatic publication

Static tests continue to enforce these boundaries and now also enforce approved
action majors, explicit runner labels, architecture mapping, credential
handling, hidden-file exclusion, immutable artifact uploads, and current-run
artifact downloads.

## Live GitHub Actions Evidence

Both maintenance workflows ran from commit
`2d4f430b741daceae4a13c5fa0ac7564c088073e` on the isolated branch.

| Workflow | Run | Result |
| --- | --- | --- |
| Validate | [29383232515](https://github.com/irgordon/aegis/actions/runs/29383232515) | PASS |
| Draft Release Artifacts | [29383232518](https://github.com/irgordon/aegis/actions/runs/29383232518) | PASS |

The Validate run passed governance and contract validation, Rust validation,
and desktop validation. It emitted no Node 20 deprecation annotation.

The non-publishing artifact run passed:

- native arm64 build on `macos-15`
- native x64 build on `macos-15-intel`
- combined checksum generation and verification on `macos-15`
- v7 artifact uploads
- v8 current-run artifact download and digest verification

The artifact names remain:

- `draft-macos-arm64-v0.4.2`
- `draft-macos-x64-v0.4.2`
- `draft-artifacts-v0.4.2`

The maintenance run is validation evidence for this branch only. Its artifacts
do not replace the frozen candidate from run `29374158498`.

No local or remote `v0.4.2` tag exists, and no `v0.4.2` GitHub Release exists.
The draft GitHub Release workflow was not dispatched.

## Changelog Policy

The canonical changelog policy records new work under `Unreleased` until the
next product release. This review therefore does not invent an internal
`0.4.2-maintenance.1` product-like version.

## Release-Candidate Isolation

This work is isolated on `mothra/ci-actions-maintenance`. It does not replace or
modify workflow run `29374158498`, which remains the frozen `v0.4.2` artifact
candidate evidence source.

The branch must remain unmerged until `v0.4.2` is tagged and published. If the
branch is merged first, the existing candidate becomes invalid and every
artifact gate must be repeated from the merged commit before release work can
continue.
