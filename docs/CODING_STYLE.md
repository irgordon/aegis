# AEGIS
## Coding Style v1.0

## Purpose

This document defines the coding style for AEGIS.

The goal is not cosmetic consistency. The goal is readable, maintainable, deterministic, security-sensitive code that a new developer can understand without reconstructing hidden intent.

Code is part of the governance system. If code is hard to reason about, the system is harder to trust.

## Scope

This style applies to all implementation languages used in AEGIS, including Python, TypeScript, Bash, Go, Rust, Zig, configuration files, schema files, and future language ports.

Language-specific overlays may add stricter rules. They must not weaken this document.

## Primary Rule

Code shall be optimized for the next maintainer.

The next maintainer may be:

- a human developer
- a security reviewer
- a compliance reviewer
- an incident responder
- an AI coding agent
- the original author six months later

Prefer simpler designs over clever designs.

Avoid speculative abstractions.

Remove unused code rather than preserving it for later.

Keep implementation proportional to the current roadmap phase.

## Reading Order

Files should read top down.

A reader should encounter:

1. Purpose
2. Public interface
3. Main entry points
4. High-level orchestration
5. Supporting helpers
6. Low-level utilities

Do not force readers to jump across the file to understand the main flow.

## File Structure

Prefer this structure:

1. Module docstring or file header, if needed
2. Imports
3. Constants
4. Public types
5. Public functions or classes
6. Internal helpers
7. Private utilities

The most important behavior should appear early.

## Naming

Names should describe intent.

Use names that explain what the object does or represents.

Prefer:

- `policy_decision`
- `execution_id`
- `approval_record`
- `validate_policy_bundle`

Avoid vague names:

- `data`
- `info`
- `thing`
- `obj`
- `handle`
- `stuff`

Short names are acceptable only for very small scopes where meaning is obvious.

## Function Design

Functions should do one thing.

A function should be easy to describe in one sentence.

If a function needs multiple paragraphs to explain, split it.

Prefer:

- validate input
- evaluate policy
- build audit record
- persist state
- return response

Do not combine unrelated responsibilities into one function.

## Function Length

Functions should be short.

A function longer than roughly 50 lines requires justification.

A function longer than roughly 100 lines should almost always be split.

Exceptions may exist for simple declarative mappings, generated code, or structured tables.

## File Length

Source files should remain focused.

A source file should not exceed 1,200 lines.

If a file grows beyond 1,200 lines, split it into smaller modules unless there is a strong architectural reason not to.

Large files make review harder, increase merge conflicts, and hide responsibility boundaries.

## Nesting

Avoid deep nesting.

Prefer guard clauses and early returns.

Do not build logic pyramids.

Bad:

```text
if request:
    if policy:
        if approved:
            execute()
```

Better:

```text
if request is invalid:
    deny

if policy is missing:
    deny

if approval is required:
    return pending

execute
```

Three levels of nesting is usually too much.

## Control Flow

Control flow should be direct.

Prefer:

- explicit validation
- explicit branching
- explicit return values
- clear failure paths

Avoid:

- hidden mutation
- implicit fallthrough
- excessive callbacks
- unnecessary inheritance
- complex metaprogramming

## Error Handling

Errors must be explicit.

Every error should answer:

- what failed
- why it failed
- whether execution continued
- what the caller should do

Do not swallow errors.

Do not convert serious errors into warnings.

Do not allow uncertain state to proceed.

AEGIS fails closed.

Externally visible gateway errors should use structured reports rather than ad hoc strings.

Each report should include a bounded code, bounded location, severity, message, reason, and next action. Messages and next actions should be written for a human operator first. Developer details belong in separate safe diagnostic fields.

When adding a new error code:

1. Add it to the shared bounded error type.
2. Write a plain-language message, reason, and next action.
3. Add tests for JSON output, audit evidence, and secret absence.
4. Keep the failure behavior fail-closed.

## Security-Sensitive Code

Security-sensitive code must be especially plain.

This includes:

- authentication
- authorization
- policy evaluation
- approval verification
- credential injection
- audit generation
- state transition handling
- replay handling
- wrapper execution

Avoid clever patterns in these areas.

Security code should look boring.

Boring code is easier to review.

## Comments

Comments should be minimal and useful.

Good comments explain why something exists.

Bad comments restate what the code already says.

Prefer clear code over explanatory comments.

Use comments for:

- security rationale
- non-obvious tradeoffs
- protocol requirements
- compatibility constraints
- references to architecture decisions

Avoid comments like:

```text
# increment counter
# return result
# check if valid
```

The code should already make those clear.

## Logging

Never log secrets.

Never log:

- API keys
- passwords
- tokens
- signing keys
- private keys
- session cookies
- raw credentials
- unredacted authorization headers

