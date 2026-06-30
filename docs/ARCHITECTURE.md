AEGIS

AI Execution Governance & Interception System

ARCHITECTURE.md

Reference Architecture v1.0

⸻

Purpose

This document defines the reference architecture for AEGIS.

Unlike the Operating Doctrine, which governs engineering behavior, this document governs the technical structure of the system.

Every implementation, regardless of programming language or deployment model, shall preserve the architectural boundaries described here.

The architecture exists to ensure that every implementation remains:

* deterministic
* secure
* auditable
* maintainable
* portable

This document intentionally describes logical architecture rather than implementation details.

⸻

Architectural Goals

AEGIS is designed to solve one problem:

How can organizations safely allow AI agents to perform real-world actions without granting them unrestricted authority?

The architecture achieves this by separating:

* planning
* governance
* enforcement
* execution
* auditing

into independent components.

⸻

Design Philosophy

AEGIS follows a small set of core architectural principles.

Deterministic by Design

Given identical:

* inputs
* execution state
* approvals
* policy bundle

the gateway must produce identical execution behavior.

⸻

Governance over Autonomy

Agents do not decide what they are allowed to do.

They request.

AEGIS governs.

⸻

Policy is Data

Security policy is configuration.

It is never embedded inside prompts or orchestration logic.

⸻

Decision and Enforcement are Separate

The component deciding whether an action is allowed is not the component enforcing the action.

This separation simplifies:

* auditing
* testing
* reasoning
* maintenance

⸻

Explicit State

Execution state is always explicit.

The system should never depend on hidden in-memory assumptions.

Runtime Ownership Rule

Execution state is the single source of truth for request progression.

No subsystem may invent an independent lifecycle.

⸻

Fail Closed

Uncertainty results in denial.

Never authorization.

⸻

High-Level Architecture

                        AI Orchestrator
                              │
                              ▼
                    Tool Execution Request
                              │
                              ▼
                     ┌──────────────────┐
                     │      AEGIS       │
                     │ Execution Gateway│
                     └──────────────────┘
                              │
        ┌──────────────┬───────────────┬───────────────┐
        ▼              ▼               ▼
 Policy Engine    Security Wrappers   Audit Engine
        │              │               │
        └──────┬───────┴───────┬───────┘
               ▼               ▼
       Approved Execution   Durable State
               │
               ▼
      External Enterprise Systems

No component bypasses the gateway.

⸻

Current Local Runtime

The current repository contains a local Rust gateway runtime.

It can:

* read request JSON
* validate the request
* verify a local policy bundle
* evaluate local policy and risk matrix files
* dispatch the built-in `health.check` wrapper after policy allows it
* execute the built-in `sandbox.note.write` wrapper inside an explicit local sandbox
* pass a safe local development credential handle to `sandbox.note.write`
* return structured JSON
* optionally append a local JSONL audit record
* optionally append a local JSONL execution state log
* expose lifecycle state

It does not yet provide external wrapper execution, production credential injection, vault integration, approval workflow, durable execution state, replay, HTTP, UI, or production deployment.

This section describes current implementation state. The remaining architecture describes the target system boundaries that current and future implementations must preserve.

⸻

Operator Surface

AEGIS is intended to include a graphical desktop operator interface delivered through a Tauri window.

The current Phase 4 operator surface uses Tauri as the desktop application shell and Slint as the graphical UI layer.

The CLI is a support surface for execution, validation, inspection, testing, and automation. It is not the primary operator experience.

AEGIS should not use a terminal UI as its primary interface and should not be treated as a CLI-only tool.

Every runtime status exposed by the CLI must also be representable as structured data suitable for graphical display.

The graphical interface should help operators understand execution state without reading raw logs first. It should favor:

* status cards
* execution timelines
* bounded status badges
* plain-language errors
* evidence drill-down

The UI is an operator surface, not an authority boundary.

The backend remains authoritative for validation, policy evaluation, execution authorization, credential handling, wrapper execution, audit evidence, state evidence, recovery inspection, and recovery planning.

Current backend authority includes:

