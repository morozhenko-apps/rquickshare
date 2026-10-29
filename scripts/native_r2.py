#!/usr/bin/env python3
"""Publish verified flat APT files to app-dist/dist/rQuickShare. Never signs."""
import argparse
import base64
from dataclasses import dataclass
import hashlib
import json
import os
from pathlib import Path
import re
import sys
import tempfile
import urllib.request

import native_apt as apt

BUCKET = "app-dist"
PREFIX = "dist/rQuickShare/"
BASE = "https://download.morozhenko.com/" + PREFIX
ACTIVE = "linux/InRelease"
IMMUTABLE = "public, max-age=31536000, immutable"


class Conflict(Exception):
    pass


class Uncertain(Exception):
    pass


class Denied(Exception):
    pass


class Unavailable(Exception):
    pass


@dataclass(frozen=True)
class Remote:
    etag: str
    sha256: str
    size: int
    data: bytes | None = None


@dataclass(frozen=True)
class Object:
    key: str
    source: Path
    sha256: str
    size: int
    cache: str
    content_type: str


def object_for(key, source, mutable=False):
    return Object(key, source, apt.digest(source), source.stat().st_size,
                  "no-store" if mutable else IMMUTABLE,
                  content_type(source))


def content_type(source):
    if source.name.endswith(".deb"):
        return "application/vnd.debian.binary-package"
    if source.name.endswith(".gpg"):
        return "application/pgp-keys"
    if source.name.endswith(".gz"):
        return "application/gzip"
    if source.name.endswith(".json"):
        return "application/json"
    if source.name in {"Packages", "InRelease", "Release"} or source.name.endswith(".sha256"):
        return "text/plain; charset=utf-8"
    return "application/octet-stream"


def publication(snapshot, keyring, fingerprint, previous=None, now=None):
    apt.verify(snapshot, keyring, fingerprint, now, previous)
    active = object_for(ACTIVE, snapshot / ACTIVE, mutable=True)
    objects = []
    for path in sorted(apt.inventory(snapshot)):
        if path.is_file() and path.name not in {"Release", "InRelease"}:
            relative = path.relative_to(snapshot).as_posix()
            objects.append(object_for(relative, path, mutable=path.name in {"Packages", "Packages.gz"}))
    objects.append(object_for("linux/rquickshare-native-archive-keyring.gpg", keyring))
    return objects, active


PUBLIC_VERIFY_USER_AGENT = "rQuickShare-ReleaseVerifier/1.0"


def public_readback(item, opener=None):
    opener = urllib.request.urlopen if opener is None else opener
    request = urllib.request.Request(
        BASE + item.key, headers={"User-Agent": PUBLIC_VERIFY_USER_AGENT}
    )
    with opener(request, timeout=45) as response:
        apt.require(response.status == 200, "Public download returned non-200")
        digest = hashlib.sha256()
        length = 0
        while chunk := response.read(min(1024 * 1024, item.size - length + 1)):
            length += len(chunk)
            apt.require(length <= item.size, "Public download exceeds expected size")
            digest.update(chunk)
        apt.require(length == item.size and digest.hexdigest() == item.sha256, "Public download checksum/size mismatch")
    return {"url": BASE + item.key, "sha256": item.sha256, "size": item.size,
            "readback_sha256": digest.hexdigest(), "readback_size": length, "readback_status": 200}


def matches(remote, expected):
    return remote is not None and remote.sha256 == expected.sha256 and remote.size == expected.size


def inspect_after_write(store, expected):
    try:
        remote = store.inspect(expected.key, expected.size, retain=expected.key == ACTIVE)
    except (Unavailable, Denied, ValueError) as error:
        raise Uncertain("Cannot confirm admitted object write") from error
    if not matches(remote, expected):
        raise Uncertain("Admitted object write remains unconfirmed")
    return remote


def ensure_immutable(store, item):
    observed = store.inspect(item.key, item.size)
    if observed is not None:
        if not matches(observed, item):
            raise Conflict("Immutable object differs")
        return
    try:
        store.put(item, none_match=True)
    except Conflict:
        # A competing create is acceptable only if actual bytes match this immutable key.
        if not matches(store.inspect(item.key, item.size), item):
            raise Conflict("Competing immutable object differs") from None
    except Uncertain:
        inspect_after_write(store, item)
    else:
        inspect_after_write(store, item)


def ensure_mutable(store, item):
    current = store.inspect(item.key, apt.MAX_INDEX)
    if matches(current, item):
        return
    try:
        store.put(item, none_match=current is None, match_etag=None if current is None else current.etag)
    except Uncertain:
        inspect_after_write(store, item)
    else:
        inspect_after_write(store, item)


