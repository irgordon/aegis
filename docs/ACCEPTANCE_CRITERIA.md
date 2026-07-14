# AEGIS
# Acceptance Criteria v1.0

## Purpose

This document defines the objective conditions required for AEGIS work to be considered complete.

Acceptance criteria are not suggestions. They are completion gates.

A feature, document, release, or implementation change is incomplete until it satisfies the applicable criteria in this document.

## Scope

These criteria apply to:

- documentation changes
- architecture changes
- policy changes
- gateway implementation
- wrapper implementation
- audit implementation
- state handling
- orchestrator integration
- release preparation

## General Acceptance Criteria

Every completed change must satisfy the following:

- The change has a clear purpose.
- The change aligns with OPERATING_DOCTRINE.md.
- The change preserves INVARIANTS.md.
- The change follows CODING_STYLE.md where code is affected.
- The change updates documentation where behavior or architecture changes.
- The change includes tests where behavior changes.
- The change fails closed where uncertainty exists.

## Documentation Acceptance Criteria

Documentation is accepted only when:

- It explains the problem being solved.
- It defines the intended behavior.
- It uses established AEGIS terminology.
- It avoids unnecessary duplication.
- It does not contradict higher-precedence documents.
- It is specific enough for implementation to proceed without guessing.
- It identifies affected components or flows.

## Functional Acceptance Criteria

Functional behavior is accepted only when:

- Tool requests are validated before execution.
- Unknown tools are denied by default.
- Every tool is capability classified before execution.
- Policy decisions return allow, deny, or pending.
- Human approval is required for configured high-risk actions.
- Denied actions do not partially execute.
- Approved actions execute through required wrappers.
- Orchestrators receive deterministic responses.

## Security Acceptance Criteria

Security behavior is accepted only when:

- Agents never receive long-lived credentials.
- Credentials are injected only at execution time.
- Secrets are never logged.
- Policy signatures and hashes are verified before activation.
- Invalid policy bundles are rejected.
- Wrapper failures halt execution.
- Authorization failures deny execution.
- Security-relevant defaults are documented.

## Determinism Acceptance Criteria

Deterministic behavior is accepted only when:

- Identical request, state, approval, and policy inputs produce identical decisions.
- Policy evaluation is side-effect free.
- Replays do not consult the LLM planning layer.
- Execution identity is stable and explicit.
- Long-running runs are pinned to their active policy bundle unless migration is documented.
- Non-deterministic values such as timestamps, IDs, and random values are explicitly captured when material.

## Audit Acceptance Criteria

Audit behavior is accepted only when every material decision records:

- run ID
- task ID or action ID where applicable
- execution ID
- tool name
- actor identity
- policy decision
- policy provenance
- timestamp
- result status
- denial or failure reason where applicable

Audit records must be structured and suitable for later investigation.

## State Acceptance Criteria

State handling is accepted only when:

- Pending approval state is durable.
- Approval state survives process restart.
- Terminal execution state is durable.
- Duplicate replay attempts are detected.
- Exactly-once execution is preserved for approved actions.
- State transitions are explicit and auditable.

## Human Approval Acceptance Criteria

Human approval behavior is accepted only when:

- Approval is bound to a specific action.
- Approval includes approver identity.
- Approval includes timestamp.
- Approval cannot be reused for different parameters.
- Stale approval reuse is denied.
- Denied approvals prevent execution.
- Pending approval does not require a blocking orchestrator loop.

## Policy Acceptance Criteria

Policy behavior is accepted only when:

- Policy is declarative.
- Policy is versioned.
- Policy is packaged as an immutable bundle.
- Policy includes provenance metadata.
- Policy declares compatibility requirements.
- Policy activation is explicit.
- Runtime policy mutation is prohibited.

## Wrapper Acceptance Criteria

Wrapper behavior is accepted only when:

- Wrappers enforce decisions but do not define policy truth.
- Wrappers fail closed.
- Wrapper execution is auditable.
- Credential wrappers do not expose secrets to agents.
- Permission wrappers restrict scope according to policy.
- HITL wrappers verify approval binding.

## API Acceptance Criteria

API behavior is accepted only when:

- Request schemas are validated.
- Response schemas are stable.
- Error states are explicit.
- Status values are bounded and documented.
- Malformed input never executes.
- API behavior is covered by contract tests.

## Testing Acceptance Criteria

Testing is accepted only when applicable test coverage includes:

- allowed path
- denied path
- pending approval path
- malformed request path
- invalid policy path
- wrapper failure path
- replay path
- duplicate execution path
- audit generation path
- fail-closed path

A bug fix should include a regression test.

## Performance Acceptance Criteria

Performance is accepted only when it does not weaken:

- security
- determinism
- auditability
- correctness
- policy integrity

Performance improvements must be measurable and must preserve architectural boundaries.

## Release Acceptance Criteria

A release is accepted only when:

- Documentation is current.
- Tests pass.
- Schemas are validated.
- Policy compatibility is documented.
- Changelog is updated.
- Known limitations are documented.
- Release artifacts are reproducible.
- Security implications are reviewed.

## Release Truth Acceptance Criteria

Release truth is accepted only when:

- User-facing release statements identify latest-release or current-development scope.
- README latest-release facts match the governed release-truth record.
- Current development target labels match package and desktop metadata.
- Changelog release headings distinguish published releases from unreleased work.
- Roadmap, phasemap, and tasks agree on active phase and task status.
- A task does not appear with conflicting or duplicate status rows.
- Historical public tags and releases remain unchanged.
- Automated verification rejects release-truth drift.

## Definition of Done

Work is done only when:

- requirements are satisfied
- invariants remain true
- implementation matches documentation
- tests validate behavior
- audit impact is understood
- security impact is reviewed
- tasks are updated
- CI passes where available

## Final Rule

If AEGIS cannot prove that a feature is safe, deterministic, documented, and auditable, the feature is not complete.
