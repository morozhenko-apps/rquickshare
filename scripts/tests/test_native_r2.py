from datetime import timedelta
import hashlib
import http.server
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).parents[1]))
import native_apt as apt
import native_r2 as r2
import native_signer as signer
import test_native_apt as fixtures


class Store:
    """Fake external conditional store; no publication rules are mocked."""
    def __init__(self):
        self.objects = {}
        self.events = []
        self.next_write = None
        self.revision = 0

    def assign(self, key, data):
        self.revision += 1
        self.objects[key] = (data, f'"opaque-{self.revision}"')

    def inspect(self, key, maximum, retain=False):
        self.events.append(("get", key))
        if key not in self.objects:
            return None
        data, etag = self.objects[key]
        return r2.Remote(etag, hashlib.sha256(data).hexdigest(), len(data), data if retain else None)

    def put(self, item, none_match=False, match_etag=None):
        self.events.append(("put", item.key, none_match, match_etag, item.cache))
        if self.next_write is not None:
            action = self.next_write
            self.next_write = None
            return action(self, item, none_match, match_etag)
        current = self.objects.get(item.key)
        if none_match and current is not None or match_etag is not None and (current is None or current[1] != match_etag):
            raise r2.Conflict()
        if not none_match and match_etag is None:
            raise AssertionError("Unconditional publication write")
        self.assign(item.key, item.source.read_bytes())


def public_proof(store, item):
    """HTTP-boundary fake checks the real published bytes, never domain decisions."""
    data = store.objects[item.key][0]
    if len(data) != item.size or hashlib.sha256(data).hexdigest() != item.sha256:
        raise ValueError("Public fixture checksum mismatch")
    return {"url": r2.BASE + item.key, "sha256": item.sha256, "size": item.size,
            "readback_sha256": item.sha256, "readback_size": len(data), "readback_status": 200}


