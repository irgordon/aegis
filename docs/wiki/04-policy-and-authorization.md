# AEGIS
# Policy and Authorization

## What Is This?

This page explains the difference between policy decisions and execution authorization.

The short version:

```text
Policy decides whether execution is allowed.
Execution authorization defines how allowed execution may proceed.
```

## Verified Policy Bundle

AEGIS evaluates local policy only after the policy bundle is verified.

The local development bundle includes:

- `manifest.yaml`
- `gateway_policy.yaml`
- `risk_matrix.yaml`
- `checksums/SHA256SUMS`
- `signatures/public.pem`
- `signatures/SHA256SUMS.sig`

The loader checks required files, version alignment, SHA-256 checksums, and Ed25519 signature verification over the checksum manifest.

This is local development signature verification. It is not production PKI or remote trust distribution.

If verification fails, AEGIS fails closed and does not evaluate policy.

## Policy Evaluation

Policy evaluation uses the verified local bundle to match a request against policy and risk data.

The current evaluator supports bounded local evaluation. It does not load remote policy, call a trust registry, or execute scripts.

Policy outputs are bounded:

- `allow`
- `deny`
- `pending_approval`

Unsupported, malformed, ambiguous, or missing policy state fails closed.

## Execution Authorization

Execution authorization is created only after policy allows execution.

It records:

- authorization identifier
- execution identity reference
- wrapper name
- wrapper version
- tool name
- capability class
- execution scope
- authority source
- authorized credential class
- expiration reference
- authorization status

Denied and pending decisions do not create execution authorization and do not dispatch wrappers.

## Authority Sources

The current runtime uses bounded authority sources.

Current usable sources:

- `PolicyAllow`
- `DevelopmentFixture` where local development context requires it

Future sources such as human approval or break-glass authority are not implemented.

## Execution Scope

Execution scope is explicit.

For example:

- `local_gateway_health`
- local sandbox note write scope used by `sandbox.note.write`

Broad scopes such as `global`, `root`, `admin`, `unrestricted`, or `all` must not be accepted.

## Why This Separation Matters

Policy evaluation answers:

> Should this request be allowed, denied, or held for approval?

Execution authorization answers:

> Under what authority may this wrapper execute this specific request?

Wrapper dispatch answers:

> Is there a registered wrapper that can execute this request under the provided authority?

Keeping these boundaries separate prevents a wrapper from becoming a hidden policy engine.

## What Is Not Implemented

AEGIS does not yet implement:

- production policy distribution
- production PKI
- remote trust registry lookup
- approval workflow execution
- break-glass execution
- wrapper self-authorization
- external system action execution beyond current built-in local wrappers
