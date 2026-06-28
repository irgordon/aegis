# AEGIS
# Architecture Decision Records v1.0

## Purpose

This document defines how AEGIS records architecture decisions.

Architecture decisions must be explicit because AEGIS relies on documentation-driven engineering, deterministic governance, and auditable implementation choices.

## ADR Principle

Decisions that affect architecture, invariants, security posture, policy semantics, runtime evidence, compatibility, or deployment safety must be recorded before or with the change that depends on them.

## When an ADR Is Required

An ADR is required when a change affects:

- trust boundaries
- gateway responsibilities
- policy evaluation semantics
- wrapper enforcement semantics
- credential handling
- audit evidence
- replay behavior
- schema compatibility
- policy distribution
- deployment model
- major dependency choices

## ADR Format

Each ADR should include:

- title
- status
- date
- context
- decision
- consequences
- alternatives considered
- affected documents

## Status Values

ADR status values are:

- proposed
- accepted
- superseded
- rejected

Status values must not be invented without updating this document.

## Storage

Future ADR entries should be stored in a stable location under repository governance.

Until a separate ADR directory is created, this document defines the required ADR format and process.

## Decision Review

ADR review must verify:

- the decision does not contradict `OPERATING_DOCTRINE.md`
- the decision preserves `INVARIANTS.md`
- affected architecture and acceptance criteria are updated
- security and audit implications are understood
- migration guidance exists for breaking changes

## Initial Decision Register

No implementation ADRs have been accepted yet.

The initial governance decision is that AEGIS follows Documentation-Driven Engineering and implementation must not begin until the Phase 0 governance baseline is complete.
