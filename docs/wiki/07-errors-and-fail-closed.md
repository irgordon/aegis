# AEGIS
# Errors and Fail-Closed Behavior

## What Is This?

This page explains how AEGIS reports failures.

AEGIS must fail closed, but the failure should still be understandable.

An operator or developer should be able to answer:

- What happened?
- Why did it happen?
- What should I do next?
- Where in the gateway did it fail?

## Normal Error Shape

Externally visible failures use a structured error report.

Required fields:

| Field | Meaning |
| --- | --- |
| `code` | Bounded machine-readable error code |
| `severity` | Bounded severity value |
| `message` | Plain-language summary of what happened |
| `reason` | Plain-language explanation of why it happened |
| `next_action` | Plain-language guidance for what to do next |
| `location` | Gateway area where the failure occurred |

Optional safe fields may include:

- request ID
- execution ID reference
- policy bundle ID
- tool name
- wrapper name
- source error kind

Error reports must not include secrets, credentials, private key material, raw tokens, or stack traces.

## Fail-Closed Rule

When AEGIS cannot prove the request is safe to continue, it denies or stops execution.

Failing closed is not an exceptional edge case. It is a core safety rule.

## Common Conditions and States

This table explains common operator-facing conditions and state classifications. It is not a promise that every row maps one-to-one to a separate exported error code.

| Error or state | What happened | Runtime behavior | Operator next action |
| --- | --- | --- | --- |
| request validation failed | Request JSON was malformed or did not satisfy the request contract | Fail closed before policy evaluation | Fix the request shape and retry validation |
| policy bundle verification failed | The local policy bundle could not be trusted | Fail closed before policy evaluation | Rebuild or replace the policy bundle |
| checksum mismatch | A policy bundle file does not match recorded checksum metadata | Fail closed before policy evaluation | Regenerate checksums and signature from trusted files |
| signature verification failed | Signed checksum metadata could not be verified | Fail closed before policy evaluation | Regenerate or replace signature metadata and public key fixture |
| policy denied | Verified policy matched a deny outcome | Do not authorize or execute | Review policy rule and request intent |
| policy pending | Verified policy requires approval | Do not authorize or execute | Wait for a future approval workflow; no approval execution exists today |
| authorization failed | Execution authorization was missing or did not match wrapper, version, capability, or scope | Do not dispatch wrapper | Fix the authorization binding or policy/runtime mapping |
| credential boundary failed | Wrapper credential requirement did not match authorization | Do not inject handle or dispatch wrapper | Fix wrapper credential requirement or authorization credential class |
| credential injection failed | Required safe local credential handle could not be provided or validated | Do not execute wrapper | Check credential class, wrapper binding, and authorization binding |
| wrapper missing | Requested wrapper is not registered | Fail closed before wrapper execution | Register the wrapper or correct the request |
| wrapper version mismatch | Requested wrapper version does not match registered or authorized version | Fail closed before wrapper execution | Align policy, authorization, and wrapper version |
| wrapper execution failed | Registered wrapper rejected context or failed during its bounded action | Fail closed and record error evidence | Fix wrapper input, sandbox, idempotency, or wrapper implementation |
| audit persistence failed | Execution evidence could not be appended when audit logging was requested | Report audit failed | Fix audit log path or filesystem permissions before relying on output |
| state log write failed | Lifecycle transition could not be appended when state logging was requested | Fail closed or stop completion reporting | Fix state log path or filesystem permissions |
| state inspection failed | State log could not be parsed or inspected safely | Return inspection failure | Fix malformed state evidence or preserve it for manual review |
| recovery planning failed | Recovery plan could not be produced from inspection evidence | Return inspection failed or planning failure evidence | Fix inspection input before planning |
| `failed_closed` | Runtime stopped because a safety gate failed | No execution continues | Use structured error evidence to correct the cause |
| `audit_failed` | Execution happened but required audit persistence failed | Do not report normal completion | Fix audit persistence before treating execution as complete |
| `inspection_failed` | Recovery inspection could not produce trustworthy findings | No recovery plan should be trusted | Preserve or quarantine corrupted state evidence for review |
| `not_recoverable_terminal` | Execution reached a terminal state | Do not recover or replay | Treat execution as closed |
| `not_recoverable_corrupted` | State evidence is corrupted or inconsistent | Do not recover automatically | Preserve evidence and investigate manually |
| `candidate_for_audit_retry` | Execution may only need future audit persistence handling | No audit retry is executed today | Preserve evidence for a future audit-specific recovery feature |
| `candidate_for_future_replay` | Execution may be eligible for future replay evaluation | No replay is executed today | Preserve evidence for a future replay evaluation feature |

## Plain-Language Standard

Error messages should be written for people first.

Good:

```text
The policy bundle could not be used.
The checksum metadata does not match the bundle file.
Regenerate the checksum manifest and signature, then rerun validation.
```

Avoid exposing internal enum names as the main explanation.

Developer details may exist, but they must be safe and clearly secondary.

## Future UI Use

A future Tauri UI should render structured errors as operator-facing feedback.

Useful visual forms include:

- error card
- severity badge
- failed lifecycle step
- next-action text
- evidence drill-down

The UI must not translate errors into new policy decisions.
