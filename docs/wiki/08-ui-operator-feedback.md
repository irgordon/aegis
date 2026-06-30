# AEGIS
# UI Operator Feedback

## What Is This?

This page explains how current backend evidence is intended to support a future Tauri operator UI.

No UI is implemented here.

The purpose is to make sure runtime evidence remains renderable by a future graphical interface.

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
  -> future Tauri presentation layer
  -> operator timeline, cards, badges, and panels
```

This keeps Rust as the authority for execution behavior and keeps the frontend focused on presentation.

## Current UI Status

The repository has documented the Tauri graphical UI foundation and UI design integrity standard.

It has not implemented:

- Tauri shell scaffold
- React or TypeScript frontend
- IPC command layer
- timeline component
- error card component
- approval UI
- recovery UI

Those belong to later platform work.

This page describes how future UI work should consume existing evidence. It does not describe implemented UI behavior.

For the formal evidence rendering contract, see `docs/UI_EVIDENCE_CONTRACT.md`.

## Documentation Boundary

Backend docs should describe evidence fields clearly enough for UI contributors to render them without guessing.

UI docs should describe presentation and feedback rules without duplicating policy logic.

If a UI needs a field that the backend does not emit, the runtime contract should be changed deliberately in the appropriate phase.
