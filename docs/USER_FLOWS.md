# AEGIS
# User Flows v1.0

## Purpose

This document defines the primary user and system flows for AEGIS.

A user flow describes how a person, agent, or system moves through AEGIS to complete an action. These flows are product behavior contracts. Implementation may vary, but the observable behavior must remain consistent with this document.

## Scope

This document covers:

- autonomous low-risk execution
- denied execution
- human approval
- deterministic resume
- policy bundle activation
- audit review
- tool onboarding
- failure handling

## Actors

### AI Orchestrator
Plans work and submits tool execution requests.

### AEGIS Gateway
Receives tool requests, coordinates policy decisions, invokes wrappers, and returns execution responses.

### Policy Engine
Evaluates requests against the active policy bundle.

### Security Wrapper
Enforces credential, permission, and approval constraints.

### Human Approver
Reviews high-risk actions and approves or denies them.

### Security Reviewer
Reviews policy, audit records, and evidence after execution.

### Platform Operator
Deploys gateways and policy bundles.

## Flow 1: Low-Risk Allowed Execution

### Goal
Allow an AI agent to perform a low-risk action without human approval.

### Example
An agent reads metrics or creates a draft in a sandbox environment.

### Steps
1. The orchestrator creates a tool request.
2. The orchestrator sends the request to AEGIS.
3. AEGIS validates the schema.
4. AEGIS assigns or verifies execution identity.
5. The policy engine classifies the tool as L0 or L1.
6. The policy engine returns allow.
7. Wrappers apply scoped execution controls.
8. The tool executes.
9. AEGIS records audit evidence.
10. AEGIS returns success to the orchestrator.

### Expected Result
The action executes and the orchestrator may continue planning.

## Flow 2: Policy Denied Execution

### Goal
Stop an action that violates policy.

### Example
An agent attempts to call an unregistered production tool.

### Steps
1. The orchestrator submits a tool request.
2. AEGIS validates the request.
3. The policy engine evaluates the request.
4. The policy engine returns deny.
5. AEGIS records denial evidence.
6. AEGIS returns denied to the orchestrator.
7. The orchestrator terminates the action path.

### Expected Result
The requested action does not execute.

### Required Behavior
Denied actions must not partially execute.

## Flow 3: Human Approval Required

### Goal
Pause high-risk execution until a human approves or denies the action.

### Example
An agent wants to send an external email, update production infrastructure, or modify a system of record.

### Steps
1. The orchestrator submits a tool request.
2. AEGIS validates the request.
3. The policy engine classifies the action as L2 or L3.
4. AEGIS creates an approval request.
5. AEGIS records pending state.
6. AEGIS returns pending to the orchestrator.
7. The orchestrator persists state and stops active execution.
8. A human approver reviews the action.
9. The approver approves or denies the request.

### Expected Result
Execution remains paused until a valid approval decision exists.

### Required Behavior
The orchestrator must not wait in a blocking loop.

## Flow 4: Approved Resume

### Goal
Resume a previously paused action without re-planning.

### Steps
1. A valid approval event is received.
2. AEGIS verifies the approval binding.
3. The orchestrator reloads durable state.
4. The orchestrator replays the stored tool request exactly as originally submitted.
5. AEGIS verifies execution identity and replay token.
6. AEGIS executes the approved action.
7. AEGIS records execution evidence.
8. AEGIS returns success or failure.

### Expected Result
The approved action executes once.

### Required Behavior
The orchestrator must not ask the LLM to regenerate the action before replay.

## Flow 5: Approval Denied

### Goal
Terminate an action when the human approver denies it.

### Steps
1. The approver denies the approval request.
2. AEGIS records denial evidence.
3. The orchestrator receives or observes denied status.
4. The orchestrator marks the action path denied.
5. No execution occurs.

### Expected Result
The run stops or follows a documented denial branch.

## Flow 6: Duplicate Replay Attempt

### Goal
Prevent duplicate execution of an approved action.

### Steps
1. A request is replayed after execution has already occurred.
2. AEGIS checks execution identity and idempotency state.
3. AEGIS detects duplicate execution.
4. AEGIS denies or returns the prior terminal result according to policy.
5. AEGIS records duplicate replay evidence.

### Expected Result
The external action is not executed twice.

## Flow 7: Policy Bundle Activation

### Goal
Deploy a new policy bundle safely.

### Steps
1. Policy is authored in the policy repository.
2. Policy is reviewed and tested.
3. CI packages the policy into a signed immutable bundle.
4. Platform deployment delivers the bundle to gateways.
5. Gateway verifies signature, hash, schema, and compatibility.
6. Gateway activates the bundle at startup or controlled reload.
7. Audit records include policy provenance.

### Expected Result
New runs use the new bundle. Existing pinned runs retain their original policy unless migration is explicitly defined.

## Flow 8: Invalid Policy Bundle

### Goal
Fail closed when policy integrity cannot be proven.

### Steps
1. Gateway attempts to load a policy bundle.
2. Signature, hash, schema, or compatibility validation fails.
3. Gateway refuses activation.
4. Gateway emits operational error evidence.
5. Gateway denies affected execution paths.

