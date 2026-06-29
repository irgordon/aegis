# AGENTS.md

# AEGIS Agent Instructions

AEGIS is documentation-driven. Do not guess intent from code or invent future behavior.

## Read First

Before changing files, read:

1. `docs/OPERATING_DOCTRINE.md`
2. `docs/INVARIANTS.md`
3. `docs/CODING_STYLE.md`
4. `docs/ROADMAP.md`
5. `docs/PHASEMAP.md`
6. `docs/TASKS.md`
7. `docs/ACCEPTANCE_CRITERIA.md`

For runtime or API work, also read:

* `docs/API_SPEC.md`
* `docs/ORCHESTRATOR_FSM_CONTRACT.md`
* `docs/COMPATIBILITY.md`
* `schemas/`

For UI or frontend work, also read:

* `docs/UI-DESIGN.md`

For UI changes, read `docs/UI-DESIGN.md` and complete the UI Integrity Review before implementing.

## Do Not

* Do not create code outside the current phase.
* Do not create dead code, unused modules, or speculative scaffolding.
* Do not weaken invariants, validation, schemas, or GitHub Actions.
* Do not add runtime behavior not described in the docs.
* Do not log secrets.
* Do not silently ignore failed validation.
* Do not update README.md for ordinary implementation progress; README.md changes only for production release or major architectural identity changes.
* Do not update OPERATING_DOCTRINE.md, ARCHITECTURE.md, or CODING_STYLE.md for routine implementation progress.

## Documentation Stability

Documentation should become progressively more stable as AEGIS matures.

Use `CHANGELOG.md` as the primary record of routine implementation progress.

Update stable governance documents only when the work genuinely changes their scope:

* `OPERATING_DOCTRINE.md`: almost never; only when repository governance changes.
* `ARCHITECTURE.md`: only when architecture changes.
* `CODING_STYLE.md`: only when engineering philosophy changes.
* `README.md`: only for production release or major architectural identity change.

## Current Phase Rule

Check `docs/PHASEMAP.md` and `docs/TASKS.md`.

Only implement work assigned to the current phase.

e.g. If the task is Phase 2, do not implement Phase 3 or later behavior.

## Validation

Before finishing, run the applicable checks:

```bash
python3 scripts/verify.py
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git status --short
```

If a command is not applicable, state why.

## Completion Report

Report:

* files changed
* validation commands run
* results
* assumptions
* risks
* follow-up tasks

## Final Rule

When documentation and code disagree, stop. If unclear, stop. Do not guess. Resolve the documentation gap first.
