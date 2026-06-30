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

`v0.4.0` is complete, tagged, pushed, and closed as a local-only source release.

The repository currently contains a local Rust gateway, a Tauri plus Slint desktop shell, governed built-in wrappers, audit and state evidence, recovery inspection, recovery planning, and an executable release validation gate.

It can validate structured requests, verify a local policy bundle, evaluate local policy, authorize governed execution, dispatch built-in local wrappers, optionally append local audit and state evidence, inspect local execution state, generate read-only recovery plan reports, and show fixed live health-check evidence in the desktop UI.

AEGIS is not ready for production use.

It does not provide published binaries, installers, packaging, signing, auto-update, production deployment, HTTP service behavior, production identity providers, real external system integrations, approval workflow execution, replay execution, recovery execution, or enterprise hardening.

Post-`v0.4.0` distribution planning has started. `v0.4.1` is selected as the first planned downloadable developer-preview target, starting with macOS arm64 and macOS x64 archive artifacts. No public release assets exist yet.

## What It Does Today

At a high level, AEGIS can:

- read a structured tool request
- validate the request
- verify a local policy bundle using checksums and Ed25519 signature verification
- evaluate local policy and risk matrix data
- return a bounded decision: allow, deny, or pending approval
- authorize execution only after policy allows it
- enforce the credential class boundary and local credential handle validation
- dispatch safe built-in local wrappers
- record audit evidence
- record execution lifecycle transitions
- inspect local state logs
- generate read-only bounded recovery plan reports
- launch a local desktop UI that renders fixed live `health.check` evidence and labeled sample evidence
- run the v0.4.0 release validation gate
- return structured JSON output

## What It Does Not Do Yet

AEGIS does not yet:

- publish downloadable release assets
- provide installers or packaged apps
- sign or notarize artifacts
- execute arbitrary external tools
- inject real secrets
- retrieve credentials from a vault
- run approval workflows
- replay executions
- recover executions automatically
- expose a production HTTP API
- let the UI submit arbitrary gateway requests
- provide production PKI or remote policy distribution

## Who Should Read This?

New readers should start here.

Contributors should read this page, then continue through the wiki before changing runtime behavior.

Engineers and architects should use this page for orientation, then rely on `docs/ARCHITECTURE.md`, `docs/INVARIANTS.md`, and `docs/TRUST_BOUNDARIES.md` for authoritative contracts.
