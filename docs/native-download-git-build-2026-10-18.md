# Historical download Worker verification

The old Worker/Git-build verification applies only to its recorded retired component and source. Its former ten-run campaign is not current CI or a deployment instruction. Current delivery uses the direct flat channel.

See [current distribution](FLAT_DISTRIBUTION.md), [tagged release](native-tagged-release.md),
[local signing](native-local-signing.md), [encrypted recovery](native-signing-backup.md)
and [bounded source CI](ci-pilot.md).

The historical test inventory remains below for provenance; it does not
reactivate the component or assert current coverage.

## Test inventory, branch coverage and N1–N12 review

| Production artifact | Risk / level | Existing test scenarios and required branch assertions |
| --- | --- | --- |
| `policy.ts::objectPolicy` | High, unit | Accepted immutable/mutable keys, versions/architecture/sidecars; wrong host, scheme, prefix, queries, credentials, path escape, encoding, size, unknown/private objects |
| `policy.ts::byteRange` | High, unit | Open/suffix/bounded ranges and minimum/maximum; invalid, reversed, multirange, overflow, unsatisfiable sizes |
| `policy.ts::matchesEtag` | Medium, unit | Matching/nonmatching strong and weak validators, wildcard, list, null and excessively long input |
| `worker.ts::serve`, `failure`, `valid`, `headers`, default fetch adapter | High, HTTP/contract | GET/HEAD/405/404; mutable freshness; ETag/304; range/206/416; invalid R2 lengths, missing/changed/throwing storage, fixed no-PII error and streaming |
| `wrangler.jsonc`, generated `Env` | Critical, build/contract | Strict types, generated-binding file stable, bounded route/name/R2 settings reviewed; bundle dry-run |
| workerd+R2+HTTP+APT bridge | Critical, integration | Local actual Worker and R2, approved public keys, signed test-only APT download, by-hash, wrong signing-key refusal, isolated filesystem and cleanup |

The positive/negative matrix for these existing scenarios maps N1 (bad input), N2
(bounds), N3 (read failures), N4 (ETag/re-read consistency), N5 (changed object
between `head`/`get`), N6 (invalid metadata), N7 (unauthorized methods/paths),
N9 (interrupted/read errors), N10 (path/secret redaction) and N12 (missing/corrupt
storage) to existing tests. N8 (locale/time) and N11 (monetization) are not
applicable to this stateless read-only projection. All identified branches must
remain under the existing tests; the CI change adds no new production branch.

Key interaction matrix: mutable/immutable × GET/HEAD × Range/If-Range/If-None-Match;
missing/changed/corrupt object × metadata/body; valid/invalid host/path × method.
Test inventory is 10 existing top-level test scenarios with table-driven subcases,
not 10 newly written tests. Per-file coverage map: `policy.ts` three exported
functions, `worker.ts` one exported serving function plus its three internal
helpers/default adapter, `wrangler.jsonc` one deployment contract. Exact numeric
branch coverage has not been measured; do not describe this as 100% coverage.
Mutation targets include changing an allowlist predicate, object maximum, range
bound, conditional validator, storage metadata check or cache policy.

[Original design and observations](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-download-git-build-2026-10-18.md) remain in Git history.
Concrete provider identifiers, private locations, grant inventories and
chronological operator receipts are not current public instructions.

<!-- Historical heading anchors retained for existing links. -->
<a id="native-download-worker-git-build-verification"></a>
<a id="initial-repository-scan-and-invariant-review"></a>
<a id="plan-and-acceptance-documentation-before-implementation"></a>
<a id="verification-receipts-and-follow-up"></a>
<a id="production-git-build-result-2026-10-18"></a>
