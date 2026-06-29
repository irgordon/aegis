# AEGIS
# Knowledge Base

## Purpose

This directory explains AEGIS from plain language to engineering detail.

Use it when the root `README.md` is too short and the governed architecture documents are more detailed than you need at first.

The wiki does not replace authoritative project documents. If the wiki and a governed document disagree, the governed document wins and the wiki should be corrected.

## Reading Order

Read these pages in order if you are new to AEGIS:

1. [Overview](01-overview.md)
2. [Execution Flow](02-execution-flow.md)
3. [Major Components](03-major-components.md)
4. [Policy and Authorization](04-policy-and-authorization.md)
5. [Credentials and Wrappers](05-credentials-and-wrappers.md)
6. [Audit, State, and Recovery](06-audit-state-and-recovery.md)
7. [Errors and Fail-Closed Behavior](07-errors-and-fail-closed.md)
8. [UI Operator Feedback](08-ui-operator-feedback.md)
9. [Architectural Decisions](09-architectural-decisions.md)
10. [Contributor Guide](10-contributor-guide.md)

## What Belongs Here

The wiki explains how the current repository fits together.

It should:

- reduce cognitive load
- explain concepts before implementation details
- link to higher-authority documents instead of copying them
- describe what exists today
- make future UI rendering needs understandable without implementing UI

It should not:

- define new architecture
- introduce new invariants
- replace `docs/ARCHITECTURE.md`
- replace `docs/INVARIANTS.md`
- behave like a changelog
- become a development diary

## Authoritative References

Use these documents when implementation details matter:

- `docs/OPERATING_DOCTRINE.md`
- `docs/ARCHITECTURE.md`
- `docs/INVARIANTS.md`
- `docs/TRUST_BOUNDARIES.md`
- `docs/POLICY_ENGINE.md`
- `docs/AUDIT_LOGGING.md`
- `docs/API_SPEC.md`
- `docs/TASKS.md`
- `CHANGELOG.md`

