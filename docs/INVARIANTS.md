# AEGIS
## Architectural Invariants v1.0

## Purpose

This document defines the non-negotiable properties of AEGIS.

An invariant is a rule that must remain true across every implementation, refactor, deployment model, and future language port. Features may change. Internal structure may change. These properties shall not change unless the architecture itself is formally revised.

If code violates this document, the code is wrong.

If tests pass while an invariant is violated, the tests are incomplete.

If documentation conflicts with this document, the higher-precedence governance documents must be reviewed before implementation continues.

## Scope

These invariants apply to:

- gateway runtime behavior
- policy evaluation
- wrapper enforcement
- orchestrator integration
- audit generation
- execution replay
- approval handling
- credential handling
- policy distribution
- deployment and rollback

## Invariant 1: Zero-Trust Credentials

Agents shall never possess long-lived credentials.

AI agents and orchestrators must not directly hold production API keys, cloud credentials, database passwords, signing keys, service account tokens, or equivalent secrets.

Credentials may be injected only at the point of authorized execution and only for the minimum scope and duration required.

What this protects:

- enterprise credentials
- downstream systems
- audit integrity
- credential rotation boundaries

What this prevents:

- prompt-injection credential theft
- agent misuse of standing permissions
- uncontrolled credential reuse
- privilege expansion outside the gateway

## Invariant 2: Mandatory Gateway Routing

All external actions shall traverse the gateway.

An external action is any action that touches a system outside the agent's local reasoning context, including APIs, databases, file stores, ticketing systems, email systems, cloud providers, source control systems, deployment systems, or enterprise applications.

No orchestrator may bypass the gateway for direct execution.

What this protects:

- policy enforcement
- audit coverage
- centralized governance
- execution accountability

What this prevents:

- shadow tool execution
- unaudited automation
- inconsistent enforcement
- direct agent-to-system calls

## Invariant 3: Declarative Policy

Policy shall be declarative and external to orchestration logic.

The orchestrator may request an action. It may not decide whether the action is authorized.

Policy decisions must be evaluated by the AEGIS policy layer using versioned policy artifacts.

What this protects:

- separation of concerns
- repeatable governance
- policy review workflows
- enterprise change control

What this prevents:

- security rules buried in prompts
- inconsistent tool-specific logic
- unreviewed policy changes
- model-generated authorization logic

## Invariant 4: Decision and Enforcement Separation

The policy engine decides.

The wrappers enforce.

The gateway coordinates.

No wrapper should become the source of policy truth. No orchestrator should become an enforcement mechanism. No policy file should execute side effects.

What this protects:

- maintainability
- testability
- policy portability
- clear failure domains

What this prevents:

- duplicated policy logic
- enforcement drift
- wrapper-specific governance
- hidden authorization paths

## Invariant 5: Side-Effect-Free Policy Evaluation

Policy evaluation shall not mutate system state.

Given the same request, context, and policy bundle, policy evaluation must return the same decision without changing external systems, execution state, audit stores, approval queues, or policy state.

State changes occur after policy evaluation, not during it.

What this protects:

- deterministic replay
- testability
- safe dry runs
- policy simulation

What this prevents:

- authorization changing state
- replay divergence
- policy-driven side effects
- hidden mutation during decision-making

## Invariant 6: Immutable Policy Bundles

Gateways shall evaluate against immutable, locally loaded policy bundles.

A running gateway must not silently replace its active policy because a registry changed. Runtime polling may discover available updates, but it must not alter active enforcement without an approved deployment or activation event.

What this protects:

- deterministic execution
- incident rollback
- policy provenance
- deployment integrity

What this prevents:

- mid-run governance drift
- live control-plane dependency
- registry outage authorization failure
- unreviewed runtime policy mutation

## Invariant 7: Policy Provenance

Every gateway decision shall record policy provenance.

At minimum, each decision and audit record must identify:

- policy version
- policy hash
- policy bundle ID
- signer identity
- deployment timestamp
- environment

What this protects:

- forensic reconstruction
- compliance review
- rollback confidence
- decision traceability

What this prevents:

- unknown policy state
- unverifiable decisions
- ambiguous audit records
- governance disputes after incidents

## Invariant 8: Execution Policy Pinning

Every agent run shall be bound to the policy bundle active at run creation unless a documented migration protocol explicitly supersedes it.

A policy deployment must not silently change the governance rules for an existing execution.

What this protects:

- long-running workflow integrity
- deterministic execution
- approval validity
- replay stability

What this prevents:

