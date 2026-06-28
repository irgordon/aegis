<div align="center">

# AEGIS

**AI Execution Governance & Interception System**

[![Rust](https://img.shields.io/badge/RUST-%23E05D44.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bash](https://img.shields.io/badge/BASH-%234EAA25.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)](https://www.gnu.org/software/bash/)
[![TypeScript](https://img.shields.io/badge/TYPESCRIPT-%233178C6.svg?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Python](https://img.shields.io/badge/PYTHON-%233776AB.svg?style=for-the-badge&logo=python&logoColor=white)](https://www.python.org/)
[![GitHub Actions](https://img.shields.io/badge/CI-GITHUB%20ACTIONS-%232088FF.svg?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/features/actions)
[![License](https://img.shields.io/badge/LICENSE-MIT-brightgreen.svg?style=for-the-badge)](LICENSE)

</div>

AEGIS is a safety and governance platform for artificial intelligence.

Imagine hiring a new employee. You would not hand them the keys to every building, every bank account, and every computer system on their first day. You would verify who they are, define what they are allowed to do, and require approval for important decisions.

AEGIS applies the same idea to AI.

Before an AI can perform a real-world action, such as sending an email, modifying a database, deploying software, or interacting with business systems, AEGIS acts as an independent checkpoint. It verifies that the action follows organizational policy, requests human approval when appropriate, and records what happened for future review.

This helps organizations adopt AI with confidence instead of blind trust.

## What AEGIS provides

- A single security checkpoint for AI actions
- Policy-based decision making
- Human approval for sensitive operations
- Complete audit history
- Deterministic, repeatable execution
- Zero-trust security principles
- Immutable policy governance
- Durable execution evidence

## Who it is for

AEGIS is for organizations building or deploying AI assistants, autonomous agents, and enterprise automation in commercial, government, healthcare, financial, and other regulated environments.

It is written for both technical and nontechnical readers. The core idea is simple: AI should not be able to take important real-world actions without a governed checkpoint.

## Mission

Enable organizations to safely transition from Authority to Operate (ATO) toward Authority to Execute (ATE) through deterministic governance, policy enforcement, and auditable execution.

## Repository status

AEGIS has completed its governance and protocol-contract foundation. Phase 2 is building the Rust Gateway MVP.

## Local Gateway MVP

The local Gateway MVP reads one `ToolCallRequest` JSON document, verifies a local policy bundle structure, validates the request, routes supported requests through a deterministic local policy adapter seam, maps the result to a bounded `ToolCallResponse`, builds audit evidence, and prints structured JSON containing:

- `response`
- `audit_record`
- `policy_bundle`

The bundle loader verifies required local files and metadata:

- `manifest.yaml`
- `gateway_policy.yaml`
- `risk_matrix.yaml`
- `signatures/`
- `checksums/`
- manifest policy identity
- risk matrix version binding
- checksum metadata presence
- signature metadata presence

Cryptographic signature verification is not implemented yet, and the runtime says so in structured output. The local runtime is for Phase 2 development only. It does not provide production policy enforcement, risk matrix evaluation, wrapper execution, credential injection, durable audit storage, approval workflow, replay execution, HTTP service, or UI.

Run with stdin:

```bash
cargo run --bin aegis-gateway -- --bundle examples/policy-bundles/local-dev < schemas/examples/valid/ToolCallRequest.json
```

Run with an explicit file path:

```bash
cargo run --bin aegis-gateway -- --bundle examples/policy-bundles/local-dev schemas/examples/valid/ToolCallRequest.json
```

Run validation:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Core documents

- [Operating Doctrine](docs/OPERATING_DOCTRINE.md)
- [Product Requirements](docs/PRD.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Invariants](docs/INVARIANTS.md)
- [Coding Style](docs/CODING_STYLE.md)
- [Acceptance Criteria](docs/ACCEPTANCE_CRITERIA.md)
- [Security Model](docs/SECURITY_MODEL.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Policy Distribution](docs/POLICY_DISTRIBUTION.md)
- [API Specification](docs/API_SPEC.md)
- [Test Strategy](docs/TEST_STRATEGY.md)
- [Tasks](docs/TASKS.md)
- [Roadmap](docs/ROADMAP.md)
- [Phasemap](docs/PHASEMAP.md)

## Core principle

AEGIS does not trust an AI agent just because it can ask to do something.

AEGIS verifies the request, checks policy, applies security controls, records evidence, and only then allows execution when permitted.
