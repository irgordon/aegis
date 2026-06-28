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