- Step 1 under one policy and Step 2 under another
- mid-run authorization drift
- inconsistent approval semantics
- non-reproducible executions

## Invariant 9: Cryptographic Approval Binding

Human approval shall be bound to a specific action and state.

Approval must not be a generic permission to continue. It must be tied to the action identity, request parameters, policy decision, execution state, approver identity, timestamp, and approval scope.

What this protects:

- human intent
- high-risk execution boundaries
- approval integrity
- forensic defensibility

What this prevents:

- stale approval reuse
- approval substitution
- approval replay
- broad unscoped authorization

## Invariant 10: Exactly-Once Execution

Approved actions shall execute exactly once unless explicitly designed as idempotent retries.

The system must prevent duplicate execution of the same approved action through idempotency keys, execution locks, action IDs, replay tokens, or equivalent mechanisms.

What this protects:

- production systems
- financial actions
- deployment safety
- data integrity

What this prevents:

- duplicate emails
- repeated database writes
- repeated destructive operations
- replay-triggered side effects

## Invariant 11: Deterministic Replay

Replay shall be deterministic and mechanical.

A replayed action must use preserved state and the pinned policy bundle. It must not ask the LLM to re-plan, reinterpret, revise, or regenerate the action.

What this protects:

- execution integrity
- approval integrity
- incident reconstruction
- workflow continuity

What this prevents:

- hallucination loops
- changed parameters after approval
- prompt-injection re-entry
- non-repeatable execution paths

## Invariant 12: Durable State

All material state transitions shall be durable.

State transitions involving running, pending approval, approved, denied, executed, failed, replayed, or canceled states must be persisted before the system relies on them.

What this protects:

- crash recovery
- long-running workflows
- audit continuity
- approval workflows

What this prevents:

- lost pending approvals
- duplicate execution after restart
- unverifiable state changes
- memory-only governance

## Invariant 13: Comprehensive Auditing

Every decision shall emit audit evidence.

Audit evidence must be generated for allowed, denied, pending, failed, replayed, canceled, and error outcomes.

Audit records must be structured, timestamped, attributable, and linked to execution identity and policy provenance.

What this protects:

- accountability
- SOC investigation
- compliance
- post-incident review

What this prevents:

- invisible denials
- invisible approvals
- missing forensic evidence
- untraceable automation

## Invariant 14: Audit Immutability

Agents and orchestrators shall not modify or delete audit records.

Audit storage must be protected from the systems being governed. Where possible, audit data should be written to append-only or write-once-read-many storage.

What this protects:

- evidence integrity
- compliance records
- incident response
- chain of custody

What this prevents:

- audit tampering
- post-action cleanup by compromised agents
- selective deletion of failures
- unverifiable activity history

## Invariant 15: Fail Closed

If AEGIS cannot establish authorization, authenticity, integrity, or compatibility, it shall deny execution.

This applies to:

- invalid policy signatures
- corrupted bundles
- missing policy metadata
- incompatible schema versions
- failed approval verification
- invalid execution identity
- wrapper failure
- credential injection failure

What this protects:

- default security posture
- high-risk systems
- governance certainty
- enterprise trust

What this prevents:

- uncertainty becoming permission
- partial authorization
- unsafe fallback modes
- silent bypass behavior

## Invariant 16: Explicit Execution Identity

Every external action shall have a unique execution identity.

Execution identity must distinguish between:

- run ID
- task ID
- action ID
- execution ID
- attempt number
- replay token

What this protects:

- deduplication
- audit traceability
- replay control
- approval binding

What this prevents:

- ambiguous actions
- duplicate approvals
- action substitution
- cross-run confusion

## Invariant 17: Capability Classification

Every tool shall be classified before execution.

At minimum, the architecture recognizes:

- L0: read-only
- L1: low-risk mutation
- L2: high-risk mutation
- L3: irreversible or highly sensitive action

Unclassified tools shall be denied by default.

What this protects:

- policy clarity
- human approval boundaries
- tool governance
- risk-based execution

What this prevents:

- unknown tool behavior
- accidental high-risk autonomy
- inconsistent approval requirements
- uncontrolled tool expansion

## Invariant 18: Human-in-the-Loop Non-Blocking Execution

Orchestrators shall not block active execution threads while waiting for human approval.

When an action enters pending approval, the orchestrator must persist state, stop active execution, and resume only after a valid approval event.

What this protects:

- resource stability
- deterministic re-entry
- long-running approval workflows
- operational scalability

What this prevents:

