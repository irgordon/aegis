# AEGIS
# UI Operator Feedback

## What Is This?

This page explains how current backend evidence supports the Slint UI inside the Tauri desktop shell.

The current UI implementation renders sample evidence and a fixed live `health.check` backend evidence path.

The purpose is to make runtime evidence understandable without giving the UI authority.

Graphical operator feedback is delivered through Slint inside the Tauri desktop shell.

## UI Role

The UI is an operator surface.

It may:

- show request status
- show policy results
- show authorization status
- show credential boundary status
- show wrapper execution status
- show audit and state evidence
- show structured errors
- capture user intent in future workflows after those workflows are implemented

It must not:

- evaluate policy
- authorize execution
- decide credential class
- dispatch wrappers directly
- invent lifecycle states
- make recovery decisions
- bypass the gateway
- create hidden execution paths
- approve, replay, recover, authorize, issue credentials, or execute arbitrary gateway requests

## Evidence the UI Should Render

The current runtime already emits structured evidence that a UI can display.

| Evidence | Suggested UI form |
| --- | --- |
| request validation | status badge or first timeline step |
| policy bundle verification | trust card with checksum and signature status |
| policy evaluation | allow, deny, or pending decision card |
| execution authorization | authorization binding panel |
| credential boundary | credential class status row |
| credential injection | safe handle reference status row |
| wrapper execution | wrapper result card |
| audit record | evidence details panel |
| state transitions | execution timeline |
| recovery inspection | state integrity panel |
| recovery plan | bounded future recovery guidance |
| structured error report | error card with reason and next action |

## Operator Feedback Principles

The UI should answer:

- What is happening?
- Why did it happen?
- What can I do next?
- Where did it fail?
- What evidence supports this result?

The UI should not hide fail-closed behavior behind vague language.

If AEGIS denies or stops execution, the operator should see the structured reason and next action.

## Visual Feedback Pipeline

```text
runtime evidence
  -> structured JSON
  -> future Slint presentation layer inside Tauri
  -> operator timeline, cards, badges, and panels
```

This keeps Rust as the authority for execution behavior and keeps the frontend focused on presentation.

## Current UI Status

The repository has a Tauri shell with a Slint graphical surface.

The surface renders static sample evidence from `src-tauri/ui/sample_evidence.json`.

It also exposes a narrow read-only Tauri command, `get_health_check_evidence`, that returns live backend evidence for the fixed local `health.check` path.

It shows:

- execution timeline stages
- status cards
- one normalized error card
- sample recovery inspection evidence
- sample recovery planning guidance
- Developer Preview status
- prerelease distribution posture
- backend authority-boundary language

It can render fixed live health-check evidence when available.

It does not submit arbitrary gateway requests, execute `sandbox.note.write`, choose wrapper names, choose policy bundles, choose audit or state paths, inspect live state logs, generate live recovery plans, replay execution, recover execution, or implement dashboard behavior.

The current UI uses a warm palette, status-first cards, explicit live/sample/not-available labels, a neutral no-error state, and an authority-boundary message.

The desktop identifies the public `v0.4.1` Developer Preview as a prerelease that is unsigned and not notarized.

The visual readability review passed for v0.4.0. Evidence is recorded in `docs/assets/release/v0.4.0-desktop-readability-review.md`.

It has not implemented:

- broad IPC command layer
- live audit or state evidence loading
- approval UI
- recovery execution UI
- replay UI

Those belong to later platform work.

This page describes how current and future UI work should consume backend evidence. It does not make the UI authoritative.

For the formal evidence rendering contract, see `docs/UI_EVIDENCE_CONTRACT.md`.

## Documentation Boundary

Backend docs should describe evidence fields clearly enough for UI contributors to render them without guessing.

UI docs should describe presentation and feedback rules without duplicating policy logic.

If a UI needs a field that the backend does not emit, the runtime contract should be changed deliberately in the appropriate phase.
