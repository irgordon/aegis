# AEGIS
# Compatibility v1.0

## Purpose

This document defines compatibility expectations for AEGIS gateway runtimes, schemas, policy bundles, wrappers, APIs, and release versions.

Compatibility is a safety boundary. Unknown compatibility must fail closed when it affects authorization, execution, replay, audit evidence, or policy activation.

## Gateway Compatibility

Gateway compatibility describes whether a gateway runtime can safely interpret the active schemas, policy bundle, wrapper contracts, state records, and API semantics.

A gateway must verify compatibility before serving governed execution traffic.

Gateway compatibility metadata should include:

- gateway version
- supported schema versions
- supported API version
- supported policy bundle manifest version
- supported wrapper contract versions
- supported execution state version

If the gateway cannot verify compatibility, it must not execute governed actions.

## Schema Compatibility

Schemas are executable protocol contracts.

Schema compatibility must preserve:

- required field meaning
- bounded enum values
- status semantics
- policy provenance fields
- approval binding fields
- replay fields
- audit evidence fields

Backward-compatible changes may add optional fields that do not change existing meaning.

Breaking changes include:

- removing required fields
- changing status or decision semantics
- weakening validation
- changing approval binding meaning
- changing replay identity meaning
- changing policy provenance meaning

Breaking schema changes require a new schema version, migration guidance, updated examples, and updated validation.

## Policy Compatibility

Policy bundle compatibility defines whether a gateway can safely activate and enforce a policy bundle.

A policy bundle manifest must declare:

- bundle ID
- policy version
- policy hash
- target environment
- compatible gateway versions
- compatible schema versions
- compatible wrapper versions
- compatible API version

Invalid, unsupported, unsigned where signing is required, or incompatible bundles must not activate.

Existing runs remain pinned to their active policy bundle unless a documented migration protocol explicitly changes that binding.

## Wrapper Compatibility

Wrapper compatibility defines whether enforcement wrappers can safely enforce gateway and policy decisions.

Wrapper compatibility metadata should identify:

- wrapper name
- wrapper version
- supported decision values
- supported credential injection behavior
- supported approval verification behavior
- supported audit evidence fields

A wrapper must not broaden authorization beyond the policy decision.

Unknown or incompatible wrappers fail closed.

## API Compatibility

API compatibility defines whether clients and gateways interpret request and response semantics consistently.

API compatibility must preserve:

- ToolCallRequest semantics
- ToolCallResponse status values
- denial behavior
- pending approval behavior
- replay behavior
- error semantics
- policy provenance
- audit references

Transport-specific adapters may map field names only when the mapping is documented and lossless.

## Semantic Versioning Expectations

AEGIS versions communicate maturity and compatibility.

Version changes should follow these expectations:

- Patch versions preserve documented behavior and compatibility.
- Minor versions may add backward-compatible fields, documents, validation, or capabilities.
- Major versions may introduce breaking contract changes with documented migration guidance.

Pre-1.0 versions may evolve quickly, but breaking changes still require documentation, task updates, schema updates, examples, and validation evidence.

## Product Version Identity

AEGIS uses one release-facing product version for Git tags, GitHub Releases, artifacts, gateway and desktop Cargo metadata, Tauri metadata, and published changelog headings.

Engineering phases are not version namespaces. They describe maturity and may span multiple product releases.

The current development product version may be newer than the latest published release. User-facing statements must identify which one they describe.

Legacy `0.2.x` changelog headings record internal repository iterations and do not represent published releases.

## Compatibility Evidence

Compatibility-sensitive changes must update the relevant evidence:

- schema files
- valid and invalid examples
- API specification
- policy bundle manifest schema
- TASKS.md
- CHANGELOG.md
- validation script

A release is not compatible merely because files parse. Compatibility must be explicit, documented, and validated.
