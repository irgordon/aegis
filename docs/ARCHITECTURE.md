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

AEGIS follows six fundamental architectural principles.

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

The gateway processes requests in the following order.

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

Requires human approval.

Examples:

* production changes
* external email
* infrastructure updates

⸻

L3

Irreversible.

Examples:

* delete production data
* terminate infrastructure
* financial transfers

Multiple approvals may be required.

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
