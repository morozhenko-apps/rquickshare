"""Regression tests for GitHub checkout's replacement of a local annotated tag ref."""

import subprocess
import tempfile
from pathlib import Path
import unittest


class NativeReleaseCheckoutTests(unittest.TestCase):
    def run_git(self, cwd, *args):
        proc = subprocess.run(
            ["git", "-C", str(cwd), *args],
            capture_output=True, text=True, check=False,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        return proc.stdout.strip()

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="rqs-native-tag-")
        self.addCleanup(self.temporary.cleanup)
        root = Path(self.temporary.name)
        self.remote = root / "origin.git"
        self.work = root / "work"
        self.run_git(root, "init", "--bare", str(self.remote))
        self.run_git(root, "init", str(self.work))
        self.run_git(self.work, "config", "user.name", "Native tag regression")
        self.run_git(self.work, "config", "user.email", "native-tag@example.invalid")
        self.run_git(self.work, "commit", "--allow-empty", "-m", "Initial release source")
        self.run_git(self.work, "branch", "-M", "master")
        self.run_git(self.work, "remote", "add", "origin", str(self.remote))
        self.commit = self.run_git(self.work, "rev-parse", "HEAD")

    def test_given_checkout_replaces_local_tag_when_remote_tag_is_fetched_then_annotated_authority_survives(self):
        tag = "native/v1.0.0"
        self.run_git(self.work, "tag", "-a", tag, "-m", "Native 1.0.0")
        self.run_git(self.work, "push", "origin", "master", "refs/tags/" + tag)
        annotated_sha = self.run_git(self.work, "rev-parse", "refs/tags/" + tag)
        self.assertEqual(self.run_git(self.work, "cat-file", "-t", annotated_sha), "tag")

        # Reproduce checkout@v5's local fetch of the tag's commit into the tag ref.
        self.run_git(self.work, "update-ref", "refs/tags/" + tag, self.commit)
        self.assertEqual(self.run_git(self.work, "cat-file", "-t", "refs/tags/" + tag), "commit")

        validation = "refs/release-validation/native-tag"
        self.run_git(self.work, "fetch", "--no-tags", "origin", "refs/tags/" + tag + ":" + validation)
        self.assertEqual(self.run_git(self.work, "cat-file", "-t", validation), "tag")
        self.assertEqual(self.run_git(self.work, "rev-parse", validation), annotated_sha)
        self.assertEqual(self.run_git(self.work, "rev-parse", validation + "^{commit}"), self.commit)
        # The runner's locally overwritten tag is irrelevant to admission.
        self.assertEqual(self.run_git(self.work, "cat-file", "-t", "refs/tags/" + tag), "commit")

    def test_given_remote_lightweight_tag_when_fetched_then_annotated_admission_rejects_it(self):
        tag = "native/v1.0.1"
        self.run_git(self.work, "tag", tag)
        self.run_git(self.work, "push", "origin", "master", "refs/tags/" + tag)
        validation = "refs/release-validation/native-tag"
        self.run_git(self.work, "fetch", "--no-tags", "origin", "refs/tags/" + tag + ":" + validation)
        self.assertEqual(self.run_git(self.work, "cat-file", "-t", validation), "commit")
        self.assertNotEqual(self.run_git(self.work, "cat-file", "-t", validation), "tag")

    def test_given_release_workflow_when_inspected_then_remote_tag_and_push_sha_are_verified(self):
        workflow = (Path(__file__).parents[2] / ".github/workflows/native-tag-publish.yml").read_text()
        self.assertIn('git fetch --no-tags origin "refs/tags/$GITHUB_REF_NAME:refs/release-validation/native-tag"', workflow)
        self.assertIn('test "$(git cat-file -t refs/release-validation/native-tag)" = tag', workflow)
        self.assertIn('"$(git rev-parse refs/release-validation/native-tag^{commit})" == "$GITHUB_SHA"', workflow)
        self.assertIn('"$(git rev-parse HEAD)" == "$(git rev-parse origin/master^{commit})"', workflow)


if __name__ == "__main__":
    unittest.main()
