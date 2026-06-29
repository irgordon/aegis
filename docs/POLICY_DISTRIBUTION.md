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

Signature verification is a separate control. Until cryptographic signature verification is implemented and tested, runtimes must say that signature cryptographic verification is not implemented rather than imply that signatures were fully verified.

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
