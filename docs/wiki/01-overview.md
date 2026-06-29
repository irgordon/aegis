# AEGIS
# Overview

## What Is This?

AEGIS is an AI execution governance gateway.

It sits between an AI system that wants to perform an action and the runtime that could carry out that action.

## Why Does It Exist?

AI systems increasingly do more than answer questions. They can ask to write files, call tools, change records, open tickets, deploy software, or interact with business systems.

That kind of execution needs a clear governance boundary.

AEGIS exists so execution is not trusted just because an AI requested it. AEGIS checks the request, verifies policy, authorizes execution, records evidence, and fails closed when something is unclear.

## Current Status

AEGIS is pre-alpha.

The repository currently contains a local Rust gateway MVP and early Governed Execution Engine work. It can validate structured requests, verify a local policy bundle, evaluate local policy, authorize governed execution, dispatch built-in local wrappers, persist audit and state evidence, inspect local execution state, and generate bounded recovery plans.

AEGIS is not ready for production use.

It does not yet provide production deployment, HTTP service behavior, production identity providers, real external system integrations, approval workflow execution, replay execution, or a completed Tauri operator UI.

## What It Does Today

At a high level, AEGIS can:

- read a structured tool request
- validate the request
- verify a local policy bundle using checksums and Ed25519 signature verification
- evaluate local policy and risk matrix data
- return a bounded decision: allow, deny, or pending approval
- authorize execution only after policy allows it
- enforce credential class and local credential handle boundaries
- dispatch safe built-in local wrappers
- record audit evidence
- record execution lifecycle transitions
- inspect local state logs
- generate bounded recovery plans
- return structured JSON output

## What It Does Not Do Yet

AEGIS does not yet:

- execute arbitrary external tools
- inject real secrets
- retrieve credentials from a vault
- run approval workflows
- replay executions
- recover executions automatically
- expose a production HTTP API
- provide a completed desktop UI
- provide production PKI or remote policy distribution

## Who Should Read This?

New readers should start here.

Contributors should read this page, then continue through the wiki before changing runtime behavior.

Engineers and architects should use this page for orientation, then rely on `docs/ARCHITECTURE.md`, `docs/INVARIANTS.md`, and `docs/TRUST_BOUNDARIES.md` for authoritative contracts.

