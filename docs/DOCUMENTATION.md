AEGIS

AI Execution Governance & Interception System

DOCUMENTATION.md

Documentation Governance Standard v1.0

⸻

Purpose

This document defines how documentation is created, maintained, reviewed, versioned, and governed throughout the AEGIS project.

The repository follows Documentation-Driven Engineering (DDE).

Documentation is not supplementary material.

Documentation is the authoritative description of the system.

Implementation exists to satisfy documentation.

Documentation never exists to explain undocumented implementation.

⸻

Philosophy

The quality of software is directly proportional to the quality of its documentation.

Good documentation enables:

* consistent engineering
* deterministic implementation
* maintainability
* onboarding
* security reviews
* architecture reviews
* AI-assisted development
* compliance
* auditing

Poor documentation creates:

* architectural drift
* inconsistent implementations
* duplicated logic
* undocumented assumptions
* institutional knowledge
* security mistakes

AEGIS treats documentation as part of the product.

⸻

Documentation Principles

The repository follows ten governing principles.

1. Documentation First

Documentation precedes implementation.

Requirements must exist before code.

Architecture must exist before implementation.

Acceptance criteria must exist before testing.

⸻

2. Documentation is Authoritative

If implementation and documentation disagree:

The documentation is presumed correct until formally updated.

Developers should not silently “fix” documentation to match implementation.

The discrepancy should be investigated.

⸻

3. Documentation Evolves with the System

Every meaningful architectural or behavioral change requires documentation updates.

Documentation debt is engineering debt.

⸻

4. Documentation Explains Intent

Documentation explains:

* why something exists
* what problem it solves
* how it should behave

Implementation explains:

* how it works

These are different responsibilities.

⸻

5. Documentation Must Be Version Controlled

Documentation belongs in source control.

Every significant documentation update should accompany the implementation that requires it.

⸻

6. Documentation Should Age Well

Avoid documenting temporary implementation details unless necessary.

Document concepts.

Not incidental implementation choices.

⸻

7. Documentation Should Be Searchable

Documents should use:

* consistent terminology
* descriptive headings
* stable structure

Avoid hidden knowledge.

⸻

8. Documentation Should Be Independent

Each document has a single responsibility.

Avoid duplicating large sections across multiple documents.

Reference documents instead.

⸻

9. Documentation Should Be Reviewable

Documentation changes deserve review.

Architecture documentation is code.

Treat it accordingly.

⸻

10. Documentation Should Reduce Risk

Every document should reduce uncertainty.

If documentation creates ambiguity, improve it.

⸻

Documentation Hierarchy

The repository follows strict precedence.

OPERATING_DOCTRINE.md
↓
PRD.md
↓
ARCHITECTURE.md
↓
INVARIANTS.md
↓
DOCUMENTATION.md
↓
USER_FLOWS.md
↓
ACCEPTANCE_CRITERIA.md
↓
CODING_STYLE.md
↓
TASKS.md

Higher documents override lower documents.

⸻

Repository Documentation

The repository contains four categories of documentation.

Governance

Defines how the project operates.

Examples:

* OPERATING_DOCTRINE.md
* DOCUMENTATION.md
* CODING_STYLE.md

⸻

Product

Defines what is being built.

Examples:

* PRD.md
* USER_FLOWS.md
* ROADMAP.md

⸻

Architecture

Defines system structure.

Examples:

* ARCHITECTURE.md
* INVARIANTS.md
* SECURITY_MODEL.md
* THREAT_MODEL.md
* POLICY_ENGINE.md

⸻

Verification

Defines how correctness is demonstrated.

Examples:

* ACCEPTANCE_CRITERIA.md
* VALIDATION.md
* TEST_STRATEGY.md

⸻

Required Repository Documents

OPERATING_DOCTRINE.md

Defines engineering governance.

Answers:

How do we build software?

⸻

PRD.md

Defines product requirements.

Answers:

What are we building?

⸻

ARCHITECTURE.md

Defines structure.

Answers:

How is the system organized?

⸻

INVARIANTS.md

Defines permanent properties.

Answers:

What must never change?

⸻

DOCUMENTATION.md

Defines documentation governance.

Answers:

How is documentation maintained?

