# AEGIS
# Security Model v1.0

## Purpose

This document defines the security model for AEGIS.

AEGIS exists to separate autonomous AI reasoning from real-world authority. The security model preserves that separation by requiring every external action to pass through a governed execution boundary before it can affect enterprise systems.

## Scope

This model applies to:

- AI orchestrator integration
- gateway request handling
- policy evaluation
- security wrapper enforcement
- credential injection
- human approval
- audit evidence
- replay and resume
- policy bundle activation

## Security Objectives

AEGIS shall provide:

- mandatory gateway routing for external actions
- deterministic policy decisions
- deny-by-default behavior
- least-privilege execution
- credential isolation from agents
- explicit human approval for high-risk actions
- immutable audit evidence
- deterministic replay from stored state

## Trust Posture

AEGIS follows zero-trust principles.

No caller, agent, orchestrator, policy bundle, approval event, wrapper result, or external system response is trusted solely because it exists. Each security-relevant input must be validated before it influences execution.

## Deny by Default

Execution is denied unless AEGIS can establish:

- request validity
- caller identity where required
- policy bundle authenticity
- policy compatibility
- authorization decision
- approval validity where required
- wrapper success
- credential availability and scope
- audit recordability

Uncertainty is not authorization.

## Credential Model

Agents and orchestrators shall never receive long-lived production credentials.

Credentials may be introduced only inside the approved execution path and only for the minimum scope and duration needed to complete the authorized action.

Credential handling must preserve these rules:

- secrets are not included in prompts
- secrets are not returned to orchestrators
- secrets are not written to logs
- secrets are not stored in audit records
- scoped credentials expire or are revoked after use
- missing credentials fail closed

## Authorization Model

The orchestrator may request an action. It may not authorize the action.

Authorization is determined by the policy engine using the active immutable policy bundle and execution context. The gateway coordinates the result. Wrappers enforce the decision.

Valid policy outcomes are:

- allow
- deny
- pending approval

Unknown tools, malformed requests, incompatible policy, and unsupported capability classes deny by default.

## Human Approval Model

Human approval is required for actions classified by policy as requiring human-in-the-loop review.

Approval must be bound to:

- execution identity
- action identity
- request parameters
- policy decision
- policy provenance
- approver identity
- approval timestamp
- approval scope

Approval is not reusable for different parameters or later unrelated actions.

## Replay Security

Replay executes stored intent. It must not ask an LLM, planner, or orchestrator to reinterpret the action.

Replay must use:

- preserved execution state
- original tool request
- approval evidence where applicable
- pinned policy bundle or documented migration state
- idempotency and duplicate-execution controls

If replay safety cannot be established, replay denies or halts closed.

## Audit Security

Every material decision must produce audit evidence.

Audit evidence must be structured, attributable, linked to execution identity, linked to policy provenance, and protected from agent or orchestrator modification.

Audit records must not contain secrets.

## Policy Bundle Security

Policy bundles must be immutable, versioned, and validated before activation.

Activation requires checking:

- manifest structure
- schema compatibility
- gateway compatibility
- wrapper compatibility
- policy checksums
- signatures where signing is enabled
- environment applicability

A gateway must not silently replace active policy because a registry changes.

## Failure Handling

Security-relevant failures deny execution or halt the unsafe operation.

Fail-closed conditions include:

- invalid request schema
- unknown tool
- missing policy
- invalid policy signature
- incompatible bundle
- invalid approval
- stale approval
- wrapper failure
- credential injection failure
- audit write failure where audit is mandatory
- replay ambiguity

## Relationship to Other Documents

- `OPERATING_DOCTRINE.md` defines engineering authority.
- `INVARIANTS.md` defines non-negotiable security properties.
- `ARCHITECTURE.md` defines component boundaries.
- `THREAT_MODEL.md` identifies threats against this model.
- `AUDIT_LOGGING.md` defines evidence requirements.
