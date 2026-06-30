# AEGIS
# Documentation Governance Standard v1.1

## Purpose

This document explains how AEGIS documentation should be written, organized, reviewed, and maintained.

AEGIS protects people and organizations from unsafe AI-driven actions. That protection only works when people can understand what the system does, why it exists, and how it should be used.

Documentation is part of the product. If the documentation cannot answer core questions clearly, define terms plainly, and guide the reader to the right next document, then the documentation has failed.

## Plain-Language Introduction

AEGIS is designed to answer one simple question:

**Should an AI system be allowed to do this?**

Modern AI systems can ask to send emails, change databases, open tickets, deploy software, or interact with business systems.

Before those actions happen, AEGIS checks whether the action is allowed, whether a person needs to approve it, and what evidence must be recorded.

This repository is built documentation-first.

That means the documents explain the rules before the software is written. The code must follow the documents, not the other way around.

## Documentation Audience Model

AEGIS documentation must support three audiences.

Each document should start simply and become more technical as needed.

### Audience 1: Non-Technical or Brand-New Readers

This reader may not know software engineering, cybersecurity, artificial intelligence, or enterprise architecture.

Write for approximately an 11th-grade reading level.

That does not mean the writing should be shallow. It means the writing should be clear.

For this audience, documentation should answer:

- What is this?
- Why does it matter?
- What problem does it solve?
- What should I read next?

Avoid starting with dense abstractions.

Start with the concrete problem.

Example:

> An AI wants to send an email outside the company. AEGIS checks whether that action is allowed before the email is sent.

Then introduce the technical term:

> This is called governed execution.

### Audience 2: Contributors and Developers

This reader wants to help improve the project.

They need to know where things belong, what rules govern changes, and how to avoid breaking the architecture.

This audience includes backend contributors and UI contributors. Backend contributors must document status, error, and evidence models in a way that UI contributors can render graphically without guessing. UI contributors must rely on backend evidence instead of recreating runtime decisions.

For this audience, documentation should answer:

- Which document should I read first?
- Where should I make a change?
- What rules must I follow?
- What tests or validation must pass?
- What should I update when behavior changes?

This audience needs clear instructions, not vague principles.

### Audience 3: Engineers and Architects

This reader needs deeper technical structure.

They may be reviewing security, designing a feature, integrating an orchestrator, or implementing a runtime component.

For this audience, documentation should answer:

- What are the trust boundaries?
- What are the runtime contracts?
- What must be deterministic?
- What state must be durable?
- What policy or schema version applies?
- What evidence proves correct behavior?

Technical depth belongs here, but it should still be readable.

Complexity is acceptable when the problem is complex. Unnecessary confusion is not.

## Documentation Reading Map

Use this map when entering the repository.

```text
New reader
   |
   v
README.md
   |
   v
PRD.md
   |
   v
ARCHITECTURE.md
   |
   v
INVARIANTS.md
   |
   v
API_SPEC.md and schemas/
   |
   v
Implementation
```

For governance and contribution work, use this path:

```text
Contributor
   |
   v
OPERATING_DOCTRINE.md
   |
   v
DOCUMENTATION.md
   |
   v
CODING_STYLE.md
   |
   v
ACCEPTANCE_CRITERIA.md
   |
   v
TASKS.md
```

For security and architecture review, use this path:

```text
Engineer or architect
   |
   v
ARCHITECTURE.md
   |
   v
SECURITY_MODEL.md
   |
   v
TRUST_BOUNDARIES.md
   |
   v
POLICY_ENGINE.md
   |
   v
ORCHESTRATOR_FSM_CONTRACT.md
   |
   v
AUDIT_LOGGING.md
```

## README Standard

`README.md` is the stable public orientation page for AEGIS.

It follows the repository communication model:

1. Why?
2. What?
3. How?
4. What If?

This mirrors the broader documentation philosophy:

- Problem
- Solution
- Conceptual model
- Engineering detail

README is for orientation.

Documentation is for understanding.

Architecture is for implementation.

The README should help a first-time reader understand why AEGIS exists, what problem it solves, how the execution model works at a high level, and what larger direction the project points toward.

It must remain short.

Do not use README as a roadmap, changelog, installation guide, developer handbook, runtime manual, or architecture document.

Only update README when:

- project identity changes
- primary purpose changes
- major architectural direction changes
- a production release fundamentally changes public understanding

Routine implementation progress belongs in:

- `CHANGELOG.md`
- `docs/ROADMAP.md`
- `docs/PHASEMAP.md`
- `docs/TASKS.md`
- relevant technical documentation

Implementation progress must not alter the README's Why -> What -> How -> What If structure.

## UI-Ready Documentation

AEGIS will use Tauri as the desktop application shell and Slint as the graphical UI layer when Phase 4 UI implementation begins.

