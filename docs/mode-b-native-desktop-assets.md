# Mode B: native desktop asset contracts

## Scope and starting evidence

Primary mode: B. Starting `dev`: `b73ecb568e156e9f4f884810394b8a9a2d65eb95`.
Preflight [40778096739](https://github.com/morozhenko-apps/rquickshare/actions/runs/40778096739)
passed for the executable package-check repair `99cb0df77cf0076ed3f7ffe1eddde259b81582c0`.
The repaired package workflow has not been dispatched. Its previous failed run
did not produce a candidate. Browser sign-in and a workflow-dispatch client are
unavailable in this execution environment; do not substitute a rerun of the old
SHA or change the manual-only workflow trigger.

Continue the existing package-validation work while this dispatch remains
blocked. The existing desktop asset tests exercise selected mutations, but do
not enumerate every localized field, unsafe character or parser ambiguity.
This slice closes that source-test inventory, without changing package bytes,
production code, versions, launch commands, consent or service ownership.

## Stages 1-3: framework, risks and complete scoped inventory

Use Python standard-library `unittest` and the real `native_deb` parser/checker.
No mocked subject, network, desktop session, process launch or sleep is needed.
Tests belong to `scripts/tests/test_native_desktop_assets.py`, selected both by
ordinary Preflight discovery and the existing `test_native_*.py` package gate.

| Artifact | Risk / level | Inventory and expected new methods |
| --- | --- | --- |
| `read_desktop_entry` | P1, unit with temporary files | Duplicate options in every shipped section/key; duplicate sections; malformed, unreadable and invalid-UTF-8 files: 2 methods |
| `validate_localized_desktop_entries` section/field contract | P0, unit | Every fixed field missing/modified, extra fields; each section layout; each missing/unsupported locale: 3 methods |
| Translation safety | P1, unit | All 20 locale keys in all 3 translated fields; valid Unicode; blank/padded, each forbidden control/bidi character, percent/backslash: 4 methods |
| Dolphin actionable caption relationship | P1, unit | Every locale's attribution, header ellipsis and exact action relationship: 1 method |

Total: 10 new named methods. Matrices count subcases separately from methods.
The 20 POSIX variants include `zh_TW` and `zh_HK`; they do not mean 20 different
languages. Expected names/fields/locales and prohibited code points are defined
independently in tests, rather than copied from a production allowlist.

## Stage 4: exhaustive scoped branch map

1. Parser: valid input; duplicate option in each of 3 sections; duplicate section;
   missing/broken section header; malformed option; missing file; invalid UTF-8.
   `interpolation=None` must preserve percent/backslash until validation.
2. Application sections: exact single group vs missing, renamed or extra group.
   Service sections: exact ordered pair vs missing, renamed, extra or reordered.
3. Each of the 19 fixed keys: present/exact vs missing or changed. All 3 sections
   reject extra keys. KDE minimum/maximum URL values additionally exercise
   zero, negative, adjacent values, leading zero, float and padded spelling.
4. Each of 60 translated fields: present vs absent. All 3 sections reject each
   unsupported locale/script key, including generic/Brazilian Portuguese,
   Simplified Chinese and English locale aliases.
5. Nonempty/trimmed translation vs empty, whitespace-only and left/right padding.
6. Each C0 character U+0000..U+001F, DEL, U+202A..U+202E and U+2066..U+2069
   rejected in each translated field for every locale. Valid multilingual
   Unicode, internal ASCII space/tilde, nonbreaking space and narrow
   nonbreaking space remain accepted under the existing contract.
7. Percent or backslash anywhere in each translation rejected independently
   from the control-character and whitespace checks.
8. Every localized service header must retain literal `rQuickShare` and have no
   trailing ellipsis; action must equal that header plus exactly one ellipsis.

## Stage 5: Positive / N1-N12 matrix

`P` = planned; `NA` = not applicable for the reasons below. No empty cells.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Parser | P | P | NA | NA | NA | NA | P | P | P | NA | NA | NA | P |
| Sections/fixed fields | P | P | P | NA | NA | NA | P | P | NA | NA | NA | NA | NA |
| Locales/translation safety | P | P | P | NA | NA | NA | P | P | P | NA | NA | NA | NA |
| Dolphin caption relationship | P | P | P | NA | NA | NA | P | P | P | NA | NA | NA | NA |

N3 network, N4 repeat actions, N5 races, N9 interruption, N10 actual PII
disclosure and N11 billing are outside this pure metadata slice. N12 covers
parser file-read error propagation only, not filesystem durability or disk-full
behavior. N7 checks packaged launch/consent metadata, not runtime authorization.

## Stages 6-8: interactions, estimates and per-file coverage

Exhaustive interactions: 20 locales x 3 translated fields x each invalid
whitespace/control/escape; all locale x caption mutations; every field x
missing/changed; each section x unexpected key; every shipped option x duplicate.
Keep fixtures isolated per subcase so a prior invalid field cannot make a later
negative assertion pass accidentally. Assert the specific rejection class and
message where validation exposes them.

| Production file / functions | Covered existing behavior | New coverage | Remaining |
| --- | --- | --- | --- |
| `scripts/native_deb.py::read_desktop_entry` | Shipped assets; one duplicate localized key | All shipped duplicate keys/sections, malformed/read/encoding failures | No instrumented line/branch measurement |
| `scripts/native_deb.py::validate_localized_desktop_entries` | Real assets, 13 selected negative mutations, exact commands and selected locales | Complete scoped branch/interaction matrices above | Editorial translation quality and actual KDE rendering |
| `scripts/native_deb.py` other functions | Existing package tests | Unchanged; run together at final stable gate | Not claimed complete by this slice |
| `.github/workflows/native-package-smoke.yml` | Existing routing regression | No change | Actual current-source package build/installed acceptance |

## Stages 9-10: implementation and mutation expectations

Implement all ten methods before executing suites. Do not weaken production
checks to accommodate a fixture. Dropping a required-key/section check, allowing
missing locales, relaxing the URL limit, any forbidden-codepoint bound, percent
or backslash filtering, or the caption relationship must fail a dedicated
assertion. Duplicate-option tests must depend on the actual strict parser and
case-sensitive keys. Run a bounded source-mutation check only for these checks;
report executed mutants separately from reasoned expectations.

## Stages 11-12: final validation and rollback

At the final gate, run once:

```sh
python3 -m unittest discover -s scripts/tests -p 'test_native_de*.py' -v
bash scripts/ci-check-workflow-policy.sh
git diff --check
```

No known flaky tests are classified in this deterministic slice. Core's
separate race-sensitive suite and installed KDE/Android checks are not executed
by these commands. Submit one executable test commit to `dev` for ordinary
Preflight, with documentation committed separately. Stop after the CI handoff;
do not poll or add a second build while it runs.

Rollback is removal of the single test file; production behavior and packaging
identity remain stable. Scoped completion requires all planned assertions and
stable checks. Repository-wide Mode B and installable-candidate acceptance remain
open until their own evidence exists. No release or publication is authorized.

## Implemented checkpoint and execution evidence

All 10 planned methods are implemented in the single new test file. No
production function, packaged asset, workflow, version or dependency changed.
The original 9 Debian tests and the 10 new asset tests passed together:

```text
python3 -m unittest discover -s scripts/tests -p 'test_native_de*.py' -v
Ran 19 tests in 3.154s
OK
exit status: 0
```

`bash scripts/ci-check-workflow-policy.sh` also exited 0. Local Git diff
whitespace validation passed. This execution uses Python 3.12.14 and real
Debian archive tooling for the existing package fixtures; it does not build or
execute the application. Agent-owned terminal commands exited normally.

The new methods enumerate 82 positive cases (80 locale/Unicode combinations,
the unchanged shipped baseline, and literal case-sensitive parsing) and 4,114
negative table cases. Counts below assign each negative to one primary category
to avoid double-counting overlapping security/locale risks:

| Primary category | New cases | Derivation |
| --- | ---: | --- |
| Positive | 82 | 20 locales x 4 Unicode texts, baseline, literal parser |
| N1 validation | 1,242 | 88 fixed/extra-field, 11 group, 780 blank/padded, 360 escaped-field, 3 malformed-file cases |
| N2 limits | 18 | Min/max URL adjacent, negative, overflow and noncanonical spellings |
| N6 missing/ambiguous data | 172 | 60 missing locale fields, 30 unsupported aliases, 79 duplicate keys and 3 duplicate sections |
| N7 authority/presentation contract | 160 | 20 locales x 3 invalid headers and 5 mismatched actions |
| N8 Unicode | 2,521 | 20 locales x 3 fields x 42 forbidden code points, plus invalid UTF-8 |
| N12 read failure | 1 | Missing file propagates `FileNotFoundError` |

N3/N4/N5/N9/N10/N11 are not applicable to this scoped metadata checker; their
runtime domains remain outside its completion claim. No known-flaky group is
defined for these deterministic tests, and none was silently skipped or
reclassified. Core race-sensitive tests were not executed here.

A bounded inline Python mutation harness compiled one changed copy of
`scripts/native_deb.py` at a time in memory, retained its actual source path for
asset loading, and ran the relevant new method against that real modified
module. Repository production bytes were never changed. **19/19 mutants were
killed**, with no surviving mutant in this selected set:

| Mutated guard | Mutants killed | Detecting new test area |
| --- | ---: | --- |
| Application and Dolphin group checks | 2 | Group layout matrix |
| Required field/locale set and fixed launch values | 2 | Locale and fixed-field matrices |
| Nonempty/trimmed translation | 1 | Blank/padded matrix |
| C0 upper bound, DEL, both bidi and both isolate endpoints | 6 | Every forbidden character in every translated slot |
| Percent and backslash filters | 2 | Escape position matrix |
| Attribution, header ellipsis and action equality | 3 | Per-locale caption matrix |
| Case-sensitive keys, strict duplicates, literal interpolation | 3 | Real parser fixtures |

Some structural/parser mutants produce explicit test errors rather than
assertion failures; both are reported as killed, not as baseline failures.
This is a targeted mutation result, not a repository-wide mutation score or
instrumented branch/line percentage. Coverage tooling was unavailable locally.

Scoped inventory, branch, boundary and interaction implementation: complete.
Local stable gate: PASS. Remote source CI: pending submission/verification.
Repository-wide Stage 12 and physical/package acceptance: OPEN. The manual
candidate build still needs an authenticated dispatch on the current `dev`:

```sh
gh workflow run native-package-smoke.yml --ref dev -R morozhenko-apps/rquickshare
```

Both historical task branches were checked with
`git rev-list --left-right --count dev...origin/<branch>` at the starting tip:
`agent/locale-reveal-notices-20261027` returned `85 0` and
`feature/native-text-review-i18n-20261026` returned `160 0`. Neither has unique
commits to merge. No branch deletion was performed; the owner requested its
command. Master, release tags and publication were not touched.