Be cautious with:

- email addresses
- user IDs
- file paths
- production hostnames
- request parameters
- tool payloads

Logging should support operations without leaking sensitive data.

## Audit Records

Audit records are not normal logs.

Audit records must be structured, durable, attributable, and suitable for investigation.

Do not place audit evidence only in free-form text logs.

Audit fields should be schema-driven.

## Validation

Validate at boundaries.

Boundary examples:

- API input
- config loading
- policy bundle loading
- wrapper invocation
- approval callback
- external tool response
- persisted state hydration

Internal code may assume validated types only after validation has occurred.

## State Mutation

Mutation should be deliberate and localized.

Separate code into phases:

1. Validate
2. Decide
3. Mutate
4. Record
5. Return

Do not mix decision logic with state mutation unless the architecture explicitly requires it.

Policy evaluation must remain side-effect free.

## Determinism

Code should avoid non-determinism unless it is explicitly part of the design.

Be careful with:

- current time
- random values
- unordered maps
- concurrency
- retries
- generated IDs
- external service responses

When non-determinism is required, capture it explicitly in state or audit metadata.

## Time

Time handling must be explicit.

Use UTC for persisted timestamps.

Avoid local time in execution logic.

Do not compare naive timestamps.

Timestamps used for audit, approval, replay, or policy provenance must be unambiguous.

## Configuration

Configuration must be explicit and validated.

Invalid configuration should stop startup or fail the relevant operation closed.

Do not silently apply defaults that change security posture.

Security-relevant defaults must be documented.

## Dependencies

Dependencies should be minimal and justified.

Before adding a dependency, consider:

- what problem it solves
- whether the standard library is sufficient
- maintenance activity
- licensing
- security history
- transitive dependency risk

A dependency is part of the attack surface.

## Tests

Code should be easy to test.

Test public behavior, not private implementation details.

Required test categories include:

- expected path
- denied path
- malformed input
- missing policy
- invalid approval
- replay behavior
- idempotency behavior
- audit generation
- fail-closed behavior

Every bug fix should add or update a regression test.

## Schemas and Contracts

Schemas are contracts.

Do not casually change schema fields.

Schema changes require:

- compatibility review
- documentation update
- tests
- versioning where applicable

## API Design

APIs should be boring and explicit.

Requests and responses should use stable fields.

Avoid overloaded fields whose meaning changes by context.

Prefer clear enums over free-form status strings.

Every API error should be understandable to callers.

## Concurrency

Concurrency must be justified.

Concurrent code should make ownership, locks, retries, and failure behavior explicit.

Avoid concurrency in security-critical paths unless needed.

When concurrency is used, test race conditions and duplicate execution behavior.

## Idempotency

External execution must account for retry safety.

Where retries exist, the code must define:

- idempotency key
- duplicate detection
- retry limit
- failure behavior
- audit behavior

Do not retry irreversible actions without explicit design.

## Source Organization

Prefer modules that map to architectural responsibility.

Examples:

- gateway
- policy
- wrappers
- audit
- state
- approvals
- schemas
- telemetry

Do not organize code only by technical convenience.

## Formatting

Use automated formatters where available.

Formatting debates should be settled by tooling.

Manual formatting preferences should not override standard project tools.

## Language-Specific Rules

Language-specific rules belong in `/invariants` or language overlay documents.

Examples:

- Python type hints required
- TypeScript strict mode required
- Bash uses `set -euo pipefail`
- Rust avoids `unwrap` in production paths
- Go passes context explicitly
- Zig uses explicit allocators

Language overlays may strengthen this document.

They may not weaken it.

## Prohibited Patterns

Avoid:

- global mutable state
- hidden network calls
- runtime policy mutation
- broad exception swallowing
- logging secrets
- long functions
- deeply nested functions
- business logic in API handlers
- security logic in prompts
- unreviewed permission expansion
- comments that compensate for unclear code

## Review Checklist

Before merging code, reviewers should ask:

- Is the main flow readable top down?
- Are responsibilities separated?
- Are invariants preserved?
- Are failure paths explicit?
- Does the code fail closed?
- Are logs safe?
- Are audit records structured?
- Are tests meaningful?
- Is documentation updated?
- Is there unnecessary complexity?

## AI Code Generation Rules

AI-generated code must follow the same rules as human-written code.

AI agents must:

- read governance docs first
- preserve invariants
- avoid inventing requirements
- keep files focused
- avoid deep nesting
- write tests
- update documentation where needed
- explain assumptions in task output

AI-generated code receives no lower review standard.

## Final Rule

AEGIS code should be clear enough that a new maintainer can understand what it protects, what it connects, and what it prevents.

If code cannot be understood, it cannot be trusted.

If code cannot be trusted, it does not belong in AEGIS.
