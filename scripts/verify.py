#!/usr/bin/env python3
"""Repository verification for AEGIS governance and protocol contracts."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
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


def main() -> int:
    failures: list[str] = []
    checks = [
        check_required_structure,
        check_required_documents,
        check_markdown_links,
        check_blocked_markers,
        check_documentation_hierarchy,
        check_roadmap_phasemap_consistency,
        check_tasks_consistency,
        check_schema_files,
        check_schema_examples,
        check_policy_bundle_integrity,
    ]
    for check in checks:
        failures.extend(check())

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}")
        return 1

    print("AEGIS repository verification passed.")
    return 0


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
    failures = []
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


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        raise ValueError(f"invalid JSON in {relative(path)}: {error}") from error


def relative(path: Path) -> str:
    return str(path.resolve().relative_to(ROOT))


if __name__ == "__main__":
    sys.exit(main())
