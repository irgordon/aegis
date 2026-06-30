# AEGIS v0.4.0 Release Readiness Review

## Executive Summary

Review date: 2026-06-30

Recommendation: Ready after small fixes.

AEGIS satisfies the implemented capability requirements for the `v0.4.0` Minimum Usable Local Release. The executable release validation gate passes on a clean worktree, the local gateway paths execute, the desktop shell launches, fixed live `health.check` evidence is available through read-only IPC, sample fallback evidence is clearly labeled, and the visual readability review now has screenshot evidence.

The repository is not ready to tag until the final release work is complete: add the explicit `v0.4.0` release notes entry and obtain maintainer approval to tag. No runtime, UI authority, architecture, validation, or visual clarity blocker was found.

## Repository Maturity

| Area | Assessment | Reasoning |
| --- | --- | --- |
| Governance | PASS | Release governance, documentation stability, and current phase rules are documented. |
| Architecture | PASS | Gateway, policy, wrapper, audit, state, recovery, and UI authority boundaries are documented and tested. |
| Implementation | PASS | Local governed execution, evidence generation, read-only recovery inspection/planning, and desktop evidence rendering exist. |
| Testing | PASS | Unit, integration, UI scaffold, desktop crate, and release-gate validations pass. |
| Documentation | PASS | README, roadmap, phasemap, release path, checklist, wiki, and UI evidence contract describe current behavior after concise drift fixes. |
| Production readiness | NOT APPLICABLE | `v0.4.0` is explicitly local-only, pre-alpha, and not production-ready. |

## Capability Review

| Capability | Status | Evidence |
| --- | --- | --- |
| Desktop application launches | PASS | Release gate phase 9 reached desktop runtime and terminated cleanly. |
| Static GUI exists | PASS | `src-tauri/ui/main.slint` renders status cards, timeline, normalized error, and recovery sample cards. |
| Live backend `health.check` evidence | PASS | `get_health_check_evidence` uses fixed request and fixed local policy bundle; UI tests cover live/sample distinction. |
| Local gateway execution | PASS | Release gate phases 5 and 6 execute gateway smoke tests. |
| Policy bundle verification | PASS | Release gate and policy bundle tests verify local bundle structure, checksums, and signature. |
| Policy evaluation | PASS | Local policy/risk matrix tests cover allow, deny, pending, malformed, ambiguous, and unsupported paths. |
| Execution authorization | PASS | Authorization evidence is required before wrapper execution and is tested. |
| Credential boundary | PASS | Credential class checks are enforced and tested. |
| Credential injection boundary | PASS | `sandbox.note.write` receives a safe local handle reference; `health.check` receives none. |
| Wrapper execution | PASS | Built-in wrappers execute only through governed dispatch. |
| `health.check` wrapper | PASS | L0 read-only wrapper executes under policy allow. |
| `sandbox.note.write` wrapper | PASS | L1 sandbox mutation writes only under supplied sandbox path with idempotency and credential gates. |
| Audit JSONL | PASS | Release gate verifies audit log creation during sandbox smoke test. |
| State JSONL | PASS | Release gate verifies state log creation during sandbox smoke test. |
| Recovery inspection | PASS | Release gate phase 7 runs `--inspect-state` successfully. |
| Recovery planning | PASS | Release gate phase 8 runs `--plan-recovery` successfully. |
| Structured errors | PASS | Error reporting tests cover request, policy, wrapper, audit, runtime, and UI-safe normalized errors. |
| Desktop UI | PASS | UI renders backend-driven fixed live evidence and labeled sample fallback evidence without authority controls; screenshot review passed in `docs/assets/release/v0.4.0-desktop-readability-review.md`. |
| Documentation | PASS | Release docs are aligned after review updates. |
| Validation script | PASS | `scripts/validate-v0.4.0-release.sh` passed and is covered by lightweight structural tests. |

## Validation Results

Validation command:

```bash
bash scripts/validate-v0.4.0-release.sh
```

Result: PASS.

The gate passed all ten phases:

1. Repository validation
2. Rust workspace validation
3. Desktop validation
4. Desktop UI validation
5. Gateway health-check smoke test
6. Sandbox mutation smoke test
7. Recovery inspection
8. Recovery planning
9. Desktop launch check
10. Repository cleanliness

