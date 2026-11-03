#!/usr/bin/env python3
"""Read-only R2 visibility probe using the same S3 adapter as Native release."""
import argparse
import re
import sys

import native_ci_archive as archive
import native_r2 as r2

READS_PER_SESSION = 3
SESSIONS = 3


def probe_session(store, expected_sha256, number, emit=print):
    """Observe repeated S3 reads; never treat a missing object as a new release."""
    all_matching = True
    for read_number in range(1, READS_PER_SESSION + 1):
        try:
            observed = store.inspect(r2.ACTIVE, 32768, retain=True)
        except Exception as error:
            emit(f"session={number} read={read_number} outcome=error class={type(error).__name__}")
            all_matching = False
            continue
        if observed is None:
            emit(f"session={number} read={read_number} outcome=NoSuchKey")
            all_matching = False
        elif observed.sha256 != expected_sha256:
            emit(f"session={number} read={read_number} outcome=checksum_mismatch")
            all_matching = False
        else:
            emit(f"session={number} read={read_number} outcome=checksum_match")
    if not all_matching:
        return False
    try:
        tag = archive.current_tag(store)
    except Exception as error:
        emit(f"session={number} tag_selection=error class={type(error).__name__}")
        return False
    if tag is None:
        emit(f"session={number} tag_selection=missing")
        return False
    emit(f"session={number} tag_selection=present")
    return True


def diagnose(account_id, expected_sha256, emit=print, connect=r2.connect):
    if not isinstance(expected_sha256, str) or re.fullmatch(r"[a-f0-9]{64}", expected_sha256) is None:
        emit("diagnostic=invalid_expected_checksum")
        return 2
    success = True
    for number in range(1, SESSIONS + 1):
        try:
            store = connect(account_id)
        except Exception as error:
            emit(f"session={number} connection=error class={type(error).__name__}")
            return 1
        try:
            if not probe_session(store, expected_sha256, number, emit):
                success = False
        finally:
            store.close()
    emit("diagnostic=consistent" if success else "diagnostic=inconsistent")
    return 0 if success else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--account-id", required=True)
    parser.add_argument("--expected-sha256", required=True)
    args = parser.parse_args()
    return diagnose(args.account_id, args.expected_sha256)


if __name__ == "__main__":
    sys.exit(main())
