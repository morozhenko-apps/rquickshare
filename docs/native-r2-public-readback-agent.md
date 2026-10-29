# Native R2 public readback: explicit verifier User-Agent

2026-10-29. MODE A narrow release-verification correction, independent of GTK language work.

## Evidence and root cause

The earlier CFVigil release-readback problem was Cloudflare HTTP 403 / error 1010 rejecting Python urllib's default User-Agent during **public HTTP readback**, even after the S3-side R2 upload succeeded. RepoTicker encountered the same default-client pattern. This does not establish an R2 write failure or a need to change signed PUT requests.

The current rQuickShare `scripts/native_r2.py::public_readback` uses `urllib.request.urlopen(BASE + item.key, timeout=45)`, without a declared User-Agent. It therefore has the *same vulnerable request shape*. No live rQuickShare 403 has been measured and production publication is not authorized.

## Scoped correction and invariants

Send `urllib.request.Request(BASE + item.key, headers={"User-Agent": "rQuickShare-ReleaseVerifier/1.0"})` to the existing injectable opener. Do not change URL origin/path, S3 credentials, bucket, upload metadata, TLS, signing, immutable object semantics, timeouts, cache, retry policy or release state. Keep the HTTP 200, bounded read, declared exact byte count and SHA-256 requirements. Unknown status including 403 must remain a failure; never bypass Cloudflare error checks or accept a private-only readback as a public success.

## Final tests / negative matrix

- Positive: for each verified object, record actual Request URL, exact User-Agent, timeout 45 and byte-for-byte hash/size.
- N1: no unrecognized/ambiguous HTTP parameters.
- N2: reject overlong/truncated/changed response bodies.
- N3: 206/301/403/404/429/5xx remain rejected; HTTP connection errors propagate.
- N4: original public verification remains independent of conditional S3 writes; no new retries or duplicate publication.
- N7/N10: no credentials or private headers in public Request; never record signed tokens in logs.
- N6/N8/N9/N11/N12: locale, daemon settings, interaction, monetization, local disk remain owned by their existing tests and unaffected.
- Final gate: run `python3 -m unittest discover -s scripts/tests -p 'test_native_*.py' -v`, native package/preflight gates where applicable, report actual CI separately. No live R2 call or publish before explicit release approval.

Rollback only this public-readback Request adapter and its tests. Keep `master`, release tags, signed APT snapshots and R2 objects untouched.

## Implemented source checkpoint (unverified CI)

`scripts/native_r2.py` now uses an explicit `urllib.request.Request` with `User-Agent: rQuickShare-ReleaseVerifier/1.0` and the original public BASE URL, timeout and strict SHA-256 / exact byte-count verification. `scripts/tests/test_native_r2.py` checks Request type, URL, UA, 45-second timeout, absence of Authorization and retains negative 206/301/403/404/429/500/503, truncated, extra and corrupted bytes. No actual download, upload, live 403 reproduction or CI test-suite run is asserted. Dev source checkpoint `a90ccb3a72518ba8f5589dad83b6ddf56ee55683`; signed-release publication remains unapproved.
