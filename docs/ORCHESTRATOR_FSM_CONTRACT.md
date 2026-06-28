# AEGIS
# Orchestrator FSM Contract v1.0

## Purpose

This document defines the finite-state-machine contract that orchestrators must follow when integrating with AEGIS.

AEGIS governs execution. Orchestrators plan and sequence work. The FSM contract prevents orchestrators from treating pending, denied, failed, or replay states as informal text responses.

## Scope

This contract applies to any orchestrator, adapter, or agent framework that submits tool execution requests to AEGIS.

## Orchestrator Responsibilities

The orchestrator may:

- plan work
- choose a tool to request
- submit a tool request to the gateway
- wait for pending approval completion
- continue after an allowed successful response
- stop or re-plan after denied or failed responses

The orchestrator must not:

- bypass the gateway
- self-authorize execution
- hold production credentials
- reinterpret approval scope
- execute denied actions directly
- call the LLM planning layer during AEGIS replay

## Gateway Response States

The gateway response state must be one of:

- allowed
- denied
- pending
- failed
- canceled
- replayed

Implementations may map these states to protocol-specific names only if the mapping is documented and lossless.

## State Handling Rules

### Allowed

The requested action was authorized and completed or accepted according to the tool contract.

The orchestrator may continue planning with the returned result.

### Denied

The requested action was not authorized.

The orchestrator must not execute the action through another path. It may stop, report the denial, or request a different action that goes through AEGIS.

### Pending

The requested action requires human approval or another asynchronous governance step.

The orchestrator must persist or track the pending reference and wait for completion. It must not resubmit a modified request as if it were the same approved action.

### Failed

The governed execution path failed.

The orchestrator may retry only if the response declares retry safety or if a new request is submitted through normal gateway validation.

### Canceled

The action was canceled before execution completed.

The orchestrator must treat the action as terminal unless a new request is submitted.

### Replayed

The action result came from a governed replay path.

The orchestrator must not treat replay as permission to re-plan or mutate the original request.

## Pending Approval Flow

When AEGIS returns pending:

1. The orchestrator records the pending execution reference.
2. The action waits for approval outside the planning loop.
3. AEGIS verifies the approval decision.
4. AEGIS resumes or denies using stored state.
5. The orchestrator receives a terminal response.

The orchestrator must not synthesize approval.

## Idempotency and Retries

Retries must preserve execution safety.

A retry must include the idempotency, execution, or action reference required by the gateway contract. Duplicate execution must be prevented for non-idempotent approved actions.

## Replay Contract

Replay is controlled by AEGIS, not the orchestrator planner.

Replay must use stored intent and preserved state. The orchestrator must not call the LLM to regenerate tool parameters for the replayed action.

## Validation

Integration tests for orchestrators must cover:

- allowed response handling
- denied response handling
- pending response handling
- failed response handling
- duplicate request handling
- replay without re-planning
- no direct credential use
- no gateway bypass
