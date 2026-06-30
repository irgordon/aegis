# AEGIS
# Audit, State, and Recovery

## What Is This?

This page explains the evidence AEGIS records around decisions, execution, lifecycle state, recovery inspection, and recovery planning.

## Audit Evidence

Audit evidence explains what happened and why.

The current runtime can build audit records for:

- request identity
- actor and tool identity
- capability class
- policy bundle identity
- policy evaluation
- response status
- execution authorization
- credential boundary and injection status
- wrapper execution
- lifecycle state
- structured errors

When an audit log path is supplied, AEGIS appends one JSON object per completed gateway decision to a local JSONL audit log.

The audit log is not a database, not WORM storage, not a SIEM integration, and not cryptographically signed.

## State Evidence

State evidence records lifecycle transitions.

When a state log path is supplied, AEGIS appends one JSON object per lifecycle transition to a local JSONL state log.

The state log is separate from the audit log.

```text
audit log = decision and evidence
state log = lifecycle progression
stdout JSON = current command output
```

## Lifecycle States

The current lifecycle uses bounded state names such as:

- `created`
- `validated`
- `bundle_verified`
- `policy_evaluated`
- `authorized`
- `dispatching`
- `executed`
- `audited`
- `completed`
- `failed_closed`
- `audit_failed`

Invalid transitions are rejected.

## Recovery Inspection

Recovery inspection reads local execution state logs and classifies what it finds.

It can detect problems such as:

- malformed state records
- duplicated lifecycle indexes
- reordered state records
- impossible transitions
- inconsistent request references
- inconsistent tool or wrapper references
- corrupted or incomplete evidence

Inspection is read-only.

It does not replay, resume, repair, or mutate state.

## Recovery Planning

Recovery planning consumes inspection output and produces bounded guidance.

It can classify executions into outcomes such as:

- `not_recoverable_terminal`
- `not_recoverable_corrupted`
- `candidate_for_audit_retry`
- `candidate_for_future_replay`
- `inspection_failed`

The planner does not execute recovery.

It only explains what a future recovery system may be allowed to consider.

The `candidate_for_future_replay` label means future replay evaluation only. Replay execution is not implemented.

## Why Audit and State Stay Separate

Audit and state logs serve different purposes.

Audit records explain decisions, policy evidence, authorization evidence, wrapper evidence, and error evidence.

State records explain lifecycle progression in order.

Keeping them separate prevents a recovery mechanism from treating audit evidence as execution state, and prevents an audit system from becoming a state machine.

## Future UI Use

The current Tauri UI renders fixed live health-check evidence and labeled sample recovery evidence.

Current and future Tauri UI views can render:

- audit evidence as decision cards
- state evidence as timelines
- recovery inspection as integrity findings
- recovery plans as bounded operator guidance

The UI must display backend evidence. It must not invent recovery actions or execution state.

Recovery execution, replay execution, and audit retry execution are not implemented.
