# AEGIS
## Project Operating Doctrine v1.0

## Purpose
This doctrine defines how AEGIS is designed, implemented, validated, and evolved. It is the highest-level governance document in the repository.

## Mission
Build a deterministic, policy-driven execution governance platform that enables organizations to safely adopt AI through Authority to Execute (ATE).

## Core Principles
- Documentation before implementation.
- Determinism before optimization.
- Security before features.
- Fail closed.
- Zero-trust credentials.
- Declarative policy.
- Human governance of high-risk actions.
- Immutable policy bundles.
- Durable execution state.
- Comprehensive auditing.

## Documentation Precedence
1. OPERATING_DOCTRINE.md
2. PRD.md
3. ARCHITECTURE.md
4. INVARIANTS.md
5. USER_FLOWS.md
6. ACCEPTANCE_CRITERIA.md
7. CODING_STYLE.md
8. TASKS.md

## Engineering Rules
Every change shall preserve architectural invariants, include documentation updates where required, include automated tests, and maintain deterministic behavior.

## Definition of Done
Implementation is complete only when documentation, tests, acceptance criteria, and task tracking have all been updated and verified.