Documentation should not describe runtime behavior as CLI-only when that behavior is meant to appear in the UI later.

Every backend status or error model that can be shown to an operator should be documented as structured data suitable for graphical display.

UI-facing runtime documentation should make these fields clear where applicable:

- status
- bounded code
- severity
- message
- reason
- next action
- location
- evidence references

The CLI may expose the same data for validation and automation, but the documentation should preserve enough structure for status cards, timelines, badges, plain-language errors, and evidence drill-down.

README is for orientation.

Documentation is for understanding.

Architecture is for implementation.

The UI is for operator feedback and intent capture, not for policy authority.

## README Stability

`README.md` is the stable public entry point for AEGIS.

It should explain the project identity, plain-language purpose, pre-alpha status, high-level execution model, and where to read next.

Do not use `README.md` as an implementation diary, roadmap summary, feature list, installation guide, or local usage walkthrough.

Implementation progress belongs in `CHANGELOG.md`, `docs/TASKS.md`, `docs/ROADMAP.md`, and the relevant architecture or runtime documents.

README should change only when project identity, primary purpose, major architectural direction, or production release status changes public understanding.

## Documentation Stability

Documentation should become progressively more stable as the implementation matures.

Routine implementation progress should not trigger broad documentation churn.

Use `CHANGELOG.md` as the primary record of routine progress.

Stable documents should change only when the work changes what that document governs:

- `README.md` changes rarely, usually only for production release or major architectural identity change.
- `docs/ARCHITECTURE.md` changes only when architecture changes.
- `docs/CODING_STYLE.md` changes only when engineering philosophy changes.
- `docs/OPERATING_DOCTRINE.md` almost never changes, because it defines repository governance.
- Feature-specific documents change only when a feature genuinely requires updated behavior, contracts, or review guidance.

When a feature changes behavior, update the narrowest relevant document instead of editing multiple high-level documents.

## Release Readiness Documentation

Documentation should increasingly support shipping the product rather than expanding conceptual coverage.

Once AEGIS enters a release cycle, documentation should favor operational clarity over additional concepts.

The release path is an authoritative planning input.

During a release cycle, documentation updates should help readers verify the current release objective, understand release limitations, remove release blockers, or maintain the current release checklist.

## If You Are Looking For...

| Goal | Read |
| --- | --- |
| Understand AEGIS in plain language | `README.md` |
| Understand the problem being solved | `docs/PRD.md` |
| Understand how the system is organized | `docs/ARCHITECTURE.md` |
| Understand what must never change | `docs/INVARIANTS.md` |
| Understand security assumptions | `docs/SECURITY_MODEL.md` |
| Understand attacks and mitigations | `docs/THREAT_MODEL.md` |
| Understand trust boundaries | `docs/TRUST_BOUNDARIES.md` |
| Understand policy decisions | `docs/POLICY_ENGINE.md` |
| Understand policy deployment | `docs/POLICY_DISTRIBUTION.md` |
| Understand API behavior | `docs/API_SPEC.md` |
| Understand audit evidence | `docs/AUDIT_LOGGING.md` |
| Understand test expectations | `docs/TEST_STRATEGY.md` |
| Understand AEGIS step by step | `docs/wiki/README.md` |
| Understand future UI evidence rendering | `docs/UI_EVIDENCE_CONTRACT.md` |
| Write or review code | `docs/CODING_STYLE.md` |
| Find current work | `docs/TASKS.md` |
| Understand release rules | `docs/RELEASE_PROCESS.md` |
| Understand the minimum usable local release path | `docs/RELEASE_PATH.md` |
| Check v0.4.0 release readiness | `docs/RELEASE_CHECKLIST_v0.4.0.md` |
| Understand architecture decisions | `docs/ADR.md` |
| Understand UI review requirements | `docs/UI-DESIGN.md` |

## Core Documentation Standard

Every document should answer four questions near the beginning:

1. What is this?
2. Why does it exist?
3. Who should read it?
4. What should the reader do next?

If a document does not answer these questions, it is not ready.

## Major Document Progression

Every major document should follow this progression:

1. What is this?
2. Why does it exist?
3. How does it work?
4. Engineering details

Maintain approximately an 11th-grade reading level until engineering sections begin.

Engineering sections may be more technical, but they should still define specialized terms before relying on them.

## Documentation Quality Rules

AEGIS documentation should be:

- clear
- structured
- searchable
- consistent
- readable by a broad audience
- precise enough for engineering work

Use short sections.

Use plain terms first.

Define unfamiliar terms before relying on them.

Prefer examples before abstract rules.

Use cross-references instead of copying the same large explanation into multiple documents.

## Reading Level Requirement

General explanations should target approximately an 11th-grade reading level.

This means:

- use direct sentences
- avoid unnecessary jargon
- explain acronyms
- define specialized terms
- use examples
- avoid large walls of text

