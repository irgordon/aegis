AEGIS

AI Execution Governance & Interception System

OPERATING_DOCTRINE.md

Project Operating Doctrine v1.0

⸻

Purpose

This document establishes the engineering doctrine for AEGIS.

It defines how the repository is governed, how architectural decisions are made, how contributors work, and how implementations are validated.

This doctrine exists to prevent architectural drift, inconsistent engineering practices, undocumented design decisions, and non-deterministic behavior.

Every contributor—human or AI—shall follow this doctrine before modifying the repository.

This document is the constitutional document of AEGIS.

⸻

Mission

AEGIS exists to provide deterministic, policy-driven governance for autonomous AI execution.

Rather than trusting AI systems with unrestricted access to enterprise resources, AEGIS establishes an independent execution boundary that evaluates every external action against organizational policy before execution.

The mission is to enable organizations to safely transition from traditional Authority to Operate (ATO) toward runtime Authority to Execute (ATE).

⸻

Vision

The long-term vision of AEGIS is to become the reference architecture for secure AI execution governance.

The system should be portable across:

* cloud providers
* operating systems
* orchestration frameworks
* programming languages
* enterprise environments

while preserving identical security guarantees.

⸻

Engineering Philosophy

The quality of an engineering organization is determined by the quality of its decisions rather than the quantity of its code.

AEGIS is built on the belief that:

* architecture is more valuable than implementation
* documentation is more valuable than tribal knowledge
* determinism is more valuable than convenience
* simplicity is more valuable than cleverness
* correctness is more valuable than speed

⸻

Repository Principles

The repository follows Documentation-Driven Engineering.

Documentation defines implementation.

Implementation never defines documentation.

Every architectural decision should exist before code is written.

Documentation is treated as executable governance.

⸻

Core Principles

1. Determinism First

Determinism is the primary design objective.

Given identical:

* inputs
* execution state
* policy bundle
* approvals
* environment

AEGIS shall produce identical outputs.

No optimization may compromise deterministic behavior.

⸻

2. Security Before Features

Security is never optional.

No feature shall weaken:

* authentication
* authorization
* auditing
* policy enforcement
* execution integrity

Feature development never supersedes security.

⸻

3. Fail Closed

When uncertainty exists, execution shall be denied.

Examples include:

* invalid signatures
* incompatible schema versions
* corrupted policy bundles
* missing credentials
* wrapper failures
* replay uncertainty
* timeout validating approvals

The absence of certainty shall never become implicit authorization.

⸻

4. Documentation Before Code

Documentation shall precede implementation.

Every architectural change must first update:

* PRD
* Architecture
* User Flows
* Invariants
* Acceptance Criteria

Code without documentation is incomplete.

⸻

5. Architecture Before Optimization

Correctness precedes performance.

Performance improvements are encouraged only after architectural correctness has been established.

⸻

6. Explicit Over Implicit

Hidden behavior increases operational risk.

AEGIS favors:

* explicit state
* explicit configuration
* explicit validation
* explicit permissions
* explicit failures
* explicit transitions

Magic behavior is discouraged.

⸻

7. Simplicity

Simple systems survive.

Complexity must always justify itself.

Every abstraction should eliminate complexity rather than create it.

⸻

8. Separation of Concerns

Responsibilities shall remain independent.

The orchestrator plans.

The gateway governs.

The policy engine decides.

The wrappers enforce.

The audit system records.

No component should assume another component’s responsibility.

⸻

Phase 3 Engineering Note

AEGIS is not intended to become the most feature-rich AI gateway.

It is intended to become the clearest, safest, and most predictable execution gateway.

Simplicity that preserves security is preferred over sophistication that increases operational burden.

⸻

Documentation Hierarchy

The repository follows strict documentation precedence.

1. OPERATING_DOCTRINE.md
2. PRD.md
3. ARCHITECTURE.md
4. INVARIANTS.md
5. USER_FLOWS.md
6. ACCEPTANCE_CRITERIA.md
7. CODING_STYLE.md
8. DOCUMENTATION.md
9. TASKS.md

Higher documents override lower documents.

⸻

Repository Workflow

Every implementation shall follow this sequence.

Read Operating Doctrine
↓
Read PRD
↓
Read Architecture
↓
Read Invariants
↓
Read User Flows
↓
Read Coding Style
↓
Read Acceptance Criteria
↓
Implement
↓
Validate
↓
Update Documentation
↓
Update Tasks
↓
Commit

Skipping documentation is prohibited.

⸻

Architectural Governance

Architecture changes require:

* documented rationale
* updated architecture diagrams
* invariant review
* acceptance criteria review
* implementation plan

Architecture evolves intentionally.

It never drifts accidentally.

⸻

Security Doctrine

Security decisions belong exclusively to AEGIS.

The orchestrator is never trusted to enforce security.

Security includes:

* authentication
* authorization
* credential management
* policy evaluation
* execution governance
* human approval
* audit logging

⸻

Zero Trust Doctrine

The repository adopts Zero Trust principles.

Assume:

* orchestrators may be compromised
* prompts may be malicious
* tools may behave unexpectedly
* external systems may fail

Trust is continuously verified.

Never assumed.

⸻

Policy Doctrine

Policy shall be:

