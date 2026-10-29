import subprocess
import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).parents[1]))
import native_release_tag as release_tag


class NativeReleaseTagTests(unittest.TestCase):
    def test_given_native_1_0_0_publication_when_noted_then_accepted_limitations_are_disclosed(self):
        workflow = (Path(__file__).parents[2] / ".github/workflows/native-tag-publish.yml").read_text()
        self.assertIn('if [[ "$TAG" == "native/v1.0.0" ]]', workflow)
        self.assertIn("issues/8", workflow)
        self.assertIn("issues/16", workflow)
        self.assertIn("Neither issue is claimed fixed", workflow)
        self.assertIn("explicit consent", workflow)

    def test_given_independent_product_semver_when_tagged_then_only_native_namespace_is_used(self):
        for version in ("0.0.0", "0.9.0", "1.0.0", "12.34.56", "123456.0.9876"):
            with self.subTest(version=version):
                tag = release_tag.tag_from_version(version)
                self.assertEqual(tag, "native/v" + version)
                self.assertEqual(release_tag.version_from_tag(tag), version)

    def test_given_upstream_or_untrusted_ref_when_parsed_then_it_cannot_be_a_native_release(self):
        invalid = (
            "", "v0.9.0", "v0.11.5", "native/v", "native/v1", "native/v1.2",
            "native/v1.2.3.4", "native/v01.2.3", "native/v1.02.3",
            "native/v1.2.03", "native/v1.2.3-rc1", "native/v1.2.3+build",
            "native/v-1.2.3", "native/v1.2.-3", "native/v1.2.3/",
            "native/v1.2.3/extra", "native/v1.2.3\\other",
            "native/../v1.2.3", "native/v1.2.3\n", "native/v1.2.3 ",
            "Native/v1.2.3", "native/V1.2.3", "native/v1.２.3",
            "native/v1.2.3\\n", "refs/tags/native/v1.2.3",
        )
        for value in invalid:
            with self.subTest(ref=value), self.assertRaises(ValueError):
                release_tag.version_from_tag(value)
        for value in invalid + ("1.2.3.4", "01.2.3", "1.2.3+test"):
            with self.subTest(version=value), self.assertRaises(ValueError):
                release_tag.tag_from_version(value)

    def test_given_native_source_tag_cli_when_admitted_then_exact_version_is_returned(self):
        script = Path(__file__).parents[1] / "native_release_tag.py"
        for operation, value, output in (
            ("version", "native/v0.9.0", "0.9.0"),
            ("tag", "0.9.0", "native/v0.9.0"),
            ("version", "native/v1.0.0", "1.0.0"),
        ):
            with self.subTest(operation=operation, value=value):
                proc = subprocess.run([sys.executable, str(script), operation, value], capture_output=True, text=True)
                self.assertEqual(proc.returncode, 0, proc.stderr)
                self.assertEqual(proc.stdout, output + "\n")
        for operation, value in (("version", "v0.9.0"), ("version", "native/v0.9.0-rc1"), ("tag", "0.9.0-rc1")):
            with self.subTest(operation=operation, value=value):
                proc = subprocess.run([sys.executable, str(script), operation, value], capture_output=True, text=True)
                self.assertNotEqual(proc.returncode, 0)
                self.assertEqual(proc.stdout, "")


if __name__ == "__main__":
    unittest.main()