It does not mean removing technical detail.

It means introducing technical detail in an order the reader can follow.

## Progressive Disclosure

Documents should move from simple to detailed.

Use this pattern where practical:

```text
Plain-language summary
   |
   v
Problem being solved
   |
   v
Key terms
   |
   v
Practical examples
   |
   v
Technical details
   |
   v
Formal rules or contracts
```

This allows a non-technical reader to understand the purpose while still giving engineers the detail they need.

## Term Definitions

Define important terms clearly.

Examples:

- **Gateway**: the checkpoint that reviews an AI action before it reaches an outside system.
- **Policy**: a written rule that says what is allowed, denied, or requires approval.
- **Policy Bundle**: a signed, immutable, versioned package containing gateway_policy, risk_matrix, manifest, signatures, and checksums.
- **Capability Level**: the risk category assigned to a tool before execution, from L0 read-only actions through L3 irreversible or highly sensitive actions.
- **Execution Identity**: the set of identifiers that uniquely describe an external action, including run ID, task ID, action ID, execution ID, attempt number, and replay token.
- **Orchestrator**: the AI planning system that proposes actions and submits tool requests to AEGIS, but does not authorize execution.
- **Fail Closed**: the rule that uncertainty results in denial rather than permission.
- **Wrapper**: an enforcement layer that safely executes or blocks an approved action.
- **Replay**: running a previously stored action again without asking the AI to rewrite it.
- **Deterministic**: producing the same result when given the same inputs.
- **Audit Record**: evidence showing what happened and why.
- **Human-in-the-Loop**: a required human approval step before a sensitive action can continue.

Do not assume the reader already knows these terms.

## Why So Much Documentation?

Most repositories start with code and slowly add documentation later.

AEGIS deliberately reverses that order.

The documents define the system before code is written.

This helps prevent:

- architecture drift
- hidden assumptions
- unsafe AI-generated code
- unclear security rules
- inconsistent implementation
- weak review standards

AEGIS is security-sensitive infrastructure. The software must be understandable before it can be trusted.

## Documentation Hierarchy

The repository follows strict precedence.

```text
OPERATING_DOCTRINE.md
   |
   v
PRD.md
   |
   v
ARCHITECTURE.md
   |
   v
INVARIANTS.md
   |
   v
DOCUMENTATION.md
   |
   v
USER_FLOWS.md
   |
   v
ACCEPTANCE_CRITERIA.md
   |
   v
CODING_STYLE.md
   |
   v
TASKS.md
```

Higher documents override lower documents when they conflict.

If two documents disagree, do not guess. Stop and resolve the conflict.

## Repository Documentation Categories

AEGIS documentation is grouped by purpose.

Top-level documentation-related directories support different parts of repository governance:

- `docs/` contains governed project documents.
- `docs/wiki/` contains explanatory knowledge base pages that help readers move from plain-language concepts to technical details.
- `schemas/` contains machine-readable protocol contracts.
- `examples/` contains reference examples that support documentation and validation.
- `prompts/` contains governed prompt artifacts where prompts are part of project behavior.
- `invariants/` contains standalone invariant files when an invariant is maintained outside `docs/`.

Standalone invariant files belong in `invariants/`. Invariants are not automatically placed under `docs/` unless they are part of a governed document such as `docs/INVARIANTS.md`.

### Governance

Governance documents explain how the project operates.

Examples:

- `OPERATING_DOCTRINE.md`
- `DOCUMENTATION.md`
- `CODING_STYLE.md`
- `ADR.md`
- `RELEASE_PROCESS.md`

### Product

Product documents explain what is being built and why.

Examples:

- `PRD.md`
- `USER_FLOWS.md`
- `ROADMAP.md`
- `PHASEMAP.md`

### Architecture

Architecture documents explain how the system is structured.

Examples:

- `ARCHITECTURE.md`
- `INVARIANTS.md`
- `SECURITY_MODEL.md`
- `TRUST_BOUNDARIES.md`
- `POLICY_ENGINE.md`
- `POLICY_DISTRIBUTION.md`

### Runtime Contracts

Runtime contract documents and schemas define how components communicate.

Examples:

- `API_SPEC.md`
- `ORCHESTRATOR_FSM_CONTRACT.md`
- `schemas/`

### User Interface

User interface documents define review standards for the Tauri desktop shell, Slint graphical UI layer, and any future visual presentation of AEGIS state.

Examples:

- `UI-DESIGN.md`
- `UI_EVIDENCE_CONTRACT.md`

### Knowledge Base

Knowledge base pages explain current AEGIS behavior in reader-friendly order.

Examples:

- `wiki/README.md`
- `wiki/02-execution-flow.md`
- `wiki/07-errors-and-fail-closed.md`