class NativeR2Tests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        fixtures.NativeAptTests.setUpClass.__func__(cls)
        try:
            cls.next_unsigned = cls.root / "next-unsigned"
            apt.prepare([cls.package], cls.next_unsigned, cls.now + timedelta(seconds=1))
            cls.next_signed = cls.root / "next-signed"
            apt.sign(cls.next_unsigned, cls.next_signed, cls.home, cls.keyring, cls.fingerprint,
                     cls.now + timedelta(seconds=1), cls.signed)
        except BaseException:
            cls.tearDownClass()
            raise

    @classmethod
    def tearDownClass(cls):
        fixtures.NativeAptTests.tearDownClass.__func__(cls)

    def publish(self, store, candidate=None, previous=None):
        return r2.publish(candidate or self.signed, self.keyring, self.fingerprint, store, previous,
                          self.now + timedelta(seconds=1), public_verify=lambda item: public_proof(store, item))

    def test_given_first_current_or_next_release_when_published_then_all_immutable_bytes_precede_one_conditional_inrelease_commit(self):
        # Arrange / Act / Assert: real verification, opaque ETags and exact public projection.
        store = Store()
        objects, active = r2.publication(self.signed, self.keyring, self.fingerprint, now=self.now)
        self.assertEqual(self.publish(store), active.sha256)
        writes = [event for event in store.events if event[0] == "put"]
        self.assertEqual([event[1] for event in writes], [item.key for item in sorted(objects, key=lambda item: item.cache != r2.IMMUTABLE)] + [r2.ACTIVE])
        self.assertTrue(all(event[2] and event[3] is None for event in writes[:-1]))
        self.assertEqual([event[1] for event in writes if event[4] == "no-store"], ["linux/Packages", "linux/Packages.gz", r2.ACTIVE])
        self.assertEqual(sum(key.endswith(".deb") for key in store.objects), 1)
        self.assertFalse(any("snapshots/" in key or "pool/" in key or "by-hash/" in key for key in store.objects))
        self.assertEqual(writes[-1], ("put", r2.ACTIVE, True, None, "no-store"))
        for item in objects + [active]:
            self.assertEqual(store.objects[item.key][0], item.source.read_bytes())
        self.assertNotIn(f"dists/{apt.SUITE}/Release", store.objects)
        self.assertNotIn(f"dists/{apt.SUITE}/main/binary-amd64/Packages.gz", store.objects)
        before = list(store.events)
        self.assertEqual(self.publish(store), active.sha256)
        self.assertFalse(any(event[0] == "put" for event in store.events[len(before):]))
        observed_etag = store.objects[r2.ACTIVE][1]
        self.publish(store, self.next_signed, self.signed)
        writes = [event for event in store.events if event[0] == "put"]
        self.assertEqual(writes[-1], ("put", r2.ACTIVE, False, observed_etag, "no-store"))
        self.assertTrue(all(key in store.objects for key in [item.key for item in objects]))
        # Neither an unacknowledged first release nor a stale baseline may write anything.
        for candidate, previous in [(self.signed, None), (self.next_signed, self.next_signed)]:
            other = Store()
            other.assign(r2.ACTIVE, b"a different observed generation")
            with self.subTest(previous=previous), self.assertRaises(r2.Conflict):
                self.publish(other, candidate, previous)
            self.assertFalse(any(event[0] == "put" for event in other.events))

    def test_given_unsigned_or_wrong_key_snapshot_when_publish_is_requested_then_reject_before_any_storage_io(self):
        # Arrange / Act / Assert: signature rejection has no storage side effect.
        for candidate, keyring, fingerprint in [(self.unsigned, self.keyring, self.fingerprint),
                                               (self.signed, *self.keys[1])]:
            store = Store()
            with self.subTest(candidate=candidate), self.assertRaises((ValueError, OSError, subprocess.CalledProcessError)):
                r2.publish(candidate, keyring, fingerprint, store, now=self.now)
            self.assertEqual(store.events, [])

    def test_given_immutable_races_denial_or_uncertain_write_when_admitted_then_only_positive_exact_readback_can_authorize_commit(self):
        # Arrange / Act / Assert: no retry or promotion on unresolved immutable admission.
        for kind, expected in [("race-same", None), ("race-different", r2.Conflict), ("lost-ack-committed", None),
                               ("lost-ack-missing", r2.Uncertain), ("denied", r2.Denied), ("readback-failed", r2.Uncertain)]:
            with self.subTest(kind=kind):
                store = Store()
                def action(store, item, none_match, match_etag):
                    self.assertTrue(none_match)
                    if kind in {"race-same", "lost-ack-committed", "readback-failed"}:
                        store.assign(item.key, item.source.read_bytes())
                    if kind == "race-different":
                        store.assign(item.key, b"conflicting immutable bytes")
                    if kind == "readback-failed":
                        def fail(*args, **kwargs):
                            raise r2.Unavailable()
                        store.inspect = fail
                    if kind.startswith("race"):
                        raise r2.Conflict()
                    if kind == "denied":
                        raise r2.Denied()
                    raise r2.Uncertain()
                store.next_write = action
                if expected is None:
                    self.publish(store)
                else:
                    with self.assertRaises(expected):
                        self.publish(store)
                    self.assertNotIn(r2.ACTIVE, store.objects)
                first_key = next(event[1] for event in store.events if event[0] == "put")
                self.assertEqual(sum(event[0] == "put" and event[1] == first_key for event in store.events), 1)
        store = Store()
        first = next(item for item in r2.publication(self.signed, self.keyring, self.fingerprint, now=self.now)[0]
                     if item.cache == r2.IMMUTABLE)
        store.assign(first.key, b"already conflicting")
        with self.assertRaises(r2.Conflict):
            self.publish(store)
        self.assertFalse(any(event[0] == "put" for event in store.events))

    def test_given_final_commit_race_or_lost_reply_when_promoting_then_cas_never_replays_or_overwrites_a_competing_generation(self):
        # Arrange: immutable writes are complete before the active conditional write is intercepted.
        for kind, expected in [("lost-ack-committed", None), ("lost-ack-missing", r2.Uncertain),
                               ("competing-cas", r2.Conflict), ("competing-readback", r2.Uncertain), ("denied", r2.Denied)]:
            with self.subTest(kind=kind):
                store = Store()
                objects, _ = r2.publication(self.signed, self.keyring, self.fingerprint, now=self.now)
                for item in objects:
                    store.assign(item.key, item.source.read_bytes())
                def action(store, item, none_match, match_etag):
                    self.assertEqual(item.key, r2.ACTIVE)
                    if kind == "denied":
                        raise r2.Denied()
                    if kind == "lost-ack-committed":
                        store.assign(item.key, item.source.read_bytes())
                    if kind.startswith("competing"):
                        store.assign(item.key, b"another committed generation")
                    if kind == "competing-cas":
                        raise r2.Conflict()
                    raise r2.Uncertain()
                store.next_write = action
                if expected:
                    with self.assertRaises(expected):
                        self.publish(store)
                else:
                    self.publish(store)
                self.assertEqual(sum(event[0] == "put" and event[1] == r2.ACTIVE for event in store.events), 1)
                if kind.startswith("competing"):
                    self.assertEqual(store.objects[r2.ACTIVE][0], b"another committed generation")

    def test_given_real_s3_model_when_requests_fail_or_complete_then_conditions_no_retry_bounds_and_body_cleanup_follow_contract(self):
        # Arrange: explicit synthetic credentials; Stubber mocks only the external R2 service.
        from botocore.response import StreamingBody
        from botocore.stub import Stubber, ANY
        from botocore.exceptions import EndpointConnectionError
        with patch.dict(os.environ, {"RQS_R2_ACCESS_KEY_ID": "TEST_ONLY", "RQS_R2_SECRET_ACCESS_KEY": "TEST_ONLY"}):
            store = r2.connect("a" * 32)
        try:
            self.assertEqual(store.client.meta.endpoint_url, "https://" + "a" * 32 + ".r2.cloudflarestorage.com")
            self.assertEqual(store.client.meta.region_name, "auto")
            self.assertEqual(store.client.meta.config.retries["total_max_attempts"], 1)
            with Stubber(store.client) as stub, tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "data"
                path.write_bytes(b"content")
                item = r2.object_for("immutable", path)
                import base64
                parameters = {"Bucket": r2.BUCKET, "Key": r2.PREFIX + item.key, "Body": ANY, "ContentLength": 7,
                              "ContentType": "application/octet-stream", "CacheControl": r2.IMMUTABLE,
                              "ContentMD5": base64.b64encode(hashlib.md5(b"content", usedforsecurity=False).digest()).decode()}
                for condition in [{"IfNoneMatch": "*"}, {"IfMatch": '"opaque"'}]:
                    stub.add_response("put_object", {"ETag": '"result"'}, parameters | condition)
                    store.put(item, none_match="IfNoneMatch" in condition, match_etag=condition.get("IfMatch"))
                for code, status, expected in [("PreconditionFailed", 412, r2.Conflict), ("ConditionalRequestConflict", 409, r2.Conflict),
                                               ("AccessDenied", 403, r2.Denied), ("InternalError", 500, r2.Uncertain)]:
                    stub.add_client_error("put_object", code, http_status_code=status, expected_params=parameters | {"IfNoneMatch": "*"})
                    with self.subTest(code=code), self.assertRaises(expected):
                        store.put(item, none_match=True)
                for conditions in [{}, {"none_match": True, "match_etag": '"opaque"'}]:
                    with self.assertRaises(ValueError):
                        store.put(item, **conditions)
                self.assertIsInstance(store.failure(EndpointConnectionError(endpoint_url="https://invalid.example"), True), r2.Uncertain)
                self.assertIsInstance(store.failure(EndpointConnectionError(endpoint_url="https://invalid.example"), False), r2.Unavailable)
                stub.add_client_error("get_object", "NoSuchKey", http_status_code=404, expected_params={"Bucket": r2.BUCKET, "Key": r2.PREFIX + "absent"})
                self.assertIsNone(store.inspect("absent", 20))
                for content, length, maximum, etag, accepted in [(b"content", 7, 7, '"token"', True),
                        (b"", 0, 0, '"token"', True), (b"content", 7, 6, '"token"', False),
                        (b"cut", 7, 7, '"token"', False), (b"content", 7, 7, "unquoted", False)]:
                    stream = io.BytesIO(content)
                    body = StreamingBody(stream, length)
                    stub.add_response("get_object", {"Body": body, "ContentLength": length, "ETag": etag}, {"Bucket": r2.BUCKET, "Key": r2.PREFIX + "read"})
                    if accepted:
                        observed = store.inspect("read", maximum, retain=True)
                        self.assertEqual(observed.data, content)
                        self.assertEqual(observed.sha256, hashlib.sha256(content).hexdigest())
                    else:
                        with self.assertRaises(r2.Unavailable):
                            store.inspect("read", maximum, retain=True)
                    self.assertTrue(stream.closed)
                stub.assert_no_pending_responses()
        finally:
            store.close()
        for account in ["", "a" * 31, "A" * 32, "a" * 33]:
            with self.subTest(account=account), self.assertRaises(ValueError):
                r2.connect(account)
        with patch.dict(os.environ, {}, clear=True), self.assertRaises(ValueError):
            r2.connect("a" * 32)

    def test_given_canonical_request_when_approval_or_input_differs_then_signer_rejects_before_signing_and_matching_subkey_output_verifies(self):
        # Arrange / Act / Assert: ephemeral external signer uses a real signing subkey.
        commit = "a" * 40
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            request = root / "request"
            approved = signer.prepare(self.next_unsigned, self.keyring, self.fingerprint, commit, request, self.signed,
                                      self.now + timedelta(seconds=1))
            data = (request / "request.json").read_bytes()
            manifest = json.loads(data)
            self.assertEqual(manifest["previous_inrelease_sha256"], apt.digest(self.signed / r2.ACTIVE))
            self.assertEqual(manifest["source_commit"], commit)
            self.assertEqual(manifest["files"], sorted(manifest["files"], key=lambda value: value["path"]))
            for source_commit, fingerprint, digest, previous in [("b" * 40, self.fingerprint, approved, self.signed),
                    (commit, self.keys[1][1], approved, self.signed), (commit, self.fingerprint, "0" * 64, self.signed),
                    (commit, self.fingerprint, approved, None)]:
                output = root / "rejected"
                with self.subTest(source=source_commit, previous=previous), self.assertRaises((ValueError, subprocess.CalledProcessError)):
                    signer.sign(self.next_unsigned, self.keyring, fingerprint, source_commit, request / "request.json", digest,
                                root / "nonexistent-signer", output, previous, self.now + timedelta(seconds=1))
                self.assertFalse(output.exists())
            apt.run("gpg", "--no-options", "--homedir", str(self.home), "--batch", "--pinentry-mode", "loopback", "--passphrase", "",
                    "--quick-add-key", self.fingerprint, "ed25519", "sign", "0")
            public = root / "with-subkey.gpg"
            with public.open("wb") as stream:
                subprocess.run(["gpg", "--no-options", "--homedir", str(self.home), "--batch", "--export", self.fingerprint], stdout=stream, check=True)
            output = root / "signed"
            self.assertEqual(signer.sign(self.next_unsigned, public, self.fingerprint, commit, request / "request.json", approved,
                                       self.home, output, self.signed, self.now + timedelta(seconds=1)), approved)
            apt.verify(output, public, self.fingerprint, self.now + timedelta(seconds=1), self.signed)
            status = apt.run("gpgv", "--homedir", str(root), "--keyring", str(public), "--status-fd", "1", str(output / r2.ACTIVE))
            signature = next(line.split() for line in status.splitlines() if line.startswith("[GNUPG:] VALIDSIG "))
            self.assertNotEqual(signature[2], self.fingerprint)
            self.assertEqual(signature[-1], self.fingerprint)
            for value in ["", "a" * 39, "A" * 40, commit + "\n"]:
                with self.assertRaises(ValueError):
                    signer.request(self.unsigned, self.keyring, self.fingerprint, value, now=self.now)

    def test_given_public_r2_projection_when_real_apt_fetches_over_http_then_flat_apt_uses_the_single_direct_deb(self):
        # Arrange: serve only committed R2 object projection on an owned loopback test server.
        store = Store()
        self.publish(store)
        requests = []
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for key, (content, _) in store.objects.items():
                path = root / key
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(content)
            class Handler(http.server.SimpleHTTPRequestHandler):
                def __init__(self, *args, **kwargs):
                    super().__init__(*args, directory=str(root), **kwargs)
                def log_message(self, *args):
                    pass
                def do_GET(self):
                    requests.append(self.path)
                    super().do_GET()
            server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
            thread = threading.Thread(target=server.serve_forever)
            thread.start()
            try:
                fixtures.NativeAptTests.assert_apt_acquisition(self, f"http://127.0.0.1:{server.server_port}/linux/")
            finally:
                server.shutdown()
                thread.join()
                server.server_close()
            self.assertIn("/linux/./InRelease", requests)
            self.assertIn("/linux/./rquickshare-native_1.0.0-1_amd64.deb", requests)
            self.assertFalse(any("/by-hash/" in path or "/pool/" in path for path in requests))
            self.assertTrue(any(path.endswith(("/Packages", "/Packages.gz")) for path in requests))
            self.assertTrue(any(path.endswith(".deb") for path in requests))
            self.assertFalse(any(path.endswith(("/Release", "/Release.gpg")) for path in requests))

    def test_given_flat_release_when_manifest_is_built_then_source_revision_and_direct_deb_are_exact(self):
        manifest = r2.latest_manifest(self.signed, self.keyring, self.fingerprint, "a" * 40, self.now)
        self.assertEqual((manifest["product"], manifest["version"], manifest["revision"], manifest["tag"]),
                         ("rQuickShare", "1.0.0", 1, "native/v1.0.0"))
        item = manifest["files"]["linux"]["deb"]
        self.assertEqual(item, {"url": r2.BASE + "linux/rquickshare-native_1.0.0-1_amd64.deb",
                                "sha256": apt.digest(self.package), "size": self.package.stat().st_size})
        self.assertEqual(set(manifest["files"]), {"linux"})
        for commit in [None, 1, "", "A" * 40, "a" * 39, "a" * 41, "a" * 40 + "\n"]:
            with self.subTest(commit=commit), self.assertRaises(ValueError):
                r2.latest_manifest(self.signed, self.keyring, self.fingerprint, commit, self.now)

    def test_given_public_body_when_verified_then_http_size_hash_and_bounded_reads_are_required(self):
        class Reply(io.BytesIO):
            def __init__(self, data, status):
                super().__init__(data)
                self.status = status
        item = r2.object_for("linux/package.deb", self.package)
        data = self.package.read_bytes()
        for content, status, accepted in [(data, 200, True), (data[:-1], 200, False),
                (data + b"extra", 200, False), (b"x" * len(data), 200, False),
                *[(data, status, False) for status in [206, 301, 403, 404, 429, 500, 503]]]:
            with self.subTest(status=status, length=len(content)):
                reply = Reply(content, status)
                calls = []
                def opener(url, timeout):
                    calls.append((url, timeout))
                    return reply
                if accepted:
                    result = r2.public_readback(item, opener)
                    self.assertEqual(result["sha256"], result["readback_sha256"])
                    self.assertEqual(result["size"], result["readback_size"])
                else:
                    with self.assertRaises(ValueError):
                        r2.public_readback(item, opener)
                self.assertEqual(len(calls), 1)
                request, timeout = calls[0]
                self.assertIsInstance(request, r2.urllib.request.Request)
                self.assertEqual(request.full_url, r2.BASE + item.key)
                self.assertEqual(request.get_header("User-agent"), r2.PUBLIC_VERIFY_USER_AGENT)
                self.assertEqual(timeout, 45)
                self.assertNotIn("Authorization", request.headers)
                self.assertTrue(reply.closed)

    def test_given_public_readback_transport_error_when_request_is_dispatched_then_it_fails_closed_without_credentials(self):
        item = r2.object_for("linux/package.deb", self.package)
        requests = []
        def unavailable(request, timeout):
            requests.append((request, timeout))
            raise OSError("synthetic connection failure")
        with self.assertRaises(OSError):
            r2.public_readback(item, unavailable)
        self.assertEqual(len(requests), 1)
        request, timeout = requests[0]
        self.assertEqual(request.full_url, r2.BASE + item.key)
        self.assertEqual(timeout, 45)
        self.assertEqual(request.get_header("User-agent"), r2.PUBLIC_VERIFY_USER_AGENT)
        self.assertNotIn("Authorization", request.headers)
        self.assertNotIn("Proxy-Authorization", request.headers)

    def test_given_public_readback_body_error_when_streaming_then_reply_closes_and_receipt_is_not_fabricated(self):
        class BrokenReply(io.BytesIO):
            status = 200
            def read(self, size=-1):
                raise OSError("synthetic mid-stream disconnect")
        item = r2.object_for("linux/package.deb", self.package)
        reply = BrokenReply(self.package.read_bytes())
        def opener(request, timeout):
            self.assertEqual(request.get_header("User-agent"), r2.PUBLIC_VERIFY_USER_AGENT)
            return reply
        with self.assertRaises(OSError):
            r2.public_readback(item, opener)
        self.assertTrue(reply.closed)

    def test_given_full_flat_publication_when_completed_then_latest_is_last_and_receipt_covers_only_one_deb(self):
        store = Store()
        keys = []
        def public(item):
            keys.append(item.key)
            return public_proof(store, item)
        with tempfile.TemporaryDirectory() as temporary:
            receipt = Path(temporary) / "receipt.json"
            result = r2.publish_release(self.signed, self.keyring, self.fingerprint, store, "a" * 40,
                                        51, receipt, now=self.now, public_verify=public)
            self.assertEqual(json.loads(receipt.read_text()), result)
            self.assertEqual(keys[-2:], [r2.ACTIVE, "latest.json"])
            self.assertEqual(sum(key.endswith(".deb") for key in store.objects), 1)
            self.assertEqual(result["artifacts"][0]["name"], "native-package")
            self.assertEqual(len(result["artifacts"][0]["files"]), 2)
            self.assertEqual(json.loads(store.objects["latest.json"][0])["commit"], "a" * 40)
            before = len(store.events)
            r2.publish_release(self.signed, self.keyring, self.fingerprint, store, "a" * 40,
                               51, Path(temporary) / "rerun.json", now=self.now, public_verify=public)
            self.assertFalse(any(event[0] == "put" for event in store.events[before:]))
            with self.assertRaises(ValueError):
                r2.publish_release(self.signed, self.keyring, self.fingerprint, store, "a" * 40,
                                   51, receipt, now=self.now, public_verify=public)

    def test_given_public_failure_when_any_publication_phase_is_unverified_then_no_cleanup_receipt_exists(self):
        for phase in [".deb", "linux/Packages", r2.ACTIVE, "latest.json"]:
            with self.subTest(phase=phase), tempfile.TemporaryDirectory() as temporary:
                store = Store()
                receipt = Path(temporary) / "receipt.json"
                def public(item):
                    if item.key.endswith(phase):
                        raise OSError("synthetic HTTP failure")
                    return public_proof(store, item)
                with self.assertRaises(OSError):
                    r2.publish_release(self.signed, self.keyring, self.fingerprint, store, "a" * 40,
                                       51, receipt, now=self.now, public_verify=public)
                self.assertFalse(receipt.exists())
                if phase != "latest.json":
                    self.assertNotIn("latest.json", store.objects)
                if phase in [".deb", "linux/Packages"]:
                    self.assertNotIn(r2.ACTIVE, store.objects)


if __name__ == "__main__":
    unittest.main()
