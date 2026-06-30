#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/aegis-v0.4.0-release.XXXXXX")"

CURRENT_PHASE="initialization"
NEXT_ACTION="Review the failed command output and rerun the release validation gate."

HEALTH_OUTPUT="$TMP_ROOT/health-check.json"
SANDBOX_OUTPUT="$TMP_ROOT/sandbox-note.json"
INSPECTION_OUTPUT="$TMP_ROOT/recovery-inspection.json"
PLAN_OUTPUT="$TMP_ROOT/recovery-plan.json"
DESKTOP_LOG="$TMP_ROOT/desktop-launch.log"
AUDIT_LOG="$TMP_ROOT/audit.jsonl"
STATE_LOG="$TMP_ROOT/state.jsonl"
SANDBOX_DIR="$TMP_ROOT/sandbox"

cleanup() {
  rm -rf "$TMP_ROOT"
}

report_failure() {
  local status=$?
  echo
  echo "==================================="
  echo "AEGIS v0.4.0 Release Validation"
  echo "FAILED"
  echo "==================================="
  echo "Failed phase: $CURRENT_PHASE"
  echo "Next action: $NEXT_ACTION"
  exit "$status"
}

trap cleanup EXIT
trap report_failure ERR

cd "$ROOT_DIR"

phase() {
  CURRENT_PHASE="$1"
  NEXT_ACTION="$2"
  echo "[$3/10] $CURRENT_PHASE..."
}

pass() {
  echo "PASS"
}

fail_phase() {
  echo "ERROR: $1"
  false
}

assert_valid_json() {
  python3 - "$1" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    json.load(handle)
PY
}

assert_health_evidence() {
  python3 - "$1" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    output = json.load(handle)

checks = {
    "allowed response": output.get("response", {}).get("status") == "allowed",
    "verified policy": output.get("policy_bundle", {}).get("verification_status") == "verified",
    "health wrapper": output.get("wrapper_execution", {}).get("wrapper_name") == "health.check",
    "wrapper executed": output.get("wrapper_execution", {}).get("wrapper_status") == "executed",
    "completed lifecycle": output.get("execution_lifecycle", {}).get("execution_state") == "completed",
}

missing = [name for name, passed in checks.items() if not passed]
if missing:
    raise SystemExit(f"missing expected health-check evidence: {', '.join(missing)}")
PY
}

assert_sandbox_artifacts() {
  local note_file

  note_file="$(find "$SANDBOX_DIR/notes" -type f -name '*.txt' -print -quit 2>/dev/null || true)"
  [[ -n "$note_file" ]] || fail_phase "sandbox note was not created"
  [[ -s "$AUDIT_LOG" ]] || fail_phase "audit log was not created"
  [[ -s "$STATE_LOG" ]] || fail_phase "state log was not created"
}

run_desktop_launch_check() {
  set +e
  cargo run --manifest-path src-tauri/Cargo.toml >"$DESKTOP_LOG" 2>&1 &
  local desktop_pid=$!
  set -e

  local reached_runtime=0
  for _ in {1..45}; do
    if grep -q "Running .*aegis-desktop" "$DESKTOP_LOG"; then
      reached_runtime=1
      break
    fi

    if ! kill -0 "$desktop_pid" 2>/dev/null; then
      break
    fi

    sleep 1
  done

  if [[ "$reached_runtime" -eq 1 ]]; then
    kill "$desktop_pid" 2>/dev/null || true
    set +e
    wait "$desktop_pid" 2>/dev/null
    set -e
    echo "Desktop launch command reached runtime; terminated after validation."
    return 0
  fi

  set +e
  wait "$desktop_pid"
  local desktop_status=$?
  set -e

  if [[ "$desktop_status" -eq 0 ]]; then
    echo "Desktop launch command exited successfully."
    return 0
  fi

  if grep -qiE "cannot open display|display server|headless|no display|failed to create webview|failed to initialize gtk|failed to connect to display" "$DESKTOP_LOG"; then
    echo "Desktop launch environment does not support displaying a GUI; compile/run command reached the environment boundary."
    return 0
  fi

  echo "Desktop launch failed. Last log lines:"
  tail -n 40 "$DESKTOP_LOG"
  return "$desktop_status"
}

phase "Repository validation" "Fix repository verification errors from scripts/verify.py." 1
python3 scripts/verify.py
pass

phase "Rust workspace validation" "Fix Rust formatting, clippy warnings, or failing tests." 2
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pass

phase "Desktop validation" "Fix desktop formatting, clippy warnings, tests, or compile errors." 3
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
pass

phase "Desktop UI validation" "Fix UI scaffold tests before release validation can continue." 4
cargo test --test ui_scaffold
pass

phase "Gateway health-check smoke test" "Fix the governed health.check runtime path or local policy bundle fixture." 5
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  schemas/examples/valid/HealthCheckRequest.json >"$HEALTH_OUTPUT"
assert_valid_json "$HEALTH_OUTPUT"
assert_health_evidence "$HEALTH_OUTPUT"
pass

phase "Sandbox mutation smoke test" "Fix sandbox.note.write, audit logging, state logging, or sandbox cleanup." 6
mkdir -p "$SANDBOX_DIR"
cargo run --quiet --bin aegis-gateway -- \
  --bundle examples/policy-bundles/local-dev \
  --audit-log "$AUDIT_LOG" \
  --state-log "$STATE_LOG" \
  --sandbox-dir "$SANDBOX_DIR" \
  schemas/examples/valid/SandboxNoteWriteRequest.json >"$SANDBOX_OUTPUT"
assert_valid_json "$SANDBOX_OUTPUT"
assert_sandbox_artifacts
pass

phase "Recovery inspection" "Fix read-only recovery inspection over generated state logs." 7
cargo run --quiet --bin aegis-gateway -- --inspect-state "$STATE_LOG" >"$INSPECTION_OUTPUT"
assert_valid_json "$INSPECTION_OUTPUT"
pass

phase "Recovery planning" "Fix read-only recovery planning over generated state logs." 8
cargo run --quiet --bin aegis-gateway -- --plan-recovery "$STATE_LOG" >"$PLAN_OUTPUT"
assert_valid_json "$PLAN_OUTPUT"
pass

phase "Desktop launch check" "Fix desktop startup if a GUI is available, or investigate unexpected launch failures." 9
run_desktop_launch_check
pass

phase "Repository cleanliness" "Remove generated artifacts or uncommitted changes before tagging v0.4.0." 10
git diff --check
git status --short --branch
[[ -z "$(git status --porcelain)" ]] || fail_phase "repository has uncommitted changes"
pass

echo "==================================="
echo "AEGIS v0.4.0 Release Validation"
echo "PASS"
echo "==================================="
