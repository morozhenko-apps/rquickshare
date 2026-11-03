"""Source contract for bounded read-only R2 visibility diagnostics."""
import hashlib
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).parents[1]))
import native_ci_archive as archive
import native_r2 as r2
import native_r2_probe as probe

EXPECTED = hashlib.sha256(b"known signed content").hexdigest()
REMOTE = r2.Remote('"et"', EXPECTED, len(b"known signed content"), b"known signed content")


class FakeStore:
    def __init__(self, replies):
        self.replies = iter(replies)
        self.calls = []
        self.closed = False

    def inspect(self, key, maximum, retain=False):
        self.calls.append((key, maximum, retain))
        result = next(self.replies)
        if isinstance(result, Exception):
            raise result
        return result

    def close(self):
        self.closed = True


class R2VisibilityProbeTests(unittest.TestCase):
    def test_given_matching_reads_and_tag_when_probed_then_success_without_writes(self):
        store = FakeStore([REMOTE] * 3)
        events = []
        with patch.object(archive, "current_tag", return_value="native/v1.0.0") as selection:
            self.assertTrue(probe.probe_session(store, EXPECTED, 1, events.append))
        self.assertEqual(len(store.calls), 3)
        self.assertTrue(all(call == (r2.ACTIVE, 32768, True) for call in store.calls))
        self.assertEqual(events.count("session=1 read=1 outcome=checksum_match"), 1)
        selection.assert_called_once_with(store)

    def test_given_intermittent_missing_or_broken_reads_then_never_select_tag(self):
        for replies in ([REMOTE, None, REMOTE], [None, None, None], [REMOTE, r2.Unavailable(), REMOTE],
                        [REMOTE, r2.Remote('"et"', "a" * 64, 1), REMOTE]):
            with self.subTest(replies=replies):
                store = FakeStore(replies)
                with patch.object(archive, "current_tag") as selection:
                    self.assertFalse(probe.probe_session(store, EXPECTED, 2, lambda _: None))
                self.assertEqual(len(store.calls), 3)
                selection.assert_not_called()

    def test_given_tag_ambiguity_then_diagnostic_fails_closed(self):
        store = FakeStore([REMOTE] * 3)
        with patch.object(archive, "current_tag", return_value=None):
            self.assertFalse(probe.probe_session(store, EXPECTED, 1, lambda _: None))
        store = FakeStore([REMOTE] * 3)
        with patch.object(archive, "current_tag", side_effect=r2.Unavailable()):
            self.assertFalse(probe.probe_session(store, EXPECTED, 1, lambda _: None))

    def test_given_bad_hash_then_no_network_client_is_created(self):
        opened = []
        self.assertEqual(probe.diagnose("a" * 32, "bad", lambda _: None, lambda _: opened.append(1)), 2)
        self.assertEqual(opened, [])

    def test_given_three_fresh_sessions_then_all_close_and_agree(self):
        stores = [FakeStore([REMOTE] * 3) for _ in range(3)]
        def connect(_):
            return stores.pop(0)
        with patch.object(archive, "current_tag", return_value="native/v1.0.0"):
            self.assertEqual(probe.diagnose("a" * 32, EXPECTED, lambda _: None, connect), 0)
        self.assertEqual(stores, [])

    def test_given_one_bad_session_then_diagnostic_refuses_success(self):
        stores = [FakeStore([REMOTE] * 3), FakeStore([REMOTE, None, REMOTE]), FakeStore([REMOTE] * 3)]
        def connect(_):
            return stores.pop(0)
        with patch.object(archive, "current_tag", return_value="native/v1.0.0"):
            self.assertEqual(probe.diagnose("a" * 32, EXPECTED, lambda _: None, connect), 1)
        self.assertEqual(stores, [])


if __name__ == "__main__":
    unittest.main()