* request validation
* policy bundle verification
* policy evaluation
* execution authorization
* credential class checks
* credential injection
* wrapper dispatch and execution
* audit evidence
* execution state
* recovery inspection
* recovery planning

The UI must not bypass gateway execution logic, recreate policy decisions, authorize execution, inject credentials, alter audit evidence, or invent lifecycle state.

⸻

Visual Feedback Model

The graphical UI should render backend evidence as plain operator feedback.

Each current runtime stage should be representable visually:

| Runtime stage | Operator-facing feedback |
| --- | --- |
| Request | Request received |
| Validation | Request valid or invalid |
| Verified Policy Bundle | Policy bundle verified or failed |
| Policy Evaluation | Policy allowed, denied, or pending |
| Execution Authorization | Execution authorized or blocked |
| Credential Class Boundary | Credential boundary satisfied or failed |
| Local Credential Injection Boundary | Credential handle issued, not required, or failed |
| Wrapper Dispatch | Wrapper selected, not found, or blocked |
| Wrapper Execution | Wrapper executed, failed, or not executed |
| Lifecycle | Execution completed, failed closed, audit failed, or stopped at a known state |
| Audit | Audit recorded or failed |
| State Log | State recorded or failed |
| Recovery Inspection | Recovery inspection available or failed |
| Recovery Plan | Recovery plan available, not recoverable, audit retry only, or future evaluation only |

UI-facing errors must remain:

* bounded
* plain-language
* structured
* secret-safe
* actionable

The UI should be able to render:

* error code
* severity
* message
* reason
* next action
* location

Raw logs may support investigation, but they should not be the first way operators understand execution state.

⸻

Trust Boundaries

AEGIS defines five trust boundaries.

Boundary 1

AI Orchestrator

The orchestrator is trusted to generate requests.

It is not trusted to authorize execution.

⸻

Boundary 2

Gateway

The gateway is trusted to coordinate execution.

It is responsible for:

* request validation
* routing
* policy coordination
* execution orchestration

⸻

Boundary 3

Policy Engine

The policy engine is trusted to evaluate policy.

It performs no side effects.

It never executes tools.

⸻

Boundary 4

Wrappers

Wrappers enforce security decisions.

They never invent policy.

⸻

Boundary 5

External Systems

Everything beyond the gateway is considered external.

Examples include:

* GitHub
* AWS
* Azure
* Jira
* Slack
* databases
* Kubernetes
* email systems

⸻

Primary Components

AI Orchestrator

Responsibilities:

* planning
* reasoning
* tool selection
* workflow sequencing

Responsibilities excluded:

* authorization
* approval
* credential management
* auditing

⸻

Gateway

The gateway coordinates execution.

Responsibilities:

* validate request
* assign execution identity
* evaluate policy
* invoke wrappers
* execute tool
* record audit
* return response

The gateway is stateless except for durable execution references.

⸻

Policy Engine

The policy engine answers one question:

Should this action execute?

Inputs:

* ToolCallRequest
* execution context
* policy bundle

Outputs:

* Allow
* Deny
* Pending Approval

The policy engine shall not:

* mutate state
* execute tools
* write audit records
* inject credentials

⸻

Security Wrappers

Wrappers perform enforcement.

Examples:

* credential injection
* execution sandbox
* permission isolation
* approval verification

Wrappers are deterministic.

⸻

Audit Engine

Responsibilities:

* structured audit records
* execution provenance
* policy provenance
* replay metadata

Audit generation is mandatory.

⸻

State Store

Durable storage for:

* execution state
* pending approvals
* replay information
* execution identity

State survives:

* crashes
* deployments
* restarts

⸻

Execution Lifecycle

The target gateway processes requests in the following order.

Receive Request
↓
Validate Schema
↓
Assign Execution Identity
↓
Load Policy Bundle
↓
Evaluate Policy
↓
Execute Wrappers
↓
Execute Tool
↓
Generate Audit Record
↓
Persist State
↓
Return Response

Every step produces deterministic output.

Current Local Lifecycle State

The local Phase 3 runtime exposes an in-memory execution lifecycle so operators and tests can see where a request is while AEGIS processes it.

