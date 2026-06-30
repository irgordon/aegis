# AEGIS
# Architectural Principles v1.0

## Purpose

This document defines the architectural principles used to evaluate design decisions in AEGIS.

Principles are different from invariants.

Invariants are hard rules that must remain true.

Principles are the design lens used when tradeoffs arise.

When two valid implementation options exist, choose the option that better satisfies these principles.

## Scope

These principles apply to:

- gateway design
- policy design
- wrapper design
- audit design
- state management
- orchestrator integration
- deployment models
- future language ports

## Principle 1: Governance Over Autonomy

AEGIS exists because autonomous execution requires governance.

AI agents may propose actions, but they do not authorize themselves.

The system should always make the governance boundary explicit.

A design that gives the agent more freedom than policy permits is invalid.

## Principle 2: Deterministic by Design

AEGIS must produce repeatable behavior when given the same inputs, state, approvals, and policy bundle.

Determinism is not an optimization. It is the foundation of trust.

Any feature that introduces non-determinism must capture that non-determinism explicitly in state or audit evidence.

## Principle 3: Policy is Data

Policy should be declarative, reviewable, versioned, and testable.

Policy should not be hidden in prompts, code branches, wrapper scripts, or orchestration logic.

When possible, policy should be handled like infrastructure as code.

## Principle 4: Separate Decision from Enforcement

The policy engine decides whether an action may proceed.

Security wrappers enforce the decision.

The gateway coordinates the flow.

Combining these responsibilities makes the system harder to test and easier to bypass.

## Principle 5: Local Enforcement, Central Governance

Policies may be centrally authored, reviewed, and signed.

Gateways enforce locally against immutable bundles.

This avoids dependence on a fragile live control plane while preserving enterprise governance.

## Principle 6: Evidence Over Assertion

AEGIS should prove what happened.

A claim that an action was safe is not enough.

The system should produce durable evidence showing:

- what was requested
- what policy applied
- what decision was made
- who approved it if approval was required
- what was executed
- what result occurred

## Principle 7: Fail Closed

Uncertainty is not permission.

When the system cannot verify authorization, integrity, compatibility, identity, approval, or state, execution should stop.

This principle applies to startup, runtime execution, replay, policy activation, and wrapper failure.

## Principle 8: Explicit Identity

Every meaningful object should have a clear identity.

This includes:

- run identity
- task identity
- action identity
- execution identity
- approval identity
- policy bundle identity

Explicit identity prevents ambiguity during replay, audit, approval, and incident response.

## Principle 9: Durable State

Important state must survive process memory.

Approval state, execution state, replay state, and terminal outcomes must be persisted before the system depends on them.

Memory-only governance is not governance.

## Principle 10: Least Privilege

AEGIS should grant the smallest authority required for the shortest useful time.

Agents should never hold standing production authority.

Credentials should be scoped, temporary, and injected only during approved execution.

## Principle 11: Deny by Default

Unknown actions, tools, policies, schemas, approvals, and states should be denied unless explicitly allowed.

AEGIS should not infer permission from intent, naming, prompt content, or historical behavior.

## Principle 12: Simplicity Over Cleverness

AEGIS is security-sensitive infrastructure.

Security-sensitive infrastructure should be simple to inspect, test, and reason about.

Prefer clear control flow, explicit types, and boring code.

Avoid clever abstractions in authorization, approval, credential, replay, or audit paths.

## Principle 13: Reduce Cognitive Load

Every feature must reduce operational risk more than it increases cognitive load.

Security that operators cannot understand is operationally weak.

Documentation, implementation, and workflows should remain intentionally simple.

## Principle 14: Every Abstraction Must Earn Its Existence

Traits, modules, services, documents, and architectural layers must exist because they solve today's problem or directly enable the next planned milestone.

Do not add speculative architecture.

## Principle 15: Useful Execution

The purpose of AEGIS is to safely govern AI execution.

Every implementation task should satisfy at least one of:

- Execute
- Govern
- Recover
- Prove

If it satisfies none of these, it likely belongs in a later phase or should not exist.

## Principle 16: Release Before Expansion

A usable release creates more value than additional unfinished capability.

Until the current release objective is achieved, new work should strengthen or complete release-critical functionality rather than expanding project scope.

Work that does not move AEGIS measurably closer to `v0.4.0` should be deferred.

## Principle 17: Orchestrator Agnostic

AEGIS should not depend on one AI framework.

The architecture should work with different orchestrators as long as they can submit structured tool requests and obey the finite state machine contract.

Framework-specific integrations should be adapters, not architectural dependencies.

## Principle 18: Cloud Agnostic

AEGIS should not require one cloud provider.

It should be deployable in cloud, hybrid, and on-premises environments.

Provider-specific implementations should live behind clear interfaces.

## Principle 19: Human Authority for High-Risk Actions

High-risk or irreversible actions require human governance.

Human approval should be explicit, attributable, auditable, and bound to a specific action.

Approval should not become a generic permission token.

## Principle 20: Replay is Mechanical

Replay should execute stored intent.

Replay should not re-enter the planning layer.

An approved action should be replayed exactly as approved, against the policy and state that governed the approval.

## Principle 21: Compatibility is a Safety Boundary

Policy bundles, schemas, wrappers, and runtime versions must be compatible before activation.

The system should reject unknown or unsupported combinations rather than attempting best-effort execution.

## Principle 22: Documentation Governs Implementation

Documentation should define what the system must do before code is written.

Code that contradicts documentation is suspect.

Documentation should be updated deliberately when the architecture changes.

## Principle 23: Test the Negative Path

The most important behavior in AEGIS is often what does not happen.

Tests must prove that unauthorized, malformed, stale, duplicate, or unsafe actions do not execute.

Successful execution tests are not enough.

## Principle 24: Build for Investigation

AEGIS should assume that future reviewers will need to reconstruct events.

Audit, state, and policy provenance should make investigation possible without relying on memory, chat history, or tribal knowledge.

## Design Decision Checklist

When evaluating a design, ask:

- Does it preserve determinism?
- Does it fail closed?
- Does it keep policy declarative?
- Does it separate decision from enforcement?
- Does it avoid giving agents standing credentials?
- Does it produce audit evidence?
- Does it preserve durable state?
- Does it work across orchestrators?
- Does it avoid unnecessary complexity?
- Does the abstraction solve today's problem or the next planned milestone?
- Does the task execute, govern, recover, or prove?
- Does the work move AEGIS measurably closer to the current release objective?
- Can it be tested through negative paths?

## Final Rule

AEGIS should make safe AI execution easier than unsafe AI execution.

Any design that makes bypass, ambiguity, hidden state, or unverifiable execution easier is moving the project in the wrong direction.