def publish(snapshot, keyring, fingerprint, store, previous=None, now=None, public_verify=public_readback):
    # Validate all signatures, package bytes and previous authority before storage IO.
    objects, active = publication(snapshot, keyring, fingerprint, previous, now)
    current = store.inspect(ACTIVE, 32768, retain=True)
    already_current = matches(current, active)
    if not already_current:
        if previous is None:
            if current is not None:
                raise Conflict("First release found an existing channel")
        else:
            expected = object_for(ACTIVE, previous / ACTIVE, mutable=True)
            if not matches(current, expected):
                raise Conflict("Published channel changed from the authenticated baseline")
    # Immutable binaries/public key first; mutable indexes next; InRelease last.
    for item in sorted(objects, key=lambda item: item.cache != IMMUTABLE):
        if item.cache == IMMUTABLE:
            ensure_immutable(store, item)
        else:
            ensure_mutable(store, item)
        public_verify(item)
    if not already_current:
        try:
            store.put(active, none_match=current is None, match_etag=None if current is None else current.etag)
        except Uncertain:
            inspect_after_write(store, active)
        else:
            inspect_after_write(store, active)
    if not matches(store.inspect(ACTIVE, 32768, retain=True), active):
        raise Conflict("Channel changed before acknowledgement")
    public_verify(active)
    return active.sha256


def latest_manifest(snapshot, keyring, fingerprint, commit, now=None):
    apt.require(isinstance(commit, str) and re.fullmatch(r"[a-f0-9]{40}", commit), "Expected exact source commit")
    state = apt.verify(snapshot, keyring, fingerprint, now)
    version, revision = state["version"].rsplit("-", 1)
    import native_release_tag
    tag = native_release_tag.tag_from_version(version)
    files = {}
    for arch in sorted(state["packages"]):
        item = object_for(f"linux/rquickshare-native_{state['version']}_{arch}.deb",
                          snapshot / f"linux/rquickshare-native_{state['version']}_{arch}.deb")
        files["deb" if arch == "amd64" else "deb_arm64"] = {"url": BASE + item.key, "sha256": item.sha256, "size": item.size}
    return {"schema": 1, "product": "rQuickShare", "version": version, "revision": int(revision),
            "tag": tag, "commit": commit, "files": {"linux": files}}


def publish_release(snapshot, keyring, fingerprint, store, commit, run_id, receipt,
                    previous=None, now=None, public_verify=public_readback):
    apt.require(type(run_id) is int and run_id > 0, "Expected positive release run")
    apt.require(not receipt.exists(), "Publication receipt already exists")
    manifest = latest_manifest(snapshot, keyring, fingerprint, commit, now)
    proofs = {}
    def verify_public(item):
        proofs[item.key] = public_verify(item)
        return proofs[item.key]
    publish(snapshot, keyring, fingerprint, store, previous, now, verify_public)
    files = [proofs[key] for key in sorted(proofs) if key.endswith((".deb", ".deb.sha256"))]
    with tempfile.TemporaryDirectory(prefix="rqs-latest-") as temporary:
        path = Path(temporary) / "latest.json"
        path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
        item = object_for("latest.json", path, mutable=True)
        ensure_mutable(store, item)
        latest = public_verify(item)
    result = {"schema": 1, "product": "rQuickShare", "base_url": BASE,
              "version": manifest["version"], "revision": manifest["revision"], "commit": commit,
              "latest": latest, "artifacts": [{"name": "native-package", "run_id": run_id, "files": files}]}
    receipt.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


