# Conditional distribution publication

Current production layout and accepted publication are described in
[flat distribution](FLAT_DISTRIBUTION.md). The canonical versioned DEB serves
both direct HTTPS and signed flat APT. No download Worker, binary latest copy,
public staging archive or extra publisher service is required by this layout.

## Admission and writes

Validate candidate and authenticated prior metadata before storage changes:
pinned signing authority, exact cleartext/index/package hashes and sizes,
source provenance, version/revision and non-downgrade rules. Network/auth failures
do not prove an absent channel. Read and retain exact current authority/ETags.

Use conditional PutObject for admitted writes: If-None-Match:* for creation,
If-Match with the exact observed opaque ETag for replacements. ETags are CAS
tokens, never cryptographic digests. Explicit publisher credentials must not
come from ambient SDK discovery or an untrusted build job. Disable write retries;
never replace these conditions with HEAD then unconditional upload.

Existing immutable bytes may be reused only after complete bounded hash/length
verification. Conflicting bytes or generations fail. An ambiguous immutable
write is accepted only after exact positive readback; absence does not prove a
previous request cannot still commit. Do not promote on unresolved writes.
An ambiguous final authority update likewise needs exact positive readback;
otherwise report uncertainty without blind replay, deletion or rollback.

## Ordering, readback and cleanup

Write/verify immutable platform files first. Replace Packages and Packages.gz,
then verified signed InRelease, then schema-checked latest.json last. Flat
multi-object indexes are not a transaction: transient mismatch must fail
signature/hash checks. Authority is fresh/no-store; immutable package objects
may be cached. Never force-cache missing/error responses as valid artifacts.

Public readback streams the entire expected body with bounded length/SHA-256;
HTTP status, object metadata or a one-byte probe cannot establish identity.
Only a completed provenance-bound receipt authorizes scoped temporary CI cleanup.
Preserve published artifacts, signatures, prior authority and unrelated products.

## Verification

Offline tests exercise real publisher logic with an external conditional-store
adapter and actual signed metadata: first/current/next/no-op, wrong authority,
tampering, conflicting creators/replacements, stale pointers, exact headers and
ordering, denied/failed/ambiguous I/O and stopped cleanup. A modeled SDK client
may use external Stubber responses; never mock the publication subject itself.
Source tests are separate from public availability, installed APT and owner
release approval. [Local signing](native-local-signing.md) and its protected-CI
subkey exception retain their own custody and provenance boundaries.

[Historical nested-archive design](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-r2.md) is retained in Git; its pool/by-hash snapshot
layout and initial external-only signing choice do not define current hosting.

<!-- Historical heading anchors retained for existing links. -->
<a id="native-apt-on-cloudflare-r2-with-external-signing"></a>
<a id="analysis-and-plan-before-code"></a>
<a id="frozen-ownership-and-publication-contract"></a>
<a id="verification-inventory-before-tests"></a>
<a id="primary-sources-inspected"></a>
<a id="current-state"></a>
<a id="completed-local-stages-and-measured-checks"></a>
<a id="operator-handoff"></a>
<a id="remaining-acceptance-and-recoverability"></a>