For a new reader: the lifecycle is a progress trail. It shows whether a request has been validated, checked against a verified policy bundle, dispatched to a wrapper, executed, audited, completed, or failed closed.

The current bounded states are:

* Created
* Validated
* BundleVerified
* PolicyEvaluated
* Authorized
* Dispatching
* Executed
* Audited
* Completed
* FailedClosed
* AuditFailed

Successful local execution follows this order:

Created
↓
Validated
↓
BundleVerified
↓
PolicyEvaluated
↓
Authorized
↓
Dispatching
↓
Executed
↓
Audited
↓
Completed

Fail-closed paths terminate at FailedClosed from the boundary that detected the problem.

Audit persistence failure after wrapper execution terminates at AuditFailed.

Contributors must add a state only when current runtime behavior exercises it and tests prove the valid transition order.

For engineers: this is not durable execution state, replay, recovery, or a workflow engine. It is a typed transition model that prevents impossible runtime state reports and prepares later durable execution state without adding persistence now.

Execution Identity Binding

Execution identity is assigned after schema validation and before policy evaluation.

The execution_id is derived from or bound to:

* orchestrator_id
* workflow_id
* tool_call_id
* policy_bundle_version
* nonce

The architecture does not require a specific final hash algorithm.

Execution identity must remain stable across Gateway replicas for the same accepted execution context. It must prevent replay ambiguity across workflows, tool calls, policy bundle versions, and retries.

The accepted Policy Bundle version is pinned when Execution Identity is assigned.

⸻

Policy Architecture

Policies are independent from execution.

Policy Registry
↓
Review
↓
Validation
↓
Signing
↓
Immutable Bundle
↓
Deployment
↓
Gateway

The gateway never evaluates live registry state.

The gateway must not evaluate live registry state during request execution. Runtime policy selection is based only on the activated local immutable bundle pinned to the accepted execution context.

⸻

Policy Bundle

Every bundle contains:

* gateway_policy.yaml
* risk_matrix.yaml
* manifest.yaml
* signatures
* checksums

Bundles are:

* immutable
* signed
* versioned

The Gateway must reject a Policy Bundle when:

* signatures fail
* checksums fail
* required bundle files are missing
* manifest.yaml policy version does not match the referenced risk matrix version
* bundle identity does not match the version pinned during Execution Identity assignment

⸻

Capability Classification

Tools are classified before execution.

L0

Read-only.

No side effects.

Examples:

* metrics
* search
* read APIs

⸻

L1

Low-risk mutation.

Examples:

* draft documents
* sandbox updates

⸻

L2

High-risk mutation.

Requires human approval according to the active Policy Bundle.

Examples:

* production changes
* external email
* infrastructure updates

⸻

L3

Irreversible.

Requires quorum-based approval according to the active Policy Bundle.

Examples:

* delete production data
* terminate infrastructure
* financial transfers

L2 and L3 approval requirements are defined by the Policy Bundle, not hardcoded into Gateway runtime logic.

Policy defines:

* quorum size
* approver roles
* break-glass rules
* expiration
* approval context requirements

The Gateway enforces the policy decision. It does not invent approval rules.

Break-glass use must be auditable and must not bypass evidence requirements.

⸻

Security Model

Security follows Zero Trust.

Principles:

* least privilege
* explicit authorization
* credential isolation
* deny by default
* immutable audit

⸻

Credential Architecture

Agents never receive standing credentials.

Execution flow:

Policy Approved
↓
Wrapper Injects Credential
↓
Tool Executes
↓
Credential Destroyed

No credential survives execution.

Wrapper Determinism and Redaction

Wrappers must remain deterministic from the Gateway perspective.

Credential injection, secret material, tokens, and volatile external values must not be written into audit records or replay payloads.

Audit redaction follows this contract:

* secrets are never recorded in plaintext
* redacted fields preserve field presence
* redacted fields preserve enough metadata to prove the wrapper path used
* replay uses stored wrapper configuration, not stored secrets
* secret values are reacquired through approved runtime mechanisms during real execution

