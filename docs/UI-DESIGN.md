# AEGIS
# UI Design Integrity Standard

## Purpose

This document defines the UI design integrity standard for future AEGIS frontend work.

AEGIS is intended to expose governed execution state through a Tauri graphical desktop interface. That interface must make security state, user intent, policy outcomes, and audit evidence clear before it tries to look polished.

## Scope

This standard applies to any future change that adds or modifies user-facing screens, controls, workflows, frontend routes, frontend components, desktop UI, web UI, or visual presentation of AEGIS state.

The Tauri desktop UI is the intended primary operator surface. The CLI remains a support surface for validation, inspection, testing, and automation.

It applies before implementation begins.

## Core Principle

The UI exists to help people understand and control governed execution.

A good AEGIS interface makes state, risk, action, and consequence obvious. It must not hide policy decisions, approval state, execution status, audit status, or denial reasons behind decoration or vague wording.

## UI Integrity Review

Every UI change must complete a UI Integrity Review before implementation.

The review must answer:

- What screen or workflow is changing?
- What state is being shown?
- What user decision or action is being requested?
- What risk, policy decision, approval state, or audit status could be misunderstood?
- How does the design prevent accidental unsafe action?

The review must cover clarity, navigation, actionability, feedback, and accessibility.

## Clarity

Every UI change must answer:

- Can a first-time user understand the screen in under 10 seconds?
- Is the primary purpose obvious?
- Is the current state visible?

Screens should use plain labels for execution state, approval state, risk level, policy decision, and audit status. Important state should not depend on tooltips, hidden menus, or decorative treatment.

## Navigation

Every UI change must answer:

- Does the layout remain stable?
- Are controls located where users expect them?
- Has navigation depth been minimized?

Navigation should keep users close to the governed action they are reviewing. Moving between request details, policy outcome, approval status, and audit evidence should not require unnecessary screen changes.

## Actionability

Every UI change must answer:

- Is the next action obvious?
- Are actions located near the information they affect?
- Are destructive actions clearly distinguished?

Actions should appear near the request, approval, or audit information they affect. Destructive or irreversible actions must be visually and textually distinct from routine actions.

## Feedback

Every UI change must answer:

- Are loading states visible?
- Are empty states explained?
- Do errors explain what happened, why, and what to do next?

Feedback must be specific. A denied request should explain the safe denial reason. A pending action should show what is waiting and what happens next. Errors must not expose secrets.

## Accessibility

Every UI change must answer:

- Is information understandable without relying only on color?
- Are icons paired with text?
- Is typography readable?

Color may support meaning, but it must not be the only way meaning is conveyed. Icons that affect decisions must have visible labels or adjacent explanatory text. Text must remain readable at normal desktop and mobile sizes.

## UI Invariants

These UI invariants are hard requirements for future UI work.

### Information Before Decoration

UI must make state, risk, action, and consequence clear before visual polish.

Decorative elements must never obscure execution state, approval state, risk level, policy decision, or audit status.

### Rust Owns Business Logic

The frontend must not decide whether an AI action is allowed, denied, pending, escalated, or executed.

Those decisions belong to the Rust runtime and policy boundary.

### Frontend Owns Presentation

The frontend displays state, captures user intent, and presents feedback.

It must not duplicate gateway policy logic.

### Thin IPC Boundaries

Frontend-to-runtime communication must use narrow, explicit request and response contracts.

The UI must not reach around the gateway boundary or create hidden execution paths.

## Rust and Frontend Boundary

Rust owns governed execution behavior, policy decisions, gateway state, audit evidence construction, and security-sensitive validation.

The frontend owns layout, presentation, input capture, status display, and user feedback.

Communication between frontend and Rust must use explicit contracts. A UI change must not create another path to execute tools, authorize actions, inject credentials, alter policy state, or bypass audit evidence.

## Review Checklist

Before implementing any UI change, confirm:

- The UI Integrity Review has been completed.
- The primary screen purpose is clear within 10 seconds.
- Current state is visible.
- Policy decision, approval state, risk level, and audit status are not hidden.
- Controls are located near the information they affect.
- Destructive actions are clearly distinguished.
- Loading, empty, denied, pending, and error states are represented.
- Errors explain what happened, why, and what to do next.
- Information does not rely only on color.
- Icons that affect decisions are paired with text.
- Typography is readable.
- Rust owns business logic.
- The frontend only presents state and captures intent.
- IPC contracts are narrow and explicit.
- No hidden execution path bypasses the gateway.

## Non-Goals

This document does not define:

- frontend framework selection
- visual brand system
- component library
- CSS architecture
- routing model
- desktop shell implementation details
- web deployment model
- mockups or visual assets
- runtime policy behavior
- gateway execution behavior

Those decisions require separate tasks and must preserve this standard.