class S3Store:
    """External SDK adapter; injected client keeps publication rules independently testable."""
    def __init__(self, client):
        self.client = client

    @staticmethod
    def failure(error, writing):
        response = getattr(error, "response", {})
        code = response.get("Error", {}).get("Code", "")
        status = response.get("ResponseMetadata", {}).get("HTTPStatusCode")
        if code in {"AccessDenied", "InvalidAccessKeyId", "SignatureDoesNotMatch"} or status in {401, 403}:
            return Denied("Storage access denied")
        if code in {"PreconditionFailed", "ConditionalRequestConflict"} or status in {409, 412}:
            return Conflict("Storage precondition failed")
        return Uncertain("Storage write unconfirmed") if writing else Unavailable("Storage read unavailable")

    def inspect(self, key, maximum, retain=False):
        try:
            response = self.client.get_object(Bucket=BUCKET, Key=PREFIX + key)
        except Exception as error:
            code = getattr(error, "response", {}).get("Error", {}).get("Code", "")
            if code == "NoSuchKey":
                return None
            raise self.failure(error, writing=False) from None
        body = response["Body"]
        try:
            length = response["ContentLength"]
            apt.require(type(length) is int and 0 <= length <= maximum, "Remote object exceeds expected bound")
            etag = response["ETag"]
            apt.require(isinstance(etag, str) and 2 < len(etag) <= 256 and etag.startswith('"') and etag.endswith('"')
                        and all(32 <= ord(c) < 127 for c in etag), "Invalid storage ETag")
            sha = hashlib.sha256()
            count = 0
            parts = []
            for chunk in body.iter_chunks(chunk_size=65536):
                count += len(chunk)
                apt.require(count <= length, "Remote body exceeds declared length")
                sha.update(chunk)
                if retain:
                    apt.require(maximum <= 32768, "Retained body exceeds metadata bound")
                    parts.append(chunk)
            apt.require(count == length, "Truncated remote object")
            return Remote(etag, sha.hexdigest(), count, b"".join(parts) if retain else None)
        except (KeyError, ValueError) as error:
            raise Unavailable("Invalid bounded storage response") from error
        except Exception:
            raise Unavailable("Storage body read failed") from None
        finally:
            body.close()

    def put(self, item, none_match=False, match_etag=None):
        apt.require(none_match != (match_etag is not None), "Exactly one conditional write is required")
        condition = {"IfNoneMatch": "*"} if none_match else {"IfMatch": match_etag}
        try:
            with item.source.open("rb") as source:
                # R2 documents Content-MD5 for single PUT; SHA256 trust comes from signed readback.
                transport = hashlib.file_digest(source, lambda: hashlib.md5(usedforsecurity=False)).digest()
                source.seek(0)
                self.client.put_object(Bucket=BUCKET, Key=PREFIX + item.key, Body=source,
                                       ContentLength=item.size, ContentType=item.content_type,
                                       CacheControl=item.cache, ContentMD5=base64.b64encode(transport).decode(),
                                       **condition)
        except Exception as error:
            raise self.failure(error, writing=True) from None

    def close(self):
        self.client.close()


def connect(account):
    apt.require(re.fullmatch(r"[a-f0-9]{32}", account), "Invalid Cloudflare account identity")
    access = os.environ.get("RQS_R2_ACCESS_KEY_ID")
    secret = os.environ.get("RQS_R2_SECRET_ACCESS_KEY")
    apt.require(bool(access) and bool(secret), "Explicit bucket-scoped R2 credentials are required")
    import boto3
    from botocore.config import Config
    client = boto3.client("s3", endpoint_url=f"https://{account}.r2.cloudflarestorage.com", region_name="auto",
                          aws_access_key_id=access, aws_secret_access_key=secret,
                          config=Config(signature_version="s3v4", retries={"total_max_attempts": 1},
                                        connect_timeout=10, read_timeout=30, s3={"addressing_style": "path"},
                                        request_checksum_calculation="when_required", response_checksum_validation="when_required"))
    return S3Store(client)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["plan", "publish"])
    parser.add_argument("--repository", type=Path, required=True)
    parser.add_argument("--keyring", type=Path, required=True)
    parser.add_argument("--fingerprint", required=True)
    baseline = parser.add_mutually_exclusive_group(required=True)
    baseline.add_argument("--first-release", action="store_true")
    baseline.add_argument("--previous", type=Path)
    parser.add_argument("--account-id")
    parser.add_argument("--commit")
    parser.add_argument("--run-id", type=int)
    parser.add_argument("--receipt", type=Path)
    args = parser.parse_args()
    store = None
    try:
        if args.command == "plan":
            objects, active = publication(args.repository, args.keyring, args.fingerprint, args.previous)
            print(json.dumps({"bucket": BUCKET, "immutable_objects": len(objects), "commit_key": active.key,
                              "commit_sha256": active.sha256, "signing": "external", "public_binding": BASE}, indent=2))
        else:
            # Local admission precedes credential discovery or any external client setup.
            publication(args.repository, args.keyring, args.fingerprint, args.previous)
            apt.require(args.account_id is not None, "Explicit account identity is required")
            apt.require(args.commit is not None and args.run_id is not None and args.receipt is not None,
                        "Publication requires source provenance and a receipt destination")
            latest_manifest(args.repository, args.keyring, args.fingerprint, args.commit)
            store = connect(args.account_id)
            publish_release(args.repository, args.keyring, args.fingerprint, store, args.commit,
                            args.run_id, args.receipt, args.previous)
            print("Published flat signed APT and verified latest manifest")
    except Exception as error:
        # SDK errors can contain credentials, URLs and service text; expose only a finite class.
        print("Native R2 operation failed:", type(error).__name__, file=sys.stderr)
        return 1
    finally:
        if store is not None:
            store.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
