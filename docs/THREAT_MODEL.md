# AEGIS
# Threat Model v1.0

## Purpose

This document identifies the primary threats AEGIS must withstand before and during implementation.

The threat model does not add product requirements. It clarifies how existing doctrine, architecture, invariants, and acceptance criteria are applied to adversarial or failure conditions.

## Protected Assets

AEGIS protects:

- enterprise credentials
- external systems
- policy integrity
- approval integrity
- audit integrity
- execution state
- replay determinism
- organization authority boundaries

## Trust Assumptions

AEGIS assumes:

- AI agents may be mistaken, compromised, prompt-injected, or over-permissive
- orchestrators may submit malformed or unauthorized requests
- external systems may fail or return unexpected responses
- policy registries may be unavailable
- operators may deploy incorrect configuration
- attackers may attempt to erase or alter evidence

AEGIS does not assume an AI agent can safely self-authorize.

## Threat: Gateway Bypass

An orchestrator or agent may attempt direct calls to external systems.

Required controls:

- external actions must route through the gateway
- production credentials must remain unavailable to agents
- direct execution paths are denied by architecture and deployment
- audit should make governed and ungoverned paths distinguishable

## Threat: Prompt-Injection Credential Theft

A malicious instruction may try to convince an agent to reveal or misuse secrets.

Required controls:

- agents do not receive long-lived credentials
- secrets are injected only at execution time
- secrets are not included in prompts, responses, logs, or audit records
- credential scope is minimized

## Threat: Policy Tampering

An attacker may alter policy to permit unauthorized execution.

Required controls:

- policy is packaged as immutable bundles
- bundles include hashes and provenance
- signatures are verified where signing is enabled
- incompatible or invalid bundles fail closed
- decisions record policy provenance

## Threat: Runtime Policy Drift

A gateway may silently begin enforcing changed policy during a run.

Required controls:

- runs are pinned to the active policy bundle
- activation is explicit
- registry changes do not mutate active local policy
- migrations are documented and auditable

## Threat: Approval Substitution

An approval for one action may be reused for another action.

Required controls:

- approvals bind to action identity, request parameters, policy decision, and execution state
- stale approvals are denied
- approval decisions include approver identity and timestamp
- denied approvals prevent execution

## Threat: Duplicate Execution

A replay, retry, or restart may execute the same approved action more than once.

Required controls:

- action identity is explicit
- idempotency keys, locks, replay tokens, or equivalent controls are used
- duplicate attempts are detected
- terminal state is durable

## Threat: Audit Tampering

An agent, compromised workflow, or attacker may attempt to hide activity.

Required controls:

- audit records are immutable or append-only where possible
- agents and orchestrators cannot modify audit records
- denied, pending, failed, replayed, and error outcomes are audited
- audit evidence is linked to execution and policy provenance

## Threat: Wrapper Drift

Wrappers may begin making policy decisions or diverge from policy intent.

Required controls:

- wrappers enforce decisions but do not define policy truth
- policy evaluation remains side-effect free
- wrapper failures fail closed
- wrapper behavior is audited

## Threat: Replay Re-Planning

Replay may accidentally call the planning layer and generate different intent.

Required controls:

- replay uses stored request and state
- replay does not consult the LLM
- policy and approval context are pinned or migrated through documented protocol
- replay uncertainty fails closed

## Threat: Schema Compatibility Failure

A component may interpret request, response, audit, policy, or state fields differently.

Required controls:

- schemas are versioned contracts
- compatibility is checked before activation
- unknown or unsupported fields do not create silent authorization
- invalid messages are rejected before execution

## Threat: Observability Leakage

Logs or audit records may expose secrets or sensitive payloads.

Required controls:

- secrets are never logged
- audit records use schema-driven fields
- sensitive values are redacted or represented by safe references
- operational logs are separate from durable audit evidence

## Threat Review Cadence

Threats must be reviewed when changes affect:

- credentials
- policy evaluation
- policy distribution
- wrappers
- approval flows
- replay
- audit records
- externally visible APIs
