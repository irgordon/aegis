import copy
import json
import unittest
from pathlib import Path

from scripts import verify


class TaskStatusTests(unittest.TestCase):
    def test_unique_bounded_task_rows_pass(self) -> None:
        tasks = "| Task | Status |\n| --- | --- |\n| First | planned |\n| Second | complete |"

        self.assertEqual(verify.task_row_failures(tasks), [])

    def test_duplicate_task_row_fails(self) -> None:
        tasks = "| Task | Status |\n| --- | --- |\n| First | planned |\n| First | planned |"

        self.assertIn("duplicates task status", verify.task_row_failures(tasks)[0])

    def test_conflicting_task_row_fails(self) -> None:
        tasks = "| Task | Status |\n| --- | --- |\n| First | planned |\n| First | complete |"

        self.assertIn("conflicts for First", verify.task_row_failures(tasks)[0])

    def test_unbounded_task_status_fails(self) -> None:
        tasks = "| Task | Status |\n| --- | --- |\n| First | waiting |"

        self.assertIn("invalid status", verify.task_row_failures(tasks)[0])


class ReleaseTruthTests(unittest.TestCase):
    def setUp(self) -> None:
        path = Path("config/release-truth.json")
        self.truth = json.loads(path.read_text(encoding="utf-8"))

    def test_current_release_truth_passes(self) -> None:
        self.assertEqual(verify.validate_release_truth(self.truth), [])

    def test_missing_required_field_fails(self) -> None:
        truth = copy.deepcopy(self.truth)
        del truth["current_development"]

        self.assertIn("current_development", verify.validate_release_truth(truth)[0])

    def test_product_version_mismatch_fails(self) -> None:
        truth = copy.deepcopy(self.truth)
        truth["current_development"]["product_version"] = "0.4.3"

        failures = verify.validate_release_truth(truth)
        self.assertIn("must equal v plus product_version", failures[0])

    def test_platform_order_mismatch_fails(self) -> None:
        truth = copy.deepcopy(self.truth)
        truth["planned_platform_order"].reverse()

        self.assertIn("platform order", verify.validate_release_truth(truth)[0])

    def test_invalid_priority_type_fails_without_exception(self) -> None:
        truth = copy.deepcopy(self.truth)
        truth["active_repository_priority"] = ["P0"]

        failures = verify.validate_release_truth(truth)
        self.assertIn("bounded from P0 through P5", failures[0])

    def test_invalid_release_metadata_fails_without_exception(self) -> None:
        truth = copy.deepcopy(self.truth)
        truth["current_development"]["version"] = 42

        failures = verify.validate_release_truth(truth)
        self.assertIn("must be a non-empty string", failures[0])

    def test_release_scope_requires_both_states(self) -> None:
        text = "Latest published release: v0.4.1"

        failures = verify.release_scope_failures(text, "sample.md", "v0.4.1", "v0.4.2")
        self.assertIn("current development target", failures[0])

    def test_stale_latest_release_claim_fails(self) -> None:
        text = "Latest published release: v0.4.0\nCurrent development target: v0.4.2"

        failures = verify.release_scope_failures(text, "sample.md", "v0.4.1", "v0.4.2")
        self.assertIn("latest published release v0.4.1", failures[0])

    def test_manifest_version_drift_fails(self) -> None:
        failures = verify.manifest_version_failures("9.9.9")

        self.assertTrue(any("Cargo.toml" in failure for failure in failures))


class DesktopCiTests(unittest.TestCase):
    def test_current_desktop_ci_passes(self) -> None:
        path = Path(".github/workflows/validate.yml")
        workflow = path.read_text(encoding="utf-8")

        self.assertEqual(verify.desktop_ci_failures(workflow), [])

    def test_missing_desktop_test_fails(self) -> None:
        workflow = "desktop:\n  runs-on: macos-latest"

        failures = verify.desktop_ci_failures(workflow)
        self.assertTrue(any("cargo test" in failure for failure in failures))


class WorkflowMaintenanceTests(unittest.TestCase):
    def test_current_workflows_use_approved_actions(self) -> None:
        for path in verify.WORKFLOW_PATHS:
            with self.subTest(path=path):
                workflow = Path(path).read_text(encoding="utf-8")
                failures = verify.workflow_action_version_failures(path, workflow)
                self.assertEqual(failures, [])

    def test_current_workflows_use_minimal_permissions(self) -> None:
        for path in verify.WORKFLOW_PATHS:
            with self.subTest(path=path):
                workflow = Path(path).read_text(encoding="utf-8")
                failures = verify.workflow_permission_failures(path, workflow)
                self.assertEqual(failures, [])

    def test_deprecated_action_major_fails(self) -> None:
        workflow = "steps:\n  - uses: actions/checkout@v4"

        failures = verify.workflow_action_version_failures("workflow.yml", workflow)
        self.assertIn("actions/checkout@v7", failures[0])

    def test_unapproved_github_action_fails(self) -> None:
        workflow = "steps:\n  - uses: actions/cache@v4"

        failures = verify.workflow_action_version_failures("workflow.yml", workflow)
        self.assertIn("unapproved GitHub action", failures[0])

    def test_checkout_requires_disabled_credentials(self) -> None:
        workflow = "steps:\n  - uses: actions/checkout@v7"

        failures = verify.checkout_credential_failures("workflow.yml", workflow)
        self.assertIn("disable persisted credentials", failures[0])

    def test_macos_latest_runner_fails(self) -> None:
        failures = verify.macos_runner_failures("workflow.yml", "runs-on: macos-latest")

        self.assertIn("must not use", failures[0])

    def test_broad_workflow_permission_fails(self) -> None:
        workflow = "permissions:\n  contents: read\n  id-token: write"

        failures = verify.workflow_permission_failures("workflow.yml", workflow)
        self.assertTrue(any("id-token: write" in failure for failure in failures))


if __name__ == "__main__":
    unittest.main()