* declarative
* immutable
* versioned
* signed
* centrally authored
* locally enforced

Runtime policy mutation is prohibited.

Policy evaluation shall be deterministic.

⸻

Policy Distribution

Policy follows GitOps.

Registry
↓
Review
↓
Validation
↓
Signing
↓
Bundle
↓
Deployment
↓
Activation

Gateways execute only locally loaded policy bundles.

Runtime polling may discover updates but shall never silently activate them.

⸻

Execution Doctrine

Every external action possesses:

* Execution ID
* Action ID
* Replay Token
* Attempt Number
* Policy Version

These collectively define execution identity.

⸻

Deterministic Replay

Replay shall never consult the LLM.

Replay shall execute:

the previously approved action

against

the identical policy bundle

using

the preserved execution state.

Replay is mechanical.

Planning is not.

⸻

State Management

Execution state is durable.

State survives:

* crashes
* deployments
* restarts
* approval delays
* infrastructure failures

Durability is mandatory.

⸻

Human Governance

Humans define governance.

AI executes within governance.

AI shall never modify governance.

⸻

Credential Doctrine

Agents shall never possess:

* production passwords
* API keys
* cloud credentials
* signing keys

Credentials are injected only during execution.

Credentials disappear immediately after execution.

⸻

Audit Doctrine

Every externally visible action shall produce immutable audit evidence.

Audit records shall include:

* execution identity
* policy provenance
* approval metadata
* timestamps
* outcome
* actor identity

Audit logs are evidence.

Not application logs.

⸻

Logging Doctrine

Operational logs and audit logs are distinct.

Operational logs support engineering.

Audit logs support governance.

Operational logs may rotate.

Audit logs shall be retained according to organizational policy.

⸻

Error Handling

Errors must be:

* explicit
* actionable
* deterministic
* logged

Silent failures are prohibited.

⸻

AI Development Doctrine

AI assists implementation.

AI does not define architecture.

AI shall:

* preserve invariants
* follow documentation
* explain assumptions
* avoid speculative implementation

Human judgment remains authoritative.

⸻

Documentation Doctrine

Every architectural decision shall be documented.

Undocumented decisions are considered temporary.

Repository documentation shall remain synchronized with implementation.

Documentation debt is engineering debt.

⸻

Coding Doctrine

Code should be readable from top to bottom.

Readers should understand:

what

before

how.

Functions should be:

* short
* focused
* composable
* independently testable

Deep nesting is discouraged.

Business logic belongs in services.

Validation belongs at boundaries.

⸻

Testing Doctrine

Every feature requires validation.

Minimum expectations include:

* unit tests
* integration tests
* contract tests
* deterministic replay tests
* negative-path tests
* policy validation tests

Every bug shall result in a regression test.

⸻

Code Review Doctrine

Code review evaluates:

* correctness
* architecture
* determinism
* security
* readability
* documentation
* testing

Passing CI alone does not constitute approval.

⸻

CI/CD Doctrine

CI exists to verify.

Never to compensate for poor engineering.

Required validation includes:

* formatting
* linting
* unit tests
* integration tests
* schema validation
* documentation validation
* policy validation
* security scanning

Failed validation blocks release.

⸻

Release Doctrine

Every release shall be:

* versioned
* reproducible
* documented
* auditable

Release artifacts shall include:

* changelog
* policy compatibility
* schema compatibility
* migration notes

⸻

Dependency Doctrine

Dependencies are liabilities.

Every dependency should justify:

* functionality
* maintenance
* security
* licensing

Prefer fewer dependencies.

⸻

Backward Compatibility

Breaking changes require:

* documented justification
* migration strategy
* semantic version increment
* updated documentation

Compatibility should be preserved whenever practical.

⸻

Technical Debt

Technical debt shall be:

identified

documented

prioritized

reduced

Ignored technical debt eventually becomes architectural debt.

⸻

Decision Priority

When tradeoffs exist, prioritize in this order:

1. Security
2. Determinism
3. Correctness
4. Architectural Integrity
5. Policy Integrity
6. Auditability
7. Maintainability
8. Simplicity
9. Performance
10. Developer Convenience

This ordering governs engineering decisions.

⸻

Definition of Done

Implementation is complete only when:

✓ Requirements are satisfied

✓ Architecture remains valid

✓ Invariants remain true

✓ Documentation is updated

✓ Tests pass

✓ Acceptance criteria pass

✓ Tasks are updated

✓ Security implications are reviewed

✓ Audit implications are reviewed

✓ CI passes

If any item is incomplete, implementation is incomplete.

⸻

Long-Term Repository Goals

AEGIS should become:

* a reference implementation
* a reference architecture
* language agnostic
* cloud agnostic
* orchestration agnostic
* deterministic by design
* secure by default
* auditable by construction

Every contribution should move the repository closer to these goals.

⸻

Final Doctrine

The objective of AEGIS is not merely to execute AI safely.

Its objective is to establish a governance architecture that organizations can trust.

Trust is not created through claims.

Trust is created through deterministic behavior, transparent governance, verifiable evidence, and disciplined engineering.

Every line of documentation should strengthen understanding.

Every line of code should strengthen the architecture.

Every architectural decision should strengthen the trustworthiness of the system.

That is the standard by which AEGIS shall be developed, reviewed, and maintained.
