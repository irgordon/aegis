# AEGIS
# Architectural Decisions

## What Is This?

This page summarizes important architectural decisions in plain language.

It does not replace `docs/ADR.md`, `docs/ARCHITECTURE.md`, or `docs/INVARIANTS.md`.

Use it as an orientation guide before reading the authoritative documents.

## Decision: Rust Backend Owns Runtime Authority

The current governed runtime is implemented in Rust.

The backend owns request validation, policy bundle verification, policy evaluation, execution authorization, credential boundaries, wrapper dispatch, audit evidence, state evidence, recovery inspection, and recovery planning.

Future UI work must consume backend evidence instead of recreating these decisions.

## Decision: Deterministic Execution

AEGIS must produce repeatable behavior when given the same request, explicit context, policy bundle, and state evidence.

No wiki page should describe hidden randomness, implicit recovery, or best-effort execution as acceptable runtime behavior.

## Decision: Fail Closed

AEGIS denies or stops execution when it cannot prove the request should continue.

This applies to malformed requests, invalid policy bundles, unsupported policy state, authorization mismatch, credential boundary failure, wrapper failure, audit persistence failure, and state evidence corruption.

## Decision: Policy Before Execution

Policy evaluation happens before authorization and wrapper execution.

Wrappers never make policy decisions.

This prevents execution code from becoming a hidden policy engine.

## Decision: Verified Bundle Before Policy Evaluation

Policy is evaluated only after local bundle structure, versions, checksums, and the checksum manifest signature verify.

This prevents AEGIS from making decisions from untrusted or modified policy files.

## Decision: Explicit Execution Authorization

Policy allow is necessary but not enough.

Allowed requests must receive explicit execution authorization before wrapper dispatch.

Authorization binds wrapper, version, tool, capability, scope, credential class, and execution identity references.

## Decision: Credential Classes Before Credential Values

AEGIS models credential class and safe handle references before any real credential provider exists.

This allows the execution model to enforce credential boundaries without introducing secrets early.

## Decision: Built-In Local Wrappers First

The current wrappers are intentionally local:

- `health.check` proves L0 read-only execution.
- `sandbox.note.write` proves controlled L1 local mutation.

They do not call external systems, use network access, spawn subprocesses, or retrieve secrets.

## Decision: Audit and State Are Separate

Audit records explain decisions and evidence.

State records explain lifecycle progression.

Keeping them separate makes future recovery work possible without turning audit logs into a state machine.

## Decision: Recovery Planning Is Read-Only

Recovery inspection and planning classify existing state evidence.

They do not replay, repair, resume, or mutate execution.

This keeps recovery work bounded until replay and recovery behavior are explicitly implemented.

The `candidate_for_future_replay` outcome is a future evaluation classification, not implemented replay.

## Decision: UI Is Presentation, Not Authority

The future Tauri UI may render status, errors, timelines, and evidence.

The CLI remains a support surface for validation, inspection, testing, and automation.

The UI must not decide policy, authorize execution, inject credentials, dispatch wrappers, make recovery decisions, or invent lifecycle state.

## Decision: Abstractions Must Earn Their Place

AEGIS should not add new top-level concepts just because they might be useful later.

Every module, trait, document, state, and boundary should solve today's problem or directly enable the next planned milestone.

## Decision: Documentation Stability

Routine implementation progress belongs in `CHANGELOG.md` and `docs/TASKS.md`.

Stable documents change only when their governed scope changes.

The README is a stable public orientation page, not a status page.