The desktop launch phase reached runtime and terminated cleanly. Temporary sandbox, audit, state, and command output artifacts were created under a temporary directory and removed by the validation script.

Additional validation:

```bash
python3 scripts/verify.py
git diff --check
git status --short --branch
```

Result: PASS.

## Documentation Review

| Document | Status | Notes |
| --- | --- | --- |
| `README.md` | PASS | Short 4MAT orientation, pre-alpha warning, points to `docs/`, no production claim. |
| `docs/ROADMAP.md` | PASS | Updated to remove stale "live UI evidence" exclusion and clarify next release steps. |
| `docs/PHASEMAP.md` | PASS | Updated to keep broader audit/state/recovery views out of the `v0.4.0` release blocker set. |
| `docs/RELEASE_PATH.md` | PASS | Describes local-only scope, release gate, explicit deferrals, and next pre-tag work. |
| `docs/RELEASE_CHECKLIST_v0.4.0.md` | PASS | Matches the executable release gate and avoids production readiness claims. |
| `docs/UI_EVIDENCE_CONTRACT.md` | PASS | Updated stale "future UI" wording; preserves UI authority boundary. |
| `docs/wiki/README.md` | PASS | Explains wiki role and points to authoritative documents. |
| `docs/TASKS.md` | PASS | Updated readiness review and next release task tracking. |
| `CHANGELOG.md` | PASS | Review entry added as `0.2.36`. |

No production or enterprise readiness overstatement was found.

## Release Blockers

| Priority | Blocker | Status | Required action |
| --- | --- | --- | --- |
| P0 | Runtime, validation, or UI authority blocker | None found | None. |
| P1 | Visual polish and readability review against the v0.4.0 UI design guidance | Resolved | Screenshot evidence and review notes are recorded in `docs/assets/release/v0.4.0-desktop-readability-review.md`. |
| P1 | Explicit `v0.4.0` release notes are not yet recorded | Open | Add a `v0.4.0` changelog/release entry before tagging. |
| P1 | Maintainer approval to tag is not recorded | Open | Obtain explicit maintainer approval before creating the tag. |
| P2 | Broader read-only audit/state/recovery UI views remain planned | Deferred | Not a `v0.4.0` blocker under the minimum usable local release scope. |

## Risks

| Area | Risk | Mitigation |
| --- | --- | --- |
| Security | Users could mistake local credential handles for production credentials. | Documentation and UI label them as local safe references only. |
| Usability | Sample fallback evidence could be mistaken for live data. | UI labels sample evidence explicitly and tests enforce the distinction. |
| Documentation | Release scope could expand before tagging. | Release governance requires every task to satisfy a `v0.4.0` criterion. |
| Maintainability | Release validation is broad and can take time. | It is intentionally a release gate, not a per-edit quick check. |
| Release process | Source-only launch may limit first-user experience. | Packaging and installers are explicitly deferred beyond `v0.4.0`. |

## Recommendation

Ready after small fixes.

Evidence:

- The executable release gate passed.
- The local gateway and desktop shell satisfy the documented minimum usable local release capabilities.
- No P0 blocker was found.
- Remaining P1 items are bounded release work: write the explicit `v0.4.0` release notes entry and obtain maintainer tagging approval.

## Required Work Before Tag

Before tagging `v0.4.0`:

1. Add the explicit `v0.4.0` release notes/changelog entry.
2. Confirm maintainer approval to tag.
3. Rerun `bash scripts/validate-v0.4.0-release.sh` on a clean worktree.
4. Confirm `git status --short --branch` is clean.
5. Tag only after maintainer approval is explicit.

## Appendix

Review compared:

- implementation under `src/`, `src-tauri/`, `tests/`, `schemas/`, `examples/`, and `scripts/`
- `docs/RELEASE_PATH.md`
- `docs/RELEASE_CHECKLIST_v0.4.0.md`
- `scripts/validate-v0.4.0-release.sh`

Manual review confirmed:

- desktop launch path reaches runtime
- fixed live `health.check` evidence path exists
- `sandbox.note.write` is CLI-only and not exposed through UI IPC
- audit and state evidence are produced by the gateway smoke path
- recovery inspection and planning are read-only
- UI labels sample recovery evidence
- UI states backend authority
- UI does not expose replay, recovery execution, approval, credential issuance, arbitrary gateway execution, or mutation wrapper controls
- screenshot-backed visual readability review passed after reducing visible text density
