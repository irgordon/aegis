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

## Local Development Audit Log

The Phase 2 local gateway can save audit records to a local file.

For a new reader: this means AEGIS can now keep a local record of why the gateway allowed, denied, or paused a request. These records help explain what happened after the process exits.

This is still pre-alpha. It is not production audit infrastructure.

## JSONL Format

The local development audit log uses newline-delimited JSON.

Each completed gateway decision appends one JSON object on one line:

```text
{"event_type":"policy_decision","status":"allowed",...}
{"event_type":"policy_decision","status":"denied",...}
```

The runtime accepts:

```bash
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log audit.jsonl \
  schemas/examples/valid/ToolCallRequest.json
```

If `--audit-log` is omitted, stdout behavior remains unchanged.

## Append-Only Behavior

The local audit writer:

- creates the file if it is missing
- opens the file in append mode
- writes exactly one audit record for each completed gateway decision
- writes a newline after each JSON object
- flushes the write before the process exits
- does not truncate existing audit files
- does not rewrite previous records

Malformed requests, unsupported tools, denied decisions, pending decisions, and allowed decisions all produce audit records.

For the local runtime, record order follows successful append order in the target file. The current local binary processes one request per invocation.

## Local Audit Failure Behavior

If `--audit-log` is provided and the record cannot be written, the local runtime fails closed.

The runtime returns an error instead of silently continuing without durable evidence.

Expected local failure cases include:

- audit path is a directory
- parent directory is missing
- file permissions prevent writing
- record serialization fails
- file write or flush fails

## Durability Assumptions

The local writer flushes process buffers before exit.

It does not claim:

- database durability
- WORM compliance
- hash chaining
- audit log signatures
- encryption at rest
- compression
- remote SIEM delivery
- syslog integration

Those controls belong to later phases.

## Contributor Validation

Audit persistence changes should run:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

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
