# AEGIS

AI Execution Governance & Interception System

## Status

AEGIS is pre-alpha.

Do not install, deploy, or rely on this repository to protect real systems yet. The project is still proving its governance model, protocol contracts, and early local Rust gateway behavior.

## What AEGIS Is

AEGIS is a governed checkpoint for AI-driven actions.

In plain terms, AEGIS is meant to answer this question before an AI system touches email, databases, cloud systems, source control, tickets, or other business tools:

```text
Should this AI action be allowed to execute?
```

The intended production system will validate the request, evaluate policy, require approval when needed, execute only through controlled wrappers, and record audit evidence.

## What Currently Exists

This repository currently contains:

- governance and architecture documents
- JSON schemas and examples
- repository validation tooling
- an early local Rust Gateway MVP
- local policy bundle structure checks
- SHA-256 checksum verification for local policy bundle files
- Ed25519 signature verification for the local checksum manifest
- contract tests for request, response, audit, policy bundle, and gateway boundaries

The local gateway can read a fixture request, verify a local policy bundle fixture, produce a bounded response, and emit structured audit evidence. This is development evidence, not production enforcement.

## What Does Not Exist Yet

AEGIS does not yet provide:

- production policy evaluation
- risk matrix evaluation
- wrapper execution
- credential injection
- approval workflow
- durable audit persistence
- replay execution
- HTTP service
- UI
- production public key infrastructure
- remote policy registry trust

## Policy Bundle Integrity

The current local policy bundle check proves that the gateway can reject a bundle when required local metadata is missing or changed.

A checksum is a fingerprint for a file. If a required policy bundle file changes, its checksum changes.

A signature proves that trusted bundle metadata has not changed since it was signed. In the local fixture, AEGIS verifies an Ed25519 signature over:

```text
examples/policy-bundles/local-dev/checksums/SHA256SUMS
```

That signed checksum manifest then controls the expected fingerprints for:

- `manifest.yaml`
- `gateway_policy.yaml`
- `risk_matrix.yaml`

This is not production PKI, certificate validation, or remote trust distribution.

## Where To Read Next

New readers should start with:

- [Documentation Standard](docs/DOCUMENTATION.md)
- [Product Requirements](docs/PRD.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Invariants](docs/INVARIANTS.md)

Contributors should read:

- [Operating Doctrine](docs/OPERATING_DOCTRINE.md)
- [Coding Style](docs/CODING_STYLE.md)
- [Tasks](docs/TASKS.md)
- [Test Strategy](docs/TEST_STRATEGY.md)

Engineers and architects should read:

- [Policy Distribution](docs/POLICY_DISTRIBUTION.md)
- [Policy Engine](docs/POLICY_ENGINE.md)
- [Trust Boundaries](docs/TRUST_BOUNDARIES.md)
- [Audit Logging](docs/AUDIT_LOGGING.md)
- [API Specification](docs/API_SPEC.md)

## Developer Validation

Run the current validation suite before committing changes:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
