# AEGIS
# Policy Distribution v1.0

## Purpose

This document defines how AEGIS policy moves from authored configuration to enforced runtime policy without weakening determinism, provenance, or security.

## Distribution Principle

Policy is centrally governed and locally enforced.

A gateway evaluates only an activated immutable local policy bundle. It must not treat live registry state as active policy.

## Policy Lifecycle

Policy distribution follows this sequence:

1. Author policy changes.
2. Review changes.
3. Validate policy and schemas.
4. Package an immutable bundle.
5. Attach manifest, checksums, and signatures where configured.
6. Deploy bundle to target environment.
7. Activate bundle explicitly.
8. Record activation evidence.
9. Enforce locally from the activated bundle.
10. Roll back through explicit activation of a prior valid bundle when needed.

## Policy Bundle Contents

A policy bundle should include:

- gateway policy
- risk matrix
- manifest
- checksums
- signatures where configured
- schema compatibility metadata
- gateway compatibility metadata
- wrapper compatibility metadata
- environment identifier

## Manifest Requirements

The manifest must identify:

- bundle ID
- policy version
- policy hash
- created timestamp
- signer identity where signing is used
- target environment
- compatible schema versions
- compatible gateway versions
- compatible wrapper versions

## Activation Rules

A policy bundle may activate only after validation succeeds.

Activation must verify:

- manifest is present
- checksums match bundle contents
- signatures are valid where required
- compatibility metadata is supported
- environment matches the target gateway
- required policy files are present
- unknown mandatory fields are rejected or explicitly handled

Invalid bundles must not activate.

## Checksum Verification

Checksum verification confirms that required local bundle files match the recorded bundle metadata.

A checksum is a file fingerprint. If `manifest.yaml`, `gateway_policy.yaml`, or `risk_matrix.yaml` changes unexpectedly, its SHA-256 checksum changes. The gateway must reject a local bundle when a required checksum is missing or does not match the file content.

Checksum verification protects local bundle integrity. It does not prove who authored or approved the bundle.

Checksum metadata for the local development bundle lives in:

```text
examples/policy-bundles/local-dev/checksums/SHA256SUMS
```

The local format is:

```text
<sha256>  manifest.yaml
<sha256>  gateway_policy.yaml
<sha256>  risk_matrix.yaml
```

## Signature Verification

Signature verification confirms that trusted bundle metadata has not changed since it was signed.

The local development bundle signs the checksum manifest, not request input, runtime output, or arbitrary policy evaluation results.

Signed artifact:

```text
examples/policy-bundles/local-dev/checksums/SHA256SUMS
```

Signature fixture:

```text
examples/policy-bundles/local-dev/signatures/SHA256SUMS.sig
```

Local development public key fixture:

```text
examples/policy-bundles/local-dev/signatures/public.pem
```

The Phase 2 local loader verifies:

- required bundle files are present
- manifest and risk matrix versions align
- required file checksums match `SHA256SUMS`
- `SHA256SUMS.sig` is a valid Ed25519 signature over `SHA256SUMS`

The loader fails closed when:

- the public key is missing or malformed
- the signature is missing or malformed
- the checksum manifest changes after signing
- a required policy bundle file no longer matches the signed checksum manifest

Regenerate the local development fixture after changing policy bundle files:

```bash
scripts/regenerate-local-policy-signature.sh
```

This helper regenerates `SHA256SUMS`, creates a fresh local development Ed25519 key pair, writes `public.pem`, signs `SHA256SUMS`, writes `SHA256SUMS.sig`, and discards the private key. It is for local development fixtures only.

Signature verification is not production PKI. It does not implement certificate validation, remote trust registry lookup, production signer authorization, or policy rule evaluation.

## Local Policy Evaluation Boundary

After a local bundle passes structure, version, checksum, and signature verification, the Phase 2 local gateway may evaluate its `gateway_policy.yaml` and `risk_matrix.yaml` files.

The verified local bundle is required before evaluation. If bundle verification fails, policy evaluation fails closed and returns denial evidence.

Local evaluation is deterministic and file-local. It does not use live registry state, remote downloads, external system calls, wrapper execution, credential injection, approval workflow execution, durable audit persistence, HTTP, or UI.

For contributors changing the local development bundle:

1. Update `examples/policy-bundles/local-dev/gateway_policy.yaml` or `examples/policy-bundles/local-dev/risk_matrix.yaml`.
2. Regenerate checksums and signature fixtures:

```bash
scripts/regenerate-local-policy-signature.sh
```

3. Run validation:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Expected fail-closed evaluation cases include:

- no verified bundle
- malformed policy file
- malformed risk matrix
- no matching policy rule
- ambiguous matching policy rules
- missing risk matrix entry
- unsupported capability class
- unsupported decision value

## Runtime Rules

A running gateway must not silently replace its active policy because a registry changes.

Existing runs remain pinned to their active bundle unless a documented migration protocol explicitly supersedes that binding.

Policy activation must be auditable.

## Rollback

Rollback is an explicit activation of a previously valid bundle.

Rollback must preserve:

- activation evidence
- previous bundle provenance
- new active bundle provenance
- compatibility checks
- run-level policy pinning

Rollback must not erase audit evidence for the superseded bundle.

## Failure Behavior

Policy distribution fails closed when:

- bundle contents are corrupted
- checksums do not match
- signatures are invalid where required
- compatibility is unknown or unsupported
- environment metadata conflicts
- required files are absent
- activation evidence cannot be recorded

## Validation

Policy distribution validation must cover:

- manifest parsing
- checksum verification
- signature verification where configured
- compatibility checks
- explicit activation
- rollback procedure
- denial of invalid bundles
- audit evidence for activation and rollback
