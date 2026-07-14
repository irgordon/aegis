#!/usr/bin/env python3
"""Repository verification for AEGIS governance and protocol contracts."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path
from collections.abc import Callable
from typing import Any

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_DIRS = [
    "docs",
    "config",
    "schemas",
    "src",
    "tests",
    "examples",
    "prompts",
    "invariants",
    "scripts",
]

REQUIRED_DOCS = [
    "README.md",
    "CHANGELOG.md",
    "docs/OPERATING_DOCTRINE.md",
    "docs/PRD.md",
    "docs/ARCHITECTURE.md",
    "docs/INVARIANTS.md",
    "docs/ARCHITECTURAL_PRINCIPLES.md",
    "docs/CODING_STYLE.md",
    "docs/DOCUMENTATION.md",
    "docs/USER_FLOWS.md",
    "docs/ACCEPTANCE_CRITERIA.md",
    "docs/ROADMAP.md",
    "docs/PHASEMAP.md",
    "docs/VALIDATION.md",
    "docs/TASKS.md",
    "docs/SECURITY_MODEL.md",
    "docs/THREAT_MODEL.md",
    "docs/TRUST_BOUNDARIES.md",
    "docs/POLICY_ENGINE.md",
    "docs/POLICY_DISTRIBUTION.md",
    "docs/AUDIT_LOGGING.md",
    "docs/ORCHESTRATOR_FSM_CONTRACT.md",
    "docs/API_SPEC.md",
    "docs/RUNTIME_EVIDENCE.md",
    "docs/TEST_STRATEGY.md",
    "docs/ADR.md",
    "docs/RELEASE_PROCESS.md",
    "docs/COMPATIBILITY.md",
]

REQUIRED_SCHEMAS = [
    "ToolCallRequest.schema.json",
    "ToolCallResponse.schema.json",
    "ExecutionState.schema.json",
    "ApprovalRequest.schema.json",
    "AuditRecord.schema.json",
    "PolicyBundleManifest.schema.json",
]

PHASE0_DOCS = [
    "TASKS.md",
    "SECURITY_MODEL.md",
    "THREAT_MODEL.md",
    "TRUST_BOUNDARIES.md",
    "POLICY_ENGINE.md",
    "POLICY_DISTRIBUTION.md",
    "AUDIT_LOGGING.md",
    "ORCHESTRATOR_FSM_CONTRACT.md",
    "API_SPEC.md",
    "RUNTIME_EVIDENCE.md",
    "TEST_STRATEGY.md",
    "ADR.md",
    "RELEASE_PROCESS.md",
]

BLOCKED_MARKERS = ["TODO", "TBD", "FIXME", "placeholder"]
RELEASE_TRUTH_PATH = ROOT / "config" / "release-truth.json"
TASK_STATUSES = {"planned", "in_progress", "blocked", "complete"}
RELEASE_TRUTH_DOCS = [
    "README.md",
    "docs/RELEASE_DISTRIBUTION_PLAN.md",
    "docs/RELEASE_PATH.md",
    "docs/ROADMAP.md",
    "docs/PHASEMAP.md",
    "docs/TASKS.md",
    "docs/wiki/01-overview.md",
]
def main() -> int:
    failures: list[str] = []
    checks = [
        check_required_structure,
        check_required_documents,
        check_markdown_links,
        check_blocked_markers,
        check_documentation_hierarchy,
        check_release_truth_record,
        check_release_truth_documents,
        check_release_version_alignment,
        check_ci_validation,
        check_roadmap_phasemap_consistency,
        check_tasks_consistency,
        check_schema_files,
        check_schema_examples,
        check_policy_bundle_integrity,
    ]
    for check in checks:
        failures.extend(run_check(check))

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1

    print("AEGIS repository verification passed.")
    return 0


def run_check(check: Callable[[], list[str]]) -> list[str]:
    try:
        return check()
    except (KeyError, OSError, TypeError, ValueError) as error:
        return [f"{check.__name__} could not complete: {error}"]


def check_required_structure() -> list[str]:
    failures = []
    for directory in REQUIRED_DIRS:
        if not (ROOT / directory).is_dir():
            failures.append(f"missing required directory: {directory}")
    return failures


def check_required_documents() -> list[str]:
    failures = []
    for document in REQUIRED_DOCS:
        if not (ROOT / document).is_file():
            failures.append(f"missing required document: {document}")
    return failures


def check_markdown_links() -> list[str]:
    failures = []
    for path in markdown_files():
        text = path.read_text(encoding="utf-8")
        for match in re.finditer(r"\[[^\]]+\]\(([^)]+)\)", text):
            link = match.group(1)
            if "://" in link or link.startswith("#") or link.startswith("mailto:"):
                continue
            target = link.split("#", 1)[0]
            if not target:
                continue
            link_path = (path.parent / target).resolve()
            if not link_path.exists():
                failures.append(f"broken markdown link in {relative(path)}: {link}")
    return failures


def check_blocked_markers() -> list[str]:
    failures = []
    ignored = {ROOT / "scripts" / "verify.py"}
    for path in [ROOT / "README.md", ROOT / "CHANGELOG.md", *markdown_files()]:
        if path in ignored:
            continue
        text = path.read_text(encoding="utf-8")
        for marker in BLOCKED_MARKERS:
            if re.search(rf"\b{re.escape(marker)}\b", text, re.IGNORECASE):
                failures.append(f"blocked marker {marker!r} found in {relative(path)}")
    return failures


def check_documentation_hierarchy() -> list[str]:
    doctrine = read("docs/OPERATING_DOCTRINE.md")
    expected = [
        "1. OPERATING_DOCTRINE.md",
        "2. PRD.md",
        "3. ARCHITECTURE.md",
        "4. INVARIANTS.md",
        "5. USER_FLOWS.md",
        "6. ACCEPTANCE_CRITERIA.md",
        "7. CODING_STYLE.md",
        "8. DOCUMENTATION.md",
        "9. TASKS.md",
    ]
    return [f"documentation hierarchy missing {item}" for item in expected if item not in doctrine]


def check_release_truth_record() -> list[str]:
    try:
        truth = load_release_truth()
    except ValueError as error:
        return [str(error)]
    return validate_release_truth(truth)


def validate_release_truth(truth: dict[str, Any]) -> list[str]:
    fields = [
        "schema_version",
        "latest_published_release",
        "current_development",
        "active_engineering_phase",
        "active_engineering_phase_name",
        "active_repository_priority",
        "active_repository_priority_name",
        "planned_platform_order",
    ]
    failures = required_field_failures(truth, fields, "release truth")
    if failures:
        return failures
    failures.extend(validate_release_truth_metadata(truth))
    failures.extend(validate_release_versions(truth))
    failures.extend(validate_release_phase(truth))
    failures.extend(validate_platform_order(truth))
    return failures


def validate_release_truth_metadata(truth: dict[str, Any]) -> list[str]:
    failures = []
    if truth["schema_version"] != "1.0":
        failures.append("release truth schema_version must be 1.0")
    names = [
        ("active_engineering_phase_name", truth["active_engineering_phase_name"]),
        ("active_repository_priority_name", truth["active_repository_priority_name"]),
    ]
    for field, value in names:
        if not is_non_empty_string(value):
            failures.append(f"release truth {field} must be a non-empty string")
    return failures


def validate_release_versions(truth: dict[str, Any]) -> list[str]:
    latest = truth["latest_published_release"]
    current = truth["current_development"]
    if not isinstance(latest, dict) or not isinstance(current, dict):
        return ["release truth release values must be objects"]
    latest_fields = [
        "version",
        "title",
        "platforms",
        "includes_health_check_fixture",
        "includes_conventional_cli_help",
    ]
    failures = required_field_failures(latest, latest_fields, "latest release")
    current_fields = ["version", "product_version", "title"]
    failures.extend(required_field_failures(current, current_fields, "current development"))
    if failures:
        return failures
    failures.extend(release_value_type_failures(latest, current))
    if failures:
        return failures
    if current["version"] != f"v{current['product_version']}":
        failures.append("current development version must equal v plus product_version")
    versions = [
        ("latest release", latest["version"]),
        ("current development", current["version"]),
    ]
    for path, version in versions:
        if not re.fullmatch(r"v\d+\.\d+\.\d+", version):
            failures.append(f"{path} uses invalid semantic version: {version}")
    if latest["version"] == current["version"]:
        failures.append("latest release and current development versions must differ")
    return failures


def release_value_type_failures(latest: dict[str, Any], current: dict[str, Any]) -> list[str]:
    failures = []
    strings = [
        ("latest release version", latest["version"]),
        ("latest release title", latest["title"]),
        ("current development version", current["version"]),
        ("current development product_version", current["product_version"]),
        ("current development title", current["title"]),
    ]
    for field, value in strings:
        if not is_non_empty_string(value):
            failures.append(f"{field} must be a non-empty string")
    if not is_string_list(latest["platforms"]):
        failures.append("latest release platforms must be a non-empty string array")
    for field in ["includes_health_check_fixture", "includes_conventional_cli_help"]:
        if not isinstance(latest[field], bool):
            failures.append(f"latest release {field} must be a boolean")
    return failures


def validate_release_phase(truth: dict[str, Any]) -> list[str]:
    failures = []
    phase = truth["active_engineering_phase"]
    if not isinstance(phase, int) or isinstance(phase, bool) or phase < 0:
        failures.append("active engineering phase must be a non-negative integer")
    priority = truth["active_repository_priority"]
    if not isinstance(priority, str) or not re.fullmatch(r"P[0-5]", priority):
        failures.append("active repository priority must be bounded from P0 through P5")
    return failures


def validate_platform_order(truth: dict[str, Any]) -> list[str]:
    order = truth["planned_platform_order"]
    if not is_string_list(order):
        return ["planned platform order must be a non-empty string array"]
    if len(order) != len(set(order)):
        return ["planned platform order must not contain duplicates"]
    published = truth["latest_published_release"]["platforms"]
    if order[: len(published)] != published:
        return ["planned platform order must begin with latest release platforms"]
    return []


def required_field_failures(value: dict[str, Any], fields: list[str], path: str) -> list[str]:
    return [f"{path} missing required field: {field}" for field in fields if field not in value]


def is_non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def is_string_list(value: Any) -> bool:
    return isinstance(value, list) and bool(value) and all(is_non_empty_string(item) for item in value)


def check_release_truth_documents() -> list[str]:
    try:
        truth = load_release_truth()
    except ValueError as error:
        return [f"cannot validate release-truth documents: {error}"]
    latest = truth["latest_published_release"]["version"]
    current = truth["current_development"]["version"]
    failures = []
    for path in RELEASE_TRUTH_DOCS:
        failures.extend(release_scope_failures(read(path), path, latest, current))
    failures.extend(active_state_failures(truth))
    failures.extend(check_changelog_truth(current))
    failures.extend(check_latest_release_guidance(truth))
    return failures


def active_state_failures(truth: dict[str, Any]) -> list[str]:
    phase = f"Phase {truth['active_engineering_phase']} {truth['active_engineering_phase_name']}"
    priority = f"{truth['active_repository_priority']} {truth['active_repository_priority_name']}"
    failures = []
    for path in ["docs/ROADMAP.md", "docs/PHASEMAP.md", "docs/TASKS.md"]:
        text = read(path)
        if phase not in text:
            failures.append(f"{path} missing active engineering phase: {phase}")
        if priority not in text:
            failures.append(f"{path} missing active repository priority: {priority}")
    return failures


def release_scope_failures(text: str, path: str, latest: str, current: str) -> list[str]:
    requirements = [
        ("latest published release", latest),
        ("current development target", current),
    ]
    failures = []
    for label, version in requirements:
        pattern = rf"{re.escape(label)}[^\n]*{re.escape(version)}"
        if not re.search(pattern, text, re.IGNORECASE):
            failures.append(f"{path} missing scoped release statement: {label} {version}")
    return failures


def check_changelog_truth(current: str) -> list[str]:
    changelog = read("CHANGELOG.md")
    failures = []
    if "## [Unreleased]" not in changelog:
        failures.append("CHANGELOG.md missing Unreleased section")
    if f"Release target: `{current}" not in changelog:
        failures.append(f"CHANGELOG.md missing development target {current}")
    return failures


def check_latest_release_guidance(truth: dict[str, Any]) -> list[str]:
    latest = truth["latest_published_release"]
    if latest.get("includes_health_check_fixture") is not False:
        return []
    overview = read("docs/wiki/01-overview.md")
    artifact_path = latest_artifact_readme_path(latest["version"])
    artifact = read(artifact_path)
    failures = []
    if "examples/health-check-request.json" in overview:
        failures.append("latest-release wiki path claims an unavailable request fixture")
    if latest.get("includes_conventional_cli_help") is False and "./bin/aegis-gateway --help" in overview:
        failures.append("latest-release wiki path claims unavailable conventional help")
    if "does not include request fixture files" not in artifact:
        failures.append(f"{artifact_path} does not record the missing request fixture")
    return failures


def latest_artifact_readme_path(version: str) -> str:
    return f"docs/releases/artifact-readme-{version}.md"


def check_release_version_alignment() -> list[str]:
    try:
        truth = load_release_truth()
    except ValueError as error:
        return [f"cannot validate version alignment: {error}"]
    current = truth["current_development"]
    failures = version_source_failures(current["product_version"])
    latest = truth["latest_published_release"]["version"]
    failures.extend(development_label_failures(current["version"], latest))
    failures.extend(workflow_version_failures(current["version"], latest))
    return failures


def version_source_failures(expected: str) -> list[str]:
    failures = manifest_version_failures(expected)
    failures.extend(lockfile_version_failures(expected))
    return failures


def manifest_version_failures(expected: str) -> list[str]:
    sources = {
        "Cargo.toml": load_toml(ROOT / "Cargo.toml")["package"]["version"],
        "src-tauri/Cargo.toml": load_toml(ROOT / "src-tauri/Cargo.toml")["package"]["version"],
        "src-tauri/tauri.conf.json": load_json(ROOT / "src-tauri/tauri.conf.json")["version"],
    }
    return [
        f"{path} version {actual} does not match {expected}"
        for path, actual in sources.items()
        if actual != expected
    ]


def lockfile_version_failures(expected: str) -> list[str]:
    packages = [
        ("Cargo.lock", "aegis"),
        ("src-tauri/Cargo.lock", "aegis"),
        ("src-tauri/Cargo.lock", "aegis-desktop"),
    ]
    failures = []
    for path, package in packages:
        actual = cargo_lock_package_version(path, package)
        if actual != expected:
            failures.append(f"{path} package {package} version {actual} does not match {expected}")
    return failures


def cargo_lock_package_version(path: str, package: str) -> str:
    packages = load_toml(ROOT / path).get("package", [])
    versions = [item["version"] for item in packages if item.get("name") == package]
    if len(versions) != 1:
        return f"<expected one entry, found {len(versions)}>"
    return versions[0]


def development_label_failures(current: str, latest: str) -> list[str]:
    ui = read("src-tauri/ui/main.slint")
    required = [f"Current development: {current}", f"Latest release {latest}"]
    return [f"desktop UI missing release-truth label: {label}" for label in required if label not in ui]


def workflow_version_failures(current: str, latest: str) -> list[str]:
    paths = [
        ".github/workflows/draft-artifacts.yml",
        ".github/workflows/draft-github-release.yml",
    ]
    failures = []
    for path in paths:
        text = read(path)
        if current not in text:
            failures.append(f"{path} does not target {current}")
        if latest in text:
            failures.append(f"{path} still targets immutable release {latest}")
    return failures


def check_ci_validation() -> list[str]:
    workflow = read(".github/workflows/validate.yml")
    return desktop_ci_failures(workflow)


def desktop_ci_failures(workflow: str) -> list[str]:
    required = [
        "desktop:",
        "runs-on: macos-latest",
        "cargo fmt --manifest-path src-tauri/Cargo.toml --check",
        "cargo clippy --locked --manifest-path src-tauri/Cargo.toml",
        "cargo test --locked --manifest-path src-tauri/Cargo.toml",
        "cargo check --locked --manifest-path src-tauri/Cargo.toml",
    ]
    return [f"validate workflow missing desktop CI: {item}" for item in required if item not in workflow]


def check_roadmap_phasemap_consistency() -> list[str]:
    roadmap = read("docs/ROADMAP.md")
    phasemap = read("docs/PHASEMAP.md")
    failures = []
    for item in PHASE0_DOCS:
        if item not in roadmap:
            failures.append(f"ROADMAP missing Phase 0 item: {item}")
        if item not in phasemap:
            failures.append(f"PHASEMAP missing v0.1.0 item: {item}")
    phase1_items = [
        "ToolCallRequest schema",
        "ToolCallResponse schema",
        "AuditRecord schema",
        "PolicyBundleManifest schema",
        "approval request schema",
        "execution state schema",
        "API specification",
    ]
    for item in phase1_items:
        if item not in roadmap:
            failures.append(f"ROADMAP missing Phase 1 item: {item}")
        if item not in phasemap:
            failures.append(f"PHASEMAP missing v0.2.0 item: {item}")
    return failures


def check_tasks_consistency() -> list[str]:
    tasks = read("docs/TASKS.md")
    failures = task_row_failures(tasks)
    completed = [
        "Finalize ToolCallRequest schema",
        "Finalize ToolCallResponse schema",
        "Finalize AuditRecord schema",
        "Finalize PolicyBundleManifest schema",
        "Finalize ApprovalRequest schema",
        "Finalize ExecutionState schema",
        "Add valid and invalid schema examples",
        "Add schema validation command or script",
        "Align API_SPEC.md with finalized schemas",
    ]
    for task in completed:
        row = f"| {task} | complete |"
        if row not in tasks:
            failures.append(f"TASKS.md does not mark complete: {task}")
    return failures


def task_row_failures(tasks: str) -> list[str]:
    failures = []
    seen: dict[str, str] = {}
    for task, status in parse_task_rows(tasks):
        if status not in TASK_STATUSES:
            failures.append(f"TASKS.md uses invalid status {status!r}: {task}")
            continue
        if task in seen:
            failures.append(duplicate_task_failure(task, seen[task], status))
            continue
        seen[task] = status
    return failures


def parse_task_rows(tasks: str) -> list[tuple[str, str]]:
    rows = []
    for line in tasks.splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) != 2 or cells[0] == "Task" or set(cells[0]) == {"-"}:
            continue
        if set(cells[1]) == {"-"}:
            continue
        rows.append((cells[0], cells[1]))
    return rows


def duplicate_task_failure(task: str, previous: str, current: str) -> str:
    if previous == current:
        return f"TASKS.md duplicates task status {current!r}: {task}"
    return f"TASKS.md conflicts for {task}: {previous!r} and {current!r}"


def check_schema_files() -> list[str]:
    failures = []
    for schema_name in REQUIRED_SCHEMAS:
        path = ROOT / "schemas" / schema_name
        if not path.is_file():
            failures.append(f"missing schema: schemas/{schema_name}")
            continue
        try:
            schema = load_json(path)
        except ValueError as error:
            failures.append(str(error))
            continue
        if schema.get("type") != "object":
            failures.append(f"schema root must be object: schemas/{schema_name}")
        if not schema.get("required"):
            failures.append(f"schema must define required fields: schemas/{schema_name}")
    return failures


def check_schema_examples() -> list[str]:
    failures = []
    for schema_name in REQUIRED_SCHEMAS:
        stem = schema_name.removesuffix(".schema.json")
        schema_path = ROOT / "schemas" / schema_name
        valid_path = ROOT / "schemas" / "examples" / "valid" / f"{stem}.json"
        invalid_path = ROOT / "schemas" / "examples" / "invalid" / f"{stem}.json"
        if not valid_path.is_file():
            failures.append(f"missing valid example: {relative(valid_path)}")
            continue
        if not invalid_path.is_file():
            failures.append(f"missing invalid example: {relative(invalid_path)}")
            continue
        schema = load_json(schema_path)
        valid_example = load_json(valid_path)
        invalid_example = load_json(invalid_path)
        valid_errors = validate_value(valid_example, schema, stem)
        invalid_errors = validate_value(invalid_example, schema, stem)
        if valid_errors:
            failures.append(f"valid example failed {relative(valid_path)}: {valid_errors[0]}")
        if not invalid_errors:
            failures.append(f"invalid example unexpectedly passed: {relative(invalid_path)}")
    return failures


def check_policy_bundle_integrity() -> list[str]:
    manifest_path = ROOT / "schemas" / "examples" / "valid" / "PolicyBundleManifest.json"
    if not manifest_path.is_file():
        return ["policy bundle manifest valid example is missing"]
    manifest = load_json(manifest_path)
    contents = manifest.get("contents", [])
    if not contents:
        return ["policy bundle manifest valid example has no contents"]
    failures = []
    for item in contents:
        if "path" not in item or "sha256" not in item:
            failures.append("policy bundle manifest content entry lacks path or sha256")
    return failures


def validate_value(value: Any, schema: dict[str, Any], path: str) -> list[str]:
    errors: list[str] = []
    expected_type = schema.get("type")
    if expected_type is not None and not matches_type(value, expected_type):
        errors.append(f"{path}: expected type {expected_type}, got {type(value).__name__}")
        return errors

    if "const" in schema and value != schema["const"]:
        errors.append(f"{path}: expected constant {schema['const']!r}")

    if "enum" in schema and value not in schema["enum"]:
        errors.append(f"{path}: value {value!r} not in enum")

    if isinstance(value, str):
        min_length = schema.get("minLength")
        if min_length is not None and len(value) < min_length:
            errors.append(f"{path}: string shorter than {min_length}")

    if isinstance(value, int) and not isinstance(value, bool):
        minimum = schema.get("minimum")
        if minimum is not None and value < minimum:
            errors.append(f"{path}: integer below minimum {minimum}")

    if isinstance(value, list):
        min_items = schema.get("minItems")
        if min_items is not None and len(value) < min_items:
            errors.append(f"{path}: array shorter than {min_items}")
        item_schema = schema.get("items")
        if isinstance(item_schema, dict):
            for index, item in enumerate(value):
                errors.extend(validate_value(item, item_schema, f"{path}[{index}]"))

    if isinstance(value, dict):
        required = schema.get("required", [])
        for field in required:
            if field not in value:
                errors.append(f"{path}: missing required field {field}")
        properties = schema.get("properties", {})
        for field, field_value in value.items():
            if field in properties:
                errors.extend(validate_value(field_value, properties[field], f"{path}.{field}"))
            elif schema.get("additionalProperties") is False:
                errors.append(f"{path}: unexpected field {field}")

    return errors


def matches_type(value: Any, expected_type: Any) -> bool:
    if isinstance(expected_type, list):
        return any(matches_type(value, item) for item in expected_type)
    if expected_type == "object":
        return isinstance(value, dict)
    if expected_type == "array":
        return isinstance(value, list)
    if expected_type == "string":
        return isinstance(value, str)
    if expected_type == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if expected_type == "number":
        return isinstance(value, (int, float)) and not isinstance(value, bool)
    if expected_type == "boolean":
        return isinstance(value, bool)
    if expected_type == "null":
        return value is None
    return True


def markdown_files() -> list[Path]:
    return sorted((ROOT / "docs").glob("*.md"))


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def load_release_truth() -> dict[str, Any]:
    if not RELEASE_TRUTH_PATH.is_file():
        raise ValueError("missing release truth: config/release-truth.json")
    truth = load_json(RELEASE_TRUTH_PATH)
    if not isinstance(truth, dict):
        raise ValueError("release truth root must be an object")
    return truth


def load_toml(path: Path) -> dict[str, Any]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        raise ValueError(f"invalid TOML in {relative(path)}: {error}") from error


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        raise ValueError(f"invalid JSON in {relative(path)}: {error}") from error


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(ROOT))


if __name__ == "__main__":
    sys.exit(main())
