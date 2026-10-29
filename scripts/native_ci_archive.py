#!/usr/bin/env python3
"""Recover the current signed flat APT channel; never keep remote snapshots."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys

import native_apt as apt
import native_r2 as r2
import native_release_tag as release_tag

PACKAGE = re.compile(r"linux/rquickshare-native_(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)-[1-9][0-9]{0,9}_(?:amd64|arm64)\.deb(?:\.sha256)?\Z")


def admitted(key):
    return isinstance(key, str) and (PACKAGE.fullmatch(key) is not None or
                                    key in {"linux/Packages", "linux/Packages.gz", "linux/InRelease"})


def copy_object(client, key, target, maximum):
    apt.require(admitted(key), "Unexpected flat archive key")
    try:
        response = client.get_object(Bucket=r2.BUCKET, Key=r2.PREFIX + key)
    except Exception:
        raise r2.Unavailable("Flat archive read unavailable") from None
    body = response["Body"]
    try:
        size = response["ContentLength"]
        apt.require(type(size) is int and 0 <= size <= maximum, "Archived object exceeds accepted bound")
        received = 0
        with target.open("xb") as output:
            for chunk in body.iter_chunks(chunk_size=65536):
                received += len(chunk)
                apt.require(received <= size, "Archived object exceeds declared length")
                output.write(chunk)
        apt.require(received == size, "Truncated archived object")
    finally:
        body.close()


def restore(store, output, keyring, fingerprint):
    """False only for a confirmed missing InRelease; all other failures are fatal."""
    current = store.inspect(r2.ACTIVE, 32768, retain=True)
    if current is None:
        return False
    apt.require(current.data is not None and hashlib.sha256(current.data).hexdigest() == current.sha256,
                "Unverified active repository readback")
    apt.public_key(keyring, fingerprint)
    with apt.stage(output) as root:
        base = root / "linux"
        base.mkdir()
        (base / "InRelease").write_bytes(current.data)
        # Authenticate the clearsigned metadata before following any file names.
        apt.run("gpgv", "--keyring", str(keyring.absolute()), "--output", str(base / "Release"),
                str(base / "InRelease"))
        apt.signatures(root, keyring, fingerprint)
        for name in ["Packages", "Packages.gz"]:
            copy_object(store.client, "linux/" + name, base / name, apt.MAX_INDEX)
        release_fields = apt.paragraphs(apt.read(base / "Release", 16384))
        apt.require(len(release_fields) == 1, "Unexpected Release catalog")
        fields = release_fields[0]
        architectures = fields.get("Architectures", "").split(" ")
        apt.require(architectures in [["amd64"], ["arm64"], ["amd64", "arm64"]], "Invalid channel architectures")
        from email.utils import parsedate_to_datetime
        date = parsedate_to_datetime(fields["Date"])
        apt.require(apt.read(base / "Release") == apt.release(root, architectures, date),
                    "Flat indexes differ from signed Release")
        entries = apt.paragraphs(apt.read(base / "Packages"))
        apt.require(len(entries) == len(architectures), "Unexpected flat package inventory")
        for entry in entries:
            apt.native_deb.validate_metadata(entry)
            filename = entry.get("Filename", "")
            apt.require(filename.startswith("./") and admitted("linux/" + filename[2:]) and filename.endswith(".deb"),
                        "Unsafe indexed package path")
            size = int(entry["Size"])
            apt.require(0 < size <= apt.MAX_PACKAGE, "Invalid indexed package size")
            name = filename[2:]
            copy_object(store.client, "linux/" + name, base / name, size)
            copy_object(store.client, "linux/" + name + ".sha256", base / (name + ".sha256"), 256)
        apt.verify(root, keyring, fingerprint, check_time=False)
        # The mutable authority must still be the exact generation inspected first.
        end = store.inspect(r2.ACTIVE, 32768, retain=True)
        apt.require(end is not None and end.sha256 == current.sha256, "Channel changed during recovery")
    return True


def current_tag(store):
    """Advisory source tag; refresh independently authenticates the signed archive."""
    if store.inspect(r2.ACTIVE, 32768, retain=True) is None:
        return None
    current = store.inspect("latest.json", 32768, retain=True)
    apt.require(current is not None and current.data is not None, "Published manifest is missing")
    manifest = json.loads(current.data)
    apt.require(isinstance(manifest, dict) and manifest.get("schema") == 1 and manifest.get("product") == "rQuickShare",
                "Unexpected published manifest")
    version = manifest.get("version")
    apt.require(isinstance(version, str), "Missing published version")
    tag = release_tag.tag_from_version(version)
    apt.require(manifest.get("tag") == tag and isinstance(manifest.get("commit"), str) and
                re.fullmatch(r"[a-f0-9]{40}", manifest["commit"]),
                "Manifest source identity mismatch")
    return tag


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--account-id", required=True)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--keyring", type=Path)
    parser.add_argument("--fingerprint")
    parser.add_argument("--select-tag", action="store_true")
    args = parser.parse_args()
    store = None
    try:
        if args.select_tag:
            apt.require(args.output is None and args.keyring is None and args.fingerprint is None, "Select-tag must not take restore inputs")
            store = r2.connect(args.account_id)
            print(current_tag(store) or "")
            return 0
        apt.require(args.output is not None and args.keyring is not None and args.fingerprint is not None, "Restore requires output and authority")
        apt.require(not args.output.exists(), "Existing restored output is forbidden")
        apt.public_key(args.keyring, args.fingerprint)
        store = r2.connect(args.account_id)
        has_previous = restore(store, args.output, args.keyring, args.fingerprint)
        print("previous" if has_previous else "first")
        return 0
    except Exception as error:
        print("Prior release recovery failed:", type(error).__name__, file=sys.stderr)
        return 1
    finally:
        if store is not None:
            store.close()


if __name__ == "__main__":
    sys.exit(main())
