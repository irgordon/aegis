# AEGIS
# Release Process v1.0

## Purpose

This document defines the release process for AEGIS.

A release is acceptable only when it is versioned, reproducible, documented, auditable, and validated against the applicable governance requirements.

## Release Principles

AEGIS releases must preserve:

- security
- determinism
- correctness
- architectural integrity
- policy integrity
- auditability
- compatibility where practical

Developer convenience does not override these priorities.

## Release Inputs

A release candidate should identify:

- version
- scope
- included changes
- schema compatibility
- policy compatibility
- migration notes
- known limitations
- validation results
- security review status

## Pre-Release Checklist

Before release:

- documentation is current
- TASKS.md reflects completed work
- tests pass for implemented components
- schemas validate where schemas are active
- policy compatibility is documented where policy exists
- security implications are reviewed
- audit implications are reviewed
- known limitations are documented
- release artifacts are reproducible

## Versioning

Version milestones communicate maturity.

A version is valid only when its documented exit criteria are satisfied in `PHASEMAP.md` and relevant roadmap scope is complete.

## Compatibility

Breaking changes require:

- documented justification
- migration strategy
- semantic version impact
- updated documentation
- updated tests
- compatibility notes

Unknown compatibility must fail closed for runtime activation and deployment decisions.

## Release Evidence

Release evidence should include:

- validation command output or references
- changelog or change summary
- affected governance documents
- schema compatibility status
- policy compatibility status
- security review notes
- unresolved risks

## Rollback

Rollback guidance must identify:

- previous known-good version
- policy bundle compatibility
- schema compatibility
- state compatibility
- audit preservation requirements

Rollback must not erase audit evidence.

## Release Approval

A release is not approved solely because CI passes.

Release approval requires governance, validation, security, and audit readiness appropriate to the phase being released.