Knowledge base pages are not higher-authority documents. They should link to governed documents instead of redefining architecture, invariants, or task scope.

### Verification

Verification documents explain how correctness is proven.

Examples:

- `ACCEPTANCE_CRITERIA.md`
- `VALIDATION.md`
- `TEST_STRATEGY.md`
- `RUNTIME_EVIDENCE.md`

## Required Repository Documents

| Document | Core question answered |
| --- | --- |
| `README.md` | What is AEGIS? |
| `OPERATING_DOCTRINE.md` | How do we build the project? |
| `PRD.md` | What are we building and why? |
| `ARCHITECTURE.md` | How is the system organized? |
| `INVARIANTS.md` | What must never change? |
| `DOCUMENTATION.md` | How should documentation work? |
| `USER_FLOWS.md` | How should people and systems move through AEGIS? |
| `ACCEPTANCE_CRITERIA.md` | How do we know work is complete? |
| `CODING_STYLE.md` | How should code be written? |
| `TASKS.md` | What work is planned or complete? |
| `SECURITY_MODEL.md` | What does the system protect? |
| `THREAT_MODEL.md` | What can attack the system? |
| `API_SPEC.md` | How do systems communicate with AEGIS? |
| `TEST_STRATEGY.md` | How is behavior tested? |
| `ADR.md` | Why were major decisions made? |

## Document Lifecycle

Documents should move through a clear lifecycle.

```text
Draft
   |
   v
Review
   |
   v
Approval
   |
   v
Implementation
   |
   v
Maintenance
   |
   v
Revision
```

No document should become abandoned.

If a document no longer reflects the system, it must be updated or deliberately archived.

## Change Management

Documentation changes fall into three categories.

### Editorial Changes

These improve grammar, wording, formatting, or clarity.

They do not change behavior.

### Functional Changes

These change what the system does or how users interact with it.

They require review and may require updates to acceptance criteria, tests, and tasks.

### Architectural Changes

These affect architecture, invariants, governance, security, policy, runtime contracts, or compatibility.

They require formal review and should update `ADR.md` when the decision matters long term.

## Documentation Reviews

Reviewers should ask:

- Is the purpose clear?
- Is the intended audience clear?
- Are important terms defined?
- Does the document answer core reader questions?
- Does it conflict with another document?
- Does it match the architecture?
- Does it preserve invariants?
- Is it readable by a broad audience?
- Is the technical detail accurate?
- Does it point readers to the right next document?

## Cross References

Documents should reference concepts and stable document names.

Good:

> See `INVARIANTS.md` for non-negotiable architectural rules.

Avoid:

> See line 384 of gateway.py.

Documentation should survive refactoring.

## Diagrams and Tables

Use diagrams when they explain structure, sequence, trust boundaries, state transitions, or policy flow.

Use tables when comparing documents, responsibilities, requirements, or decisions.

Do not use diagrams or tables as decoration.

Every diagram and table should answer a reader question.

## AI-Generated Documentation

AI-generated documentation must meet the same quality standard as human-written documentation.

AI-generated changes must:

- preserve terminology
- preserve hierarchy
- avoid inventing requirements
- avoid contradicting architecture
- define unfamiliar terms
- remain readable
- pass review

Human review remains required.

## Source Code Comments Are Not Documentation

Comments explain small local details in code.

Repository documentation explains system behavior, governance, architecture, and contracts.

Do not use excessive comments to compensate for missing documentation.

## Documentation Anti-Patterns

Avoid:

- documentation written only after implementation
- unexplained jargon
- duplicate documents
- hidden architecture in source code
- undocumented assumptions
- large walls of text
- broken cross-references
- outdated diagrams
- design decisions recorded only in pull requests
- tribal knowledge

## Documentation Review Checklist

Before merging documentation, confirm:

- The purpose is clear.
- The audience is clear.
- The first page answers the core questions.
- Unfamiliar terms are defined.
- The document starts in plain language.
- Technical depth increases gradually.
- Examples are used where helpful.
- Related documents are linked.
- Terminology is consistent.
- The document does not contradict higher-precedence documents.
- The document is readable.
- The document supports future contributors.

## Definition of Complete Documentation

Documentation is complete when:

- requirements are defined
- architecture is documented
- terms are clear
- invariants are identified
- acceptance criteria exist
- reader questions are answered
- examples exist where concepts are unfamiliar
- implementation can proceed without guessing

If implementation requires guessing, the documentation is incomplete.

## Final Principle

Documentation is not paperwork.

Documentation is how AEGIS explains what it protects, what it connects, and what it prevents.

The purpose of documentation is not to describe software after it has been written.

The purpose of documentation is to make correct software easier to build and unsafe software harder to introduce.

Every document in AEGIS should reduce uncertainty, define terms clearly, preserve architectural intent, and help future contributors build the system consistently.
