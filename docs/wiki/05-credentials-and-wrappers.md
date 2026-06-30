# AEGIS
# Credentials and Wrappers

## What Is This?

This page explains how AEGIS currently handles wrapper execution and credential boundaries.

AEGIS can execute built-in local wrappers, but it does not expose real secrets.

## Core Rule

Credential classes authorize categories, not secrets.

Credential injection currently provides safe local credential handle references only. It does not retrieve, generate, store, or expose real credentials.

## Credential Class Boundary

Every wrapper must explicitly declare a credential requirement.

The credential boundary checks:

- whether the wrapper requires credentials
- which credential class the wrapper requires
- whether execution authorization permits that class

There is no implicit default credential class.

Every wrapper execution path must pass through the credential boundary, including wrappers that require no credentials.

## Local Credential Injection

The local credential injection boundary can return a safe local development credential handle reference when a wrapper requires `LocalRuntime` and the authorization allows it.

The handle is an evidence reference, not a secret.

It may appear in runtime, audit, or state evidence only as a safe reference.

It must not contain:

- passwords
- API keys
- bearer tokens
- private keys
- certificate material
- environment variables
- vault references
- secret file paths

## Current Built-In Wrappers

| Wrapper | Capability | Credential requirement | Behavior |
| --- | --- | --- | --- |
| `health.check` | L0 read-only | `None` | Returns local gateway health data |
| `sandbox.note.write` | L1 local mutation | `LocalRuntime` | Writes a note under an explicit sandbox directory |

## `health.check`

`health.check` is local, read-only, deterministic, and safe.

It does not:

- require credentials
- write files
- call a network
- spawn subprocesses
- mutate external systems

## `sandbox.note.write`

`sandbox.note.write` proves controlled local mutation.

It writes a note only under the caller-supplied sandbox root.

It requires:

- allowed policy decision
- L1 capability class
- execution authorization
- satisfied credential boundary
- local development credential handle reference
- idempotency context
- valid sandbox directory
- path containment check
- audit evidence

If any gate fails, the wrapper does not write.

## Wrapper Responsibilities

Wrappers may:

- validate their execution context
- perform the narrow action they were built for
- return bounded output
- report safe execution evidence

Wrappers must not:

- evaluate policy
- authorize themselves
- request arbitrary credential classes
- widen authority
- expose secret values
- execute shell commands in the current local wrapper paths
- bypass audit or lifecycle evidence

## Future Work

Future credential providers may eventually connect to vaults or identity systems.

That work must preserve the current boundary:

```text
authorized credential class -> safe credential handle -> wrapper execution context
```

No future provider should make wrappers responsible for finding or authorizing their own secrets.
