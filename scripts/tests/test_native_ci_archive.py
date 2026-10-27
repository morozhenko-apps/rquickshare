import gzip
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).parents[1]))
import native_apt as apt
import native_r2 as r2
import native_ci_archive as archive
import test_native_apt as fixtures


class StreamingBody:
    def __init__(self, data):
        self.data = data
        self.closed = False

    def iter_chunks(self, chunk_size=65536):
        for i in range(0, len(self.data), chunk_size):
            yield self.data[i:i + chunk_size]

    def close(self):
        self.closed = True


class FakeS3:
    """External S3 byte adapter. No archive discovery/listing API is available."""
    def __init__(self, objects):
        self.objects = dict(objects)
        self.fail_reads = False
        self.length_override = {}
        self.requests = []
        self.bodies = []

    def get_object(self, Bucket, Key):
        assert Bucket == "app-dist"
        assert Key.startswith("dist/rQuickShare/")
        self.requests.append(Key)
        if self.fail_reads:
            raise OSError("Bucket unavailable")
        if Key not in self.objects:
            error = OSError("Missing key")
            error.response = {"Error": {"Code": "NoSuchKey"}}
            raise error
        data = self.objects[Key]
        body = StreamingBody(data)
        self.bodies.append(body)
        return {"ContentLength": self.length_override.get(Key, len(data)),
                "ETag": '"fixture-etag"', "Body": body}

    def close(self):
        pass


class NativeCIArchiveTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        fixtures.NativeAptTests.setUpClass.__func__(cls)

    @classmethod
    def tearDownClass(cls):
        fixtures.NativeAptTests.tearDownClass.__func__(cls)

    def channel(self):
        objects, active = r2.publication(self.signed, self.keyring, self.fingerprint, now=self.now)
        manifest = r2.latest_manifest(self.signed, self.keyring, self.fingerprint, "a" * 40, self.now)
        return FakeS3({**{r2.PREFIX + item.key: item.source.read_bytes() for item in objects},
                       r2.PREFIX + active.key: active.source.read_bytes(),
                       r2.PREFIX + "latest.json": json.dumps(manifest).encode()}), active

    def test_given_signed_flat_channel_when_recovered_then_single_original_package_and_authority_are_verified(self):
        # Arrange / Act / Assert: real GPG, Debian and archive verification.
        client, active = self.channel()
        with tempfile.TemporaryDirectory() as temporary:
            recovered = Path(temporary) / "previous"
            self.assertTrue(archive.restore(r2.S3Store(client), recovered, self.keyring, self.fingerprint))
            self.assertEqual(apt.verify(recovered, self.keyring, self.fingerprint, self.now)["version"], "1.0.0-1")
            self.assertEqual((recovered / r2.ACTIVE).read_bytes(), active.source.read_bytes())
            packages = list(recovered.glob("linux/*.deb"))
            self.assertEqual(len(packages), 1)
            self.assertEqual(packages[0].read_bytes(), self.package.read_bytes())
            self.assertTrue(all(body.closed for body in client.bodies))
            self.assertFalse(any("snapshots/" in key or "pool/" in key for key in client.requests))

    def test_given_flat_manifest_when_source_is_selected_then_only_canonical_native_tag_is_proposed(self):
        client, active = self.channel()
        self.assertEqual(archive.current_tag(r2.S3Store(client)), "native/v1.0.0")
        key = r2.PREFIX + "latest.json"
        original = json.loads(client.objects[key])
        for field, value in [("product", "Other"), ("version", "../escape"), ("version", None),
                             ("tag", "v1.0.0"), ("commit", "bad"), ("commit", 7), ("schema", 2)]:
            with self.subTest(field=field, value=value):
                client.objects[key] = json.dumps(original | {field: value}).encode()
                with self.assertRaises(ValueError):
                    archive.current_tag(r2.S3Store(client))
        del client.objects[key]
        with self.assertRaises(ValueError):
            archive.current_tag(r2.S3Store(client))
        del client.objects[r2.PREFIX + active.key]
        self.assertIsNone(archive.current_tag(r2.S3Store(client)))

    def test_given_missing_authority_when_recovered_then_only_confirmed_absence_is_first_release(self):
        client, active = self.channel()
        del client.objects[r2.PREFIX + active.key]
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "previous"
            self.assertFalse(archive.restore(r2.S3Store(client), output, self.keyring, self.fingerprint))
            self.assertFalse(output.exists())

    def test_given_missing_corrupt_or_oversized_objects_when_recovered_then_no_local_output_is_committed(self):
        for kind in ["missing-index", "missing-package", "corrupt-signature", "corrupt-index", "corrupt-package", "oversize", "remote-outage"]:
            with self.subTest(kind=kind), tempfile.TemporaryDirectory() as temporary:
                client, active = self.channel()
                package = next(key for key in client.objects if key.endswith(".deb"))
                if kind == "missing-index":
                    del client.objects[r2.PREFIX + "linux/Packages"]
                elif kind == "missing-package":
                    del client.objects[package]
                elif kind == "corrupt-signature":
                    client.objects[r2.PREFIX + active.key] = b"tampered signed metadata"
                elif kind == "corrupt-index":
                    client.objects[r2.PREFIX + "linux/Packages"] += b"tampered"
                elif kind == "corrupt-package":
                    client.objects[package] = b"tampered"
                elif kind == "oversize":
                    client.length_override[package] = apt.MAX_PACKAGE + 1
                else:
                    client.fail_reads = True
                output = Path(temporary) / "previous"
                with self.assertRaises((ValueError, OSError, subprocess.CalledProcessError, r2.Unavailable)):
                    archive.restore(r2.S3Store(client), output, self.keyring, self.fingerprint)
                self.assertFalse(output.exists())
                self.assertTrue(all(body.closed for body in client.bodies))

    def test_given_untrusted_keys_when_admitted_then_only_flat_public_metadata_and_exact_packages_are_allowed(self):
        for key in ["linux/Packages", "linux/Packages.gz", "linux/InRelease",
                    "linux/rquickshare-native_1.0.0-1_amd64.deb", "linux/rquickshare-native_1.0.0-1_arm64.deb.sha256"]:
            with self.subTest(key=key):
                self.assertTrue(archive.admitted(key))
        for key in [None, 7, "", "latest.json", "../escape", "linux/../secret", "linux/%2e%2e/secret",
                    "snapshots/hash/linux/Packages", "linux/Release.gpg", "linux/rquickshare-native_1.0.0-0_amd64.deb",
                    "linux/rquickshare-native_1.0.0-1_i386.deb", "linux/rquickshare-native_1.0.0-1_amd64.deb?query"]:
            with self.subTest(key=key):
                self.assertFalse(archive.admitted(key))

    def test_given_signed_unsafe_filename_when_recovered_then_no_package_path_is_followed(self):
        # Arrange: sign a deliberately malformed catalog with the ephemeral trusted fixture key.
        for filename in ["../escape.deb", "./../escape.deb", "https://other.invalid/x.deb", "./rquickshare-native_1.0.0-1_i386.deb"]:
            with self.subTest(filename=filename), tempfile.TemporaryDirectory() as temporary:
                local = Path(temporary) / "malformed"
                shutil.copytree(self.signed, local)
                plain = (local / "linux/Packages").read_bytes().replace(
                    b"Filename: ./rquickshare-native_1.0.0-1_amd64.deb", ("Filename: " + filename).encode())
                (local / "linux/Packages").write_bytes(plain)
                (local / "linux/Packages.gz").write_bytes(gzip.compress(plain, mtime=0))
                (local / "linux/Release").write_bytes(apt.release(local, ["amd64"], self.now))
                apt.run("gpg", "--no-options", "--homedir", str(self.home), "--batch", "--yes",
                        "--local-user", self.fingerprint, "--digest-algo", "SHA256", "--output",
                        str(local / "linux/InRelease"), "--clearsign", str(local / "linux/Release"))
                client, _ = self.channel()
                for name in ["InRelease", "Packages", "Packages.gz"]:
                    client.objects[r2.PREFIX + "linux/" + name] = (local / "linux" / name).read_bytes()
                with self.assertRaises(ValueError):
                    archive.restore(r2.S3Store(client), Path(temporary) / "out", self.keyring, self.fingerprint)
                self.assertFalse(any(key.endswith(".deb") for key in client.requests))

    def test_given_remote_body_bounds_when_copied_then_truncation_overflow_and_invalid_lengths_fail_and_close(self):
        for size in [-1, True, 4, 8, 100]:
            with self.subTest(size=size), tempfile.TemporaryDirectory() as temporary:
                key = r2.PREFIX + "linux/Packages"
                client = FakeS3({key: b"content"})
                client.length_override[key] = size
                with self.assertRaises(ValueError):
                    archive.copy_object(client, "linux/Packages", Path(temporary) / "data", 8)
                self.assertTrue(client.bodies[0].closed)


if __name__ == "__main__":
    unittest.main()