- sleep-loop orchestration
- memory-only pending state
- infinite approval waits
- prompt-driven re-planning during approval

## Invariant 19: Orchestrator Re-Entry Safety

After approval, an orchestrator shall replay the approved action exactly as stored.

It shall not call the planning layer before executing the approved action.

What this protects:

- approved parameter integrity
- human intent
- deterministic replay
- execution correctness

What this prevents:

- LLM reinterpretation after approval
- changed tool parameters
- approval bypass through re-planning
- hallucinated follow-up execution

## Invariant 20: Wrapper Non-Bypassability

Security wrappers shall not be optional from the orchestrator's perspective.

The execution path must ensure that wrapper enforcement occurs for every applicable external action.

What this protects:

- enforcement consistency
- credential safety
- policy execution
- audit integrity

What this prevents:

- direct tool calls
- alternate unwrapped paths
- selective enforcement
- security-by-convention

## Invariant 21: Compatibility Validation

Policy bundles, schemas, wrappers, and gateway runtime versions shall be compatibility checked before activation.

A gateway must reject unsupported combinations.

What this protects:

- predictable deployments
- safe upgrades
- rollback reliability
- runtime correctness

What this prevents:

- policy/runtime mismatch
- wrapper incompatibility
- schema confusion
- runtime-only discovery of broken governance

## Invariant 22: No Silent Privilege Expansion

No change may increase agent authority without explicit policy, documentation, and review.

This includes new tools, expanded scopes, lower approval thresholds, broader credentials, and changed capability classes.

What this protects:

- least privilege
- change control
- organizational risk boundaries
- governance review

What this prevents:

- accidental privilege creep
- hidden tool expansion
- reduced approval requirements without review
- overbroad execution rights

## Invariant 23: Idempotency at Execution Boundaries

Execution boundaries shall handle retries safely.

Where external systems cannot guarantee idempotency, AEGIS must enforce action-level deduplication before execution.

What this protects:

- downstream system integrity
- retry safety
- exactly-once behavior
- operational resilience

What this prevents:

- duplicate side effects
- retry storms
- inconsistent external state
- repeated irreversible actions

## Invariant 24: Deny-by-Default Tooling

Unknown, unregistered, or malformed tool calls shall be denied.

AEGIS shall not infer permission from naming, prompt context, or model intent.

What this protects:

- tool registry integrity
- policy enforcement
- secure defaults
- predictable behavior

What this prevents:

- accidental tool enablement
- prompt-based authorization
- malformed request execution
- unsafe default allow behavior

## Invariant 25: Evidence Over Assertion

AEGIS shall prefer verifiable evidence over claims.

Security, policy, approval, execution, and audit behavior must be demonstrable through tests, logs, schemas, and durable records.

What this protects:

- enterprise trust
- compliance readiness
- engineering rigor
- incident response

What this prevents:

- unverifiable security claims
- documentation-only guarantees
- incomplete validation
- governance theater

## Invariant 26: Release Truth

Every user-facing statement about project capabilities, installation, downloads,
release status, or supported workflows shall identify whether it describes the
latest published release or the current development branch.

Documentation shall not silently combine published-release behavior with
unreleased development behavior.

Release-sensitive facts shall be derived from the governed release-truth record.
Historical release evidence remains immutable. Development changes require a new
release rather than mutation of an existing public tag or release.

What this protects:

- user trust
- release reproducibility
- documentation accuracy
- historical release evidence

What this prevents:

- unreleased capabilities being attributed to published artifacts
- stale or contradictory release labels
- ambiguous version ownership
- mutation of historical release truth

## Required Validation

Future implementations should include automated checks for these invariants where practical.

Examples:

- schema tests for execution identity
- policy tests for deny-by-default behavior
- replay tests proving no LLM re-planning
- wrapper tests proving enforced routing
- audit tests proving provenance capture
- policy bundle tests proving signature and compatibility validation
- negative-path tests proving fail-closed behavior
- release-truth tests proving documentation and version alignment

## Change Control

Changing an invariant requires a formal architecture decision.

The change must update:

- OPERATING_DOCTRINE.md if doctrine changes
- ARCHITECTURE.md if structure changes
- INVARIANTS.md if the invariant changes
- ACCEPTANCE_CRITERIA.md if validation changes
- TEST_STRATEGY.md if test coverage changes

No implementation may weaken an invariant before the documentation is updated and reviewed.

## Final Rule

AEGIS exists to make AI execution governable.

Governance depends on stable properties.

These invariants are those properties.

They are not suggestions.

They are the architectural contract.
