# AEGIS

AI Execution Governance & Interception System

## Status

AEGIS is pre-alpha.

Do not install, deploy, or rely on this repository to protect real systems yet.

Phase 2 is complete. The repository now contains a working local Rust Gateway MVP that can validate a fixture request, verify a local policy bundle, evaluate simple local policy, return a bounded response, emit audit evidence, and optionally append a local JSONL audit record.

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
- local policy and risk matrix evaluation for verified development bundles
- local append-only JSONL audit logging for development
- structured JSON error reports for local runtime failures
- contract tests for request, response, audit, policy bundle, and gateway boundaries

The local gateway can read a fixture request, verify a local policy bundle fixture, evaluate simple local policy and risk rules, produce a bounded response, optionally append a local JSONL audit record, and emit structured audit evidence. This is development evidence, not production enforcement.

## Roadmap Summary

The next milestone is Phase 3: Governed Execution Engine.

Phase 3 focuses on safely executing real AI actions under governance. The priority order is wrapper execution, credential injection, execution lifecycle, approval workflow, durable execution state, replay and recovery, mutation execution, and integration testing.

HTTP, service deployment, plugin architecture, and UI belong to Phase 4. Production PKI, remote policy distribution, high availability, performance, security review, fuzzing, compatibility guarantees, release engineering, and operational documentation belong to Phase 5.

## What Does Not Exist Yet

AEGIS does not yet provide:

- production-grade policy evaluation
- production-grade risk matrix evaluation
- wrapper execution
- credential injection
- approval workflow
- production durable audit persistence
- replay execution
- HTTP service
- UI
- production public key infrastructure
- remote policy registry trust

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
