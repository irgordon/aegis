# AEGIS
# UI Design Integrity Standard

## Purpose

This document defines the UI design integrity standard for future AEGIS frontend work.

AEGIS is intended to expose governed execution state through a Tauri graphical desktop interface. That interface must make security state, user intent, policy outcomes, and audit evidence clear before it tries to look polished.

AEGIS will use Tauri as the desktop application shell and Slint as the graphical UI layer when Phase 4 UI implementation begins.

## Scope

This standard applies to any future change that adds or modifies user-facing screens, controls, workflows, frontend routes, frontend components, desktop UI, web UI, or visual presentation of AEGIS state.

Generic references to frontend, desktop UI, or visual presentation in AEGIS UI planning now mean Slint UI rendered through the Tauri desktop application shell.

The Tauri desktop shell with Slint graphical UI layer is the intended primary operator surface. The CLI remains a support surface for validation, inspection, testing, and automation.

It applies before implementation begins.

## Core Principle

The UI exists to help people understand and control governed execution.

A good AEGIS interface makes state, risk, action, and consequence obvious. It must not hide policy decisions, approval state, execution status, audit status, or denial reasons behind decoration or vague wording.

The UI must be visual, understandable, and operator-friendly. It must use backend evidence as its source of truth. It must not become raw-log-first, and it must not become a terminal UI.

## v0.4.0 Visual Direction

The AEGIS desktop UI should be light, warm, and professional rather than dark-mode-first.

Use `#FAF0CA` as the dominant surface color and `#0D3B66` as the stabilizing authority color.

The interface should look intentional enough to build trust, but restrained enough that evidence remains more important than decoration.

The UI should not look like a raw terminal, debug console, generic admin template, or unfinished demo.

## v0.4.0 UI Palette

| Role | Color |
| --- | --- |
| Primary dark / navigation / high-emphasis | `#0D3B66` |
| Primary surface / main background / soft control grouping | `#FAF0CA` |
| Attention / warning / highlight | `#F4D35E` |
| Secondary accent / status emphasis | `#EE964B` |
| Critical / denied / error | `#F95738` |

## Palette Usage

`#FAF0CA`

Use for the dominant application surface, grouped panels, card backgrounds, calm reading areas, and non-hostile control groupings.

This should be the main visual base. It should make the application feel warmer than a sterile white interface without turning into dark mode.

`#0D3B66`

Use for the app header, navigation, primary labels, strong section titles, high-emphasis borders, and stable authority framing.

This is the main anchoring color. It should communicate structure, trust, and control.

`#F4D35E`

Use for warning states, pending states, pre-alpha notices, attention markers, and soft highlights.

Do not overuse it as a background color. It should guide attention, not dominate the interface.

`#EE964B`

Use for secondary status indicators, recovery/planning emphasis, non-critical operator attention, and transitional states.

Use where the user needs to notice something but the condition is not critical.

`#F95738`

Use only for errors, denied execution, failed-closed states, corrupted evidence, destructive warnings, or critical operator attention.

Do not use it for decoration.

## Typography

Typography must support operator comprehension before brand expression.

The selected font should prioritize:

- clear readability
- simple letter shapes
- distinct characters
- multiple weights and styles
- accessibility support
- consistent appearance across sizes

Do not hardcode a font name in documentation unless the repository bundles it or the implementation explicitly depends on it. Do not add font files, link proprietary font files, or claim a font is bundled unless it is actually bundled.

### Font Selection Criteria

Clear readability

The font must be easy to read on both large and small screens. Letters should not look too close together, too thin, or overly decorative.

Simple letter shapes

The font should use clean and simple shapes that reduce confusion. Characters such as uppercase I, lowercase l, and the number 1 should remain visually distinct.

Different weights and styles

The font family should support multiple weights, such as regular, medium, and bold. This allows the same font family to support titles, section headers, menus, labels, and body text.

Accessibility support

The font should have enough spacing and clear forms to support users with low vision or visual fatigue. The UI should remain readable at smaller sizes and under normal desktop scaling.

Consistent look

The font should remain balanced across sizes. A font that works well in headings must also remain readable in smaller controls, labels, and buttons.

### Serif Font Guidance

A serif font may be used for the AEGIS desktop UI if it remains highly legible in a software interface.

The selected serif should be restrained, readable, and modern. It should not feel ornamental, academic, old-fashioned, or print-first.

If the serif font reduces legibility in controls, status cards, or small labels, use it only for headings or replace it with a more readable UI-safe font.

## Evidence Color Semantics

Color must never be the only indicator of state.

| Evidence state | Color guidance |
| --- | --- |
| Allowed / healthy / completed | Use `#0D3B66` or neutral text treatment. |
| Warning / pending / pre-alpha | Use `#F4D35E` with clear labels. |
| Recovery / planning / operator attention | Use `#EE964B` with clear labels. |
| Denied / failed-closed / corrupted / error | Use `#F95738` with clear labels. |
| Surface / grouping | Use `#FAF0CA` or derived light variants. |

Status cards should combine color, text labels, and clear grouping. Color should support the label, not replace it.

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

Text must maintain readable contrast against the selected surface color.

Critical states must not rely on color alone.

Status cards should combine color, text labels, and clear grouping.

Small labels must remain readable.

The UI should avoid thin text on warm backgrounds.

Spacing should reduce visual fatigue.

Evidence type labels such as `Live backend evidence` and `Sample evidence` must remain clear.

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

## v0.4.0 Design Constraints

The v0.4.0 UI design must preserve these constraints:

- UI is not an authority boundary.
- UI does not authorize execution.
- UI does not issue credentials.
- UI does not perform replay.
- UI does not perform recovery execution.
- UI does not hide sample fallback.
- UI does not make sample evidence look live.
- UI does not imply production readiness.

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
- The v0.4.0 palette supports evidence clarity before decoration.
- Text remains readable on warm surfaces.
- Live and sample evidence labels remain clear.
- Rust owns business logic.
- The frontend only presents state and captures intent.
- IPC contracts are narrow and explicit.
- No hidden execution path bypasses the gateway.

## Non-Goals

This document does not define:

- additional frontend framework selection beyond the documented Slint direction
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

For the backend evidence that future UI components must render, see `UI_EVIDENCE_CONTRACT.md`.
