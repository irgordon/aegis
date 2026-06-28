<div align="center">

# AEGIS

**AI Execution Governance & Interception System**

[![Status](https://img.shields.io/badge/status-initial%20architecture-blue)](https://github.com/irgordon/aegis)
[![Governance](https://img.shields.io/badge/governance-documentation--driven-6f42c1)](docs/OPERATING_DOCTRINE.md)
[![Security](https://img.shields.io/badge/security-fail--closed-critical)](docs/INVARIANTS.md)
[![Policy](https://img.shields.io/badge/policy-immutable%20bundles-2ea44f)](docs/POLICY_DISTRIBUTION.md)
[![License](https://img.shields.io/badge/license-TBD-lightgrey)](LICENSE)

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

AEGIS is currently in the initial architecture and governance phase. The repository is being built documentation-first so the implementation follows a clear security and execution model before code is added.

## Core documents

- [Operating Doctrine](docs/OPERATING_DOCTRINE.md)
- [Product Requirements](docs/PRD.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Invariants](docs/INVARIANTS.md)
- [Coding Style](docs/CODING_STYLE.md)
- [Acceptance Criteria](docs/ACCEPTANCE_CRITERIA.md)
- [Roadmap](docs/ROADMAP.md)
- [Phasemap](docs/PHASEMAP.md)

## Core principle

AEGIS does not trust an AI agent just because it can ask to do something.

AEGIS verifies the request, checks policy, applies security controls, records evidence, and only then allows execution when permitted.