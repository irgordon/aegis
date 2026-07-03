# AEGIS
# Policy Engine v1.0

## Purpose

This document defines the governance contract for the AEGIS policy engine.

The policy engine answers one question:

Should this requested action execute?

## Scope

The policy engine covers authorization decision logic only. It does not execute tools, mutate execution state, inject credentials, request human approval directly, or write audit records as a side effect of evaluation.

## Inputs

Policy evaluation receives:

- validated tool request
- execution context
- active immutable policy bundle
- actor identity where available
- environment
- policy compatibility metadata
- relevant durable state references

Inputs must already be schema-valid before evaluation begins.

## Outputs

Policy evaluation returns one of these decisions:

- allow
- deny
- pending approval

The decision must include enough structured detail for the gateway to record audit evidence and route the next step.

Decision details should include:

- decision value
- reason code
- capability class
- policy rule identifier where available
- policy bundle ID
- policy version
- policy hash
- signer identity where available
- required approval scope when pending

## Determinism

Given the same request, context, state references, and policy bundle, the policy engine must return the same decision.

The policy engine must not depend on uncontrolled time, randomness, network calls, mutable global state, live registry state, or LLM interpretation.

If time or other non-deterministic values are needed, the gateway must provide captured values as explicit inputs.

## Side-Effect-Free Evaluation

Policy evaluation must not:

- write durable state
- emit audit records directly
- enqueue approvals directly
- call external systems
- mutate policy bundles
- inject credentials
- execute tools

State changes occur after the gateway receives the policy decision.

## Capability Classes

Policy classifies tools before execution.

- L0: read-only, no side effects
- L1: low-risk mutation
- L2: high-risk mutation requiring human approval
- L3: irreversible or highly sensitive action requiring stricter approval

Unknown or unclassified tools deny by default.

## Local Development Evaluation

The Phase 2 local gateway can now read a verified local development policy bundle and use it to make bounded decisions.

This is Developer Preview maturity. It is not production policy engine maturity.

For the local development bundle:

- `gateway_policy.yaml` contains simple policy rules.
- `risk_matrix.yaml` maps risk keys to bounded outcomes.
- the bundle must pass structure, version, checksum, and Ed25519 signature verification before evaluation.

Local policy rules match only:

- tool name
- capability class
- actor type, where the rule provides one

The local evaluator does not run scripts, evaluate expressions, call remote registries, execute tools, inject credentials, or start workflows.

Example local rule shape:

```text
rules:
  - id: allow_health_check_agent
    tool: health.check
    capability: L0
    actor_type: agent
    risk: local_l0_health_allow
  - id: allow_sandbox_note_write_agent
    tool: sandbox.note.write
    capability: L1
    actor_type: agent
    risk: local_l1_sandbox_allow
  - id: allow_metrics_read_agent
    tool: metrics.read
    capability: L0
    actor_type: agent
    risk: local_l0_allow
```

Example local risk matrix shape:

```text
entries:
  - id: local_l0_allow
    capability: L0
    decision: allow
    reason: local_l0_allowed
    message: Local L0 read request is allowed by the verified policy bundle.
```

Supported local decisions are bounded:

- `allow`
- `deny`
- `pending_approval`

Local evaluation fails closed when policy state is missing, malformed, ambiguous, unsupported, or not backed by a verified bundle.

The local development policy now allows `health.check` as an L0 request so the Phase 3 runtime can prove allowed-only built-in wrapper execution. Policy evaluation still does not execute the wrapper; it only returns the bounded decision that the gateway later enforces through the wrapper boundary.

The local development policy also allows `sandbox.note.write` as an L1 sandbox mutation. The policy decision alone is not enough to write. The runtime still requires execution authorization, credential class compatibility, idempotency context, sandbox directory validation, path containment, and audit evidence before the wrapper executes.

## Policy Bundle Contract

The policy engine evaluates only against the active policy bundle provided by the gateway.

A policy bundle should contain:

- declarative gateway policy
- risk matrix
- manifest
- checksums
- signatures where configured
- compatibility metadata

The policy engine must not evaluate live registry state as policy truth.

## Failure Behavior

Policy evaluation fails closed when:

- policy is missing
- policy cannot be parsed
- policy compatibility is invalid
- policy provenance is absent where required
- tool classification is missing
- request fields required by policy are absent
- conflicting policy rules cannot be resolved deterministically

Failure must produce an explicit deny or error decision that the gateway can audit.

## Testing Requirements

Policy engine tests must cover:

- allowed path
- denied path
- pending approval path
- unknown tool
- malformed policy
- incompatible policy
- deterministic repeated evaluation
- side-effect-free evaluation
- policy provenance in decisions

## Relationship to Wrappers

The policy engine decides. Wrappers enforce.

A wrapper may reject execution for safety, but it must not broaden authorization beyond the policy engine decision.
