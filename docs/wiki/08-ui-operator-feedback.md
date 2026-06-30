# AEGIS
# UI Operator Feedback

## What Is This?

This page explains how current backend evidence is intended to support the Slint UI inside the Tauri desktop shell.

The current UI implementation is a static scaffold only.

The purpose is to make sure runtime evidence remains renderable by a future graphical interface.

Future graphical operator feedback should be delivered through Slint inside the Tauri desktop shell.

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

The repository has an initial Tauri shell with a Slint graphical surface.

The surface renders static sample evidence from `src-tauri/ui/sample_evidence.json`.

It shows:

- execution timeline stages
- status cards
- one normalized error card
- sample recovery inspection evidence
- sample recovery planning guidance
- pre-alpha status
- backend authority-boundary language

It does not render live backend evidence, call gateway execution, define IPC commands, inspect live state logs, generate live recovery plans, replay execution, or implement dashboard behavior.

It has not implemented:

- IPC command layer
- live runtime evidence loading
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