Redaction is part of deterministic replay safety.

⸻

Human Approval

Approval is asynchronous.

The gateway returns:

Pending

The orchestrator must:

* persist state
* terminate active execution
* await approval event

No blocking loops.

Approval tokens must be:

* single-use
* scoped to execution_id
* scoped to tool_call_hash
* time-limited by TTL
* revocable before execution
* invalid after use, expiration, revocation, or execution context mismatch

Approval state must not be reused across a changed request, changed tool parameters, changed wrapper configuration, or changed policy bundle.

⸻

Replay Architecture

Replay uses stored execution state.

Replay does not call the LLM.

Replay executes:

Stored Action
↓
Stored Parameters
↓
Stored Policy
↓
Stored Approval
↓
Execution

Replay evidence includes:

* stored action
* stored parameters
* stored policy bundle identity
* stored approval evidence where applicable
* stored wrapper configuration
* stored external system schema version
* stored response mapping version if versioned
* stored audit redaction metadata

Wrapper updates or external API and schema changes must not silently alter replay interpretation.

⸻

Audit Architecture

Every decision produces evidence.

Audit includes:

* execution identity
* timestamps
* policy provenance
* approval information
* decision
* result

Audit records are immutable.

⸻

Failure Model

The gateway fails closed.

Failures include:

* invalid policy
* invalid approval
* wrapper failure
* schema failure
* credential failure
* compatibility failure

Denied execution is safer than uncertain execution.

⸻

Determinism

Determinism requires:

* immutable policy
* durable state
* explicit identities
* replay safety
* idempotency

No hidden randomness.

Idempotency

For L1 through L3 mutation-capable actions, the Gateway must generate or assign an idempotency key and pass it to supported external systems.

The idempotency key must bind to:

* execution_id
* tool_call_hash
* target system
* operation type
* policy bundle version

Idempotency protects against double execution after crashes, retries, network failures, or partial state persistence.

If an external system does not support idempotency keys, the Gateway must record that limitation in audit evidence and apply the safer behavior required by policy.

⸻

Scalability

Multiple gateways may execute simultaneously.

Each gateway:

* loads identical policy bundle
* evaluates locally
* records independent audit evidence

No runtime dependency exists on a centralized authorization service.

⸻

High Availability

Gateway replicas are interchangeable.

Requirements:

* shared durable state
* immutable policy bundle
* identical schemas

Gateway replacement should not affect execution behavior.

⸻

Extensibility

New tools should require:

1. Schema
2. Capability classification
3. Policy
4. Wrapper compatibility
5. Tests

Adding tools must not require gateway redesign.

⸻

Language Independence

This architecture is implementation agnostic.

Reference implementations may exist in:

* Python
* Go
* Rust
* Zig

All implementations shall preserve:

* architecture
* invariants
* execution semantics

⸻

Repository Structure

docs/
config/
schemas/
src/
tests/
examples/
prompts/
invariants/
scripts/

Documentation governs implementation.

⸻

Component Responsibilities

Component	Responsibility
Orchestrator	Plan work
Gateway	Coordinate execution
Policy Engine	Decide authorization
Wrappers	Enforce security
Audit Engine	Record evidence
State Store	Preserve execution
External Tool	Perform requested action

Responsibilities shall not overlap.

⸻

Architectural Constraints

Every implementation shall preserve:

* deterministic execution
* immutable policy
* durable state
* zero-trust credentials
* explicit execution identity
* policy provenance
* comprehensive auditing
* wrapper enforcement
* fail-closed behavior

These constraints are mandatory.

⸻

Evolution

Architecture evolves deliberately.

Changes require:

* updated doctrine
* updated architecture
* updated invariants
* updated acceptance criteria
* updated tests

Implementation follows documentation.

Never the reverse.

⸻

Final Architecture Statement

AEGIS is not an orchestration framework.

It is an execution governance architecture.

The orchestrator determines what should happen.

The policy engine determines whether it may happen.

The wrappers determine how it is safely executed.

The audit engine determines how it is proven.

This separation of responsibilities is the foundation of AEGIS and shall be preserved across all future implementations.
