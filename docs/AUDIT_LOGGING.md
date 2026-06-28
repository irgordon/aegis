# AEGIS
# Audit Logging v1.0

## Purpose

This document defines audit evidence requirements for AEGIS.

Audit records are not ordinary logs. They are durable governance evidence used for security review, compliance, incident response, replay analysis, and operational accountability.

## Scope

Audit evidence is required for:

- request receipt
- schema validation outcome
- policy decision
- human approval decision
- wrapper execution result
- tool execution result
- denied execution
- failed execution
- pending approval
- replay attempt
- cancellation
- policy activation
- policy rollback

## Audit Principles

Audit records must be:

- structured
- attributable
- timestamped
- linked to execution identity
- linked to policy provenance
- durable
- protected from agent and orchestrator modification
- free of secrets

## Required Decision Fields

Every material decision record should include:

- audit record ID
- execution ID
- run ID where available
- action ID where available
- actor identity where available
- tool name where applicable
- decision or status
- reason code where applicable
- policy bundle ID
- policy version
- policy hash
- timestamp
- environment
- component emitting the record

## Approval Fields

Approval audit records should include:

- approval ID
- execution ID
- action ID
- approver identity
- approval decision
- approval timestamp
- approval scope
- policy provenance
- expiration or stale-state outcome where applicable

## Replay Fields

Replay audit records should include:

- replay token
- original execution ID
- replay attempt number
- stored request reference
- pinned policy bundle
- duplicate detection result
- replay outcome

## Policy Activation Fields

Policy activation audit records should include:

- bundle ID
- policy version
- policy hash
- signer identity where available
- activation timestamp
- environment
- compatibility result
- activation actor or process
- previous bundle ID where applicable

## Secret Handling

Audit records must not contain:

- API keys
- passwords
- bearer tokens
- private keys
- signing keys
- raw credentials
- unredacted authorization headers
- session cookies

Sensitive payloads should be represented by safe references, hashes, redacted fields, or documented summaries.

## Operational Logs

Operational logs may support debugging and operations, but they do not replace audit records.

Logs must not be the only location for governance evidence.

## Immutability

Audit storage should be append-only or write-once-read-many where practical.

Agents and orchestrators must not be able to modify or delete audit records.

## Failure Behavior

If AEGIS cannot produce mandatory audit evidence for a material decision, execution must fail closed unless a documented emergency operating mode explicitly applies.

Emergency operating modes must not erase the requirement to reconstruct what happened.

## Validation

Audit validation must verify:

- required fields exist
- records are structured
- secrets are absent
- denials include reasons
- failures include reasons
- approval decisions include approver identity
- replay attempts are recorded
- policy provenance is present
