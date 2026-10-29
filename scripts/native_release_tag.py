#!/usr/bin/env python3
"""Canonical independent Native release source tag ↔ product SemVer contract."""
import argparse
import re
import sys

TAG_PREFIX = "native/v"
SEMVER = re.compile(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)", re.ASCII)


def tag_from_version(version: str) -> str:
    if not isinstance(version, str) or SEMVER.fullmatch(version) is None:
        raise ValueError("Invalid Native product SemVer")
    return TAG_PREFIX + version


def version_from_tag(tag: str) -> str:
    if not isinstance(tag, str) or not tag.startswith(TAG_PREFIX):
        raise ValueError("Not a Native release tag")
    version = tag[len(TAG_PREFIX):]
    if SEMVER.fullmatch(version) is None:
        raise ValueError("Invalid Native release tag")
    return version


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", choices=("version", "tag"))
    parser.add_argument("value")
    args = parser.parse_args()
    try:
        print(version_from_tag(args.value) if args.operation == "version" else tag_from_version(args.value))
    except ValueError as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