### Expected Result
No action is authorized by an invalid policy bundle.

## Flow 9: Audit Review

### Goal
Allow a reviewer to reconstruct what happened.

### Steps
1. Reviewer searches by run ID, execution ID, action ID, or tool name.
2. Reviewer retrieves audit records.
3. Reviewer reviews decision, policy provenance, actor identity, and result.
4. Reviewer confirms whether execution matched policy.

### Expected Result
A reviewer can answer: who requested the action, what was requested, which policy applied, why it was allowed or denied, who approved it, and what happened.

## Flow 10: Tool Onboarding

### Goal
Add a new tool safely.

### Steps
1. Developer defines the tool contract.
2. Security classifies the tool capability level.
3. Policy rules are added.
4. Wrapper requirements are defined.
5. Tests are added.
6. Documentation is updated.
7. Tool is enabled through policy deployment.

### Expected Result
No tool becomes executable without classification, policy, tests, and documentation.

## Flow 11: Malformed Request

### Goal
Reject invalid or incomplete tool calls.

### Steps
1. Orchestrator sends malformed request.
2. AEGIS schema validation fails.
3. AEGIS records validation failure.
4. AEGIS returns denied or invalid request response.
5. No policy or execution side effect occurs.

### Expected Result
Malformed requests never execute.

## Flow 12: Wrapper Failure

### Goal
Fail closed when enforcement fails.

### Steps
1. Policy allows or routes an action.
2. Required wrapper fails.
3. AEGIS halts execution.
4. AEGIS records wrapper failure.
5. AEGIS returns denied or failed status.

### Expected Result
A wrapper failure never becomes permission to execute.

## Flow 13: Credential Injection

### Goal
Use credentials without exposing them to the agent.

### Steps
1. Policy allows execution.
2. Wrapper requests scoped credential.
3. Credential is injected only into the execution environment.
4. Tool executes.
5. Credential is removed or expires.
6. Audit evidence records credential scope, not secret value.

### Expected Result
The agent never receives long-lived credentials.

## Flow 14: Emergency Read-Only Mode

### Goal
Permit narrowly scoped inspection during degraded operation if explicitly configured.

### Steps
1. Gateway detects degraded state.
2. Gateway rejects mutation-capable actions.
3. Gateway permits only documented emergency read-only operations.
4. Gateway records degraded-mode audit evidence.

### Expected Result
AEGIS remains safe during failure.

## Flow 15: Operator Reviews Execution Result

### Goal
Show a graphical summary of a completed execution.

### Steps
1. Operator opens the AEGIS desktop UI.
2. Operator sees recent executions as status cards.
3. Operator selects an execution.
4. UI shows an execution timeline.
5. UI shows request status, policy result, authorization result, credential status, wrapper result, audit status, and state status.
6. Operator expands evidence only when needed.

### Expected Result
The operator can understand what happened without reading raw logs first.

### Required Behavior
The UI consumes backend evidence. It does not recreate policy decisions or execution state.

## Flow 16: Operator Reviews Failed-Closed Execution

### Goal
Explain why AEGIS blocked or stopped execution.

### Steps
1. Operator opens the failed execution.
2. UI shows a failed-closed status badge.
3. UI shows the stage where execution stopped.
4. UI shows the structured error message, reason, next action, severity, and location.
5. Operator expands supporting evidence if needed.

### Expected Result
The operator can answer what happened, why it happened, what to do next, and where the failure occurred.

### Required Behavior
The UI must keep errors bounded, plain-language, structured, secret-safe, and actionable.

## Flow 17: Operator Reviews Recovery Inspection

### Goal
Show what AEGIS found when inspecting local execution state evidence.

### Steps
1. Operator opens recovery inspection for an execution or state log.
2. UI shows inspection status.
3. UI shows last known state, transition count, terminal status, and recoverability status.
4. UI shows safe inspection errors when evidence is malformed or inconsistent.
5. Operator expands lifecycle evidence only when needed.

### Expected Result
The operator can see where execution stopped and whether the evidence is trustworthy.

### Required Behavior
Inspection is read-only. The UI must not repair state, resume execution, or hide corrupted evidence.

## Flow 18: Operator Reviews Recovery Plan

### Goal
Show what a future recovery engine may be allowed to consider.

### Steps
1. Operator opens a recovery plan.
2. UI shows recovery plan status.
3. UI shows each execution as a bounded plan outcome.
4. UI shows allowed future action as a classification, not a command.
5. UI shows normalized planning errors when evidence cannot be trusted.

### Expected Result
The operator can distinguish terminal, corrupted, audit-retry candidate, and future-replay-evaluation candidate evidence.

### Required Behavior
Recovery plans do not authorize replay, resume execution, write audit records, write state records, or execute wrappers.

## Universal Flow Requirements

Every flow must preserve:

- deny by default
- deterministic execution
- durable state
- policy provenance
- audit evidence
- zero-trust credential handling
- non-bypassable wrappers
- exact execution identity

## Final Rule

If a user flow requires trusting the AI agent to enforce security, the flow is invalid.

AEGIS governs execution outside the model.