⸻

USER_FLOWS.md

Defines user interaction.

Answers:

How should the product behave?

⸻

CODING_STYLE.md

Defines implementation standards.

Answers:

How should code be written?

⸻

ACCEPTANCE_CRITERIA.md

Defines completion.

Answers:

How do we know a feature is done?

⸻

TASKS.md

Defines current execution.

Answers:

What is being worked on?

⸻

Document Lifecycle

Every document follows the same lifecycle.

Draft
↓
Review
↓
Approval
↓
Implementation
↓
Maintenance
↓
Revision
↓
Archive

No document should become abandoned.

⸻

Change Management

Documentation changes fall into three categories.

Editorial

Grammar

Formatting

Clarification

No architectural review required.

⸻

Functional

Behavior changes.

Requires review.

⸻

Architectural

Changes affecting:

* architecture
* invariants
* governance
* security

Require formal review.

⸻

Documentation Reviews

Reviewers should ask:

Does the document:

* explain intent?
* match architecture?
* conflict with another document?
* define terminology consistently?
* remain implementation independent?
* support future contributors?

⸻

Terminology

Terminology should remain consistent.

Examples:

Always use:

Policy Bundle

Execution Identity

Replay Token

Capability Class

Human-in-the-Loop

Policy Engine

Security Wrapper

Avoid inventing alternate names.

⸻

Diagrams

Diagrams should illustrate:

* architecture
* execution
* state transitions
* trust boundaries
* policy flow

Avoid decorative diagrams.

Every diagram should answer a question.

⸻

Tables

Prefer tables for:

* comparisons
* capabilities
* permissions
* mappings
* responsibilities

Avoid large prose when a table communicates more clearly.

⸻

Cross References

Documents should reference:

concepts

not implementation.

Example:

Correct

See INVARIANTS.md, Invariant 12.

Incorrect

See line 384 of gateway.py.

Documentation should survive refactoring.

⸻

Architecture Decisions

Major decisions should reference:

DECISIONS.md

Every architectural change should answer:

Why was this chosen?

What alternatives existed?

Why were they rejected?

⸻

AI-Generated Documentation

AI-generated documentation must satisfy the same quality standards as human-written documentation.

AI should:

* preserve terminology
* preserve hierarchy
* avoid inventing requirements
* avoid contradicting architecture

Human review remains mandatory.

⸻

Source Code Comments

Comments are not documentation.

Comments explain:

small local context.

Documentation explains:

system behavior.

Do not replace missing documentation with excessive comments.

⸻

Documentation Metrics

Good documentation should answer:

What?

Why?

Who?

When?

How?

Without requiring the reader to inspect implementation.

⸻

Review Checklist

Before merging documentation:

□ Is terminology consistent?

□ Does it contradict another document?

□ Does it duplicate information unnecessarily?

□ Does it explain intent?

□ Is it technically accurate?

□ Is it implementation independent?

□ Is it maintainable?

□ Is it readable?

⸻

Documentation Anti-Patterns

Avoid:

Documentation written after implementation

Duplicate documents

Architecture hidden in source code

Large undocumented assumptions

Copy/paste across files

Outdated diagrams

Broken cross references

Temporary notes committed permanently

Design decisions only recorded in pull requests

Tribal knowledge

⸻

Versioning

Documentation versions follow repository evolution.

Major architectural revisions should update:

* version number
* revision history
* affected documents

Older documentation should remain available through version control.

⸻

Ownership

Every repository contributor owns documentation quality.

Maintainers own documentation governance.

Reviewers own documentation consistency.

AI assistants assist documentation.

Humans approve documentation.

⸻

Definition of Complete Documentation

Documentation is complete when:

* requirements are defined
* architecture is documented
* terminology is consistent
* invariants are identified
* acceptance criteria exist
* diagrams explain structure
* implementation can proceed without guessing

If implementation requires guessing, the documentation is incomplete.

⸻

Final Principle

The purpose of documentation is not to describe software after it has been written.

The purpose of documentation is to make correct software inevitable.

Every document in AEGIS should reduce uncertainty, preserve architectural intent, and enable future contributors—human or AI—to build the system consistently.

Documentation is the foundation upon which the entire project is built.
