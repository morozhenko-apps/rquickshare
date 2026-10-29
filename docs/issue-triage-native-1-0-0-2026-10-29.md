# Native 1.0.0 issue triage and closure evidence — 2026-10-29

## Mode and scope

Primary MODE B: read-only repository/source/test/CI evidence audit and GitHub issue administration, **no production code changes**. Owner requested to eliminate purposeless open issues, preserving explicit frozen issues **#8 and #16 unchanged** (including their issue state, text and implementation). A closed issue means its original actionable claim is fixed, accepted by the owner's currently installed candidate, superseded by a precise current task, or invalidated as an unmeasured hypothesis. It does **not** mean every transport edge case has passed on all physical peers. Preserve historical observations in issue bodies, including failures and redacted diagnostics. Do not erase negative tests.

Source basis: dev contains native GTK 1.0.0-1 source after accepted installed 0.9.0-4, frozen Core transfer implementation unchanged. Exact code gate [GTK #40943358536](https://github.com/morozhenko-apps/rquickshare/actions/runs/40943358536) and [Preflight #40943358595](https://github.com/morozhenko-apps/rquickshare/actions/runs/40943358595), created **2026-10-29 19:48:18 Lisbon** on source a941e468, terminal SUCCESS at **19:50:17** and **19:54:32** respectively. Later only release-workflow changes, documentation, and disclosure tests; a separate final 1.0.0 tagged/package gate remains required. Physical acceptance: owner expressly reported installed 0.9.0-4 5/5 PASS for About, Native launcher, Russian empty card, RU/EN/PT live switching, bidirectional Pixel/KDE text and image and ordinary Decline/Cancel; an earlier 0.9.0-3 also passed 5/5. This is not a claim about committed BWU Cancel, initial BLE attempt or multiple devices.

## Triage decisions — candidates pending confirmation and state changes

| # | Classification | Evidence and disposition |
|---|---|---|
| 9 | Source defect fixed | Injectable real mDNS discovery event and TCP reachability seam, exact-source prior successful test CI 40093504760; no unmocked real SDK experiment claim. Close completed for testability work; future interoperability belongs to 23. |
| 11 | Source defect fixed | select_lan_ipv4 rejects unroutable special IPv4, tests and exact-source 40093504760 PASS; no assertion every VPN route works. Close completed. |
| 12 | Source defect fixed | replace_discovery_with cancels and joins previous worker, SDK shutdown joined, tests and 40085426478/77 PASS; close completed. |
| 13 | GTK functionality implemented | Authoritative pending request count/attention and exact sender focus tests, native GTK 40037554078 PASS; owner recent installed tray/inbound acceptance PASS. Multi-device physical stress unproven; close implemented with that scope, reopen for a concrete present-day failure. |
| 17 | Feature completed to agreed scope | All 20 static GTK catalogs including About, no full 20-language human approval (expressly rejected by owner), GTK/Python catalog checks and installed RU/EN/PT live acceptance; close completed. Native 1.0.0 release tests separate. |
| 18 | Superseded | Original cold-discovery delay was narrowed by video to first post-discovery BLE Weave connection; the confirmed remaining failure has a direct owner #23. Physical Decline/Cancel accepted in later 0.9.0-4. Close duplicate/superseded by #23 rather than declare first connection fixed. |
| 19 | Source defect fixed | mDNS producer now emits fullname/id/present:false tombstones, NativeRuntime retires exactly matching routes, exact-source 40080949959/968 PASS. Owner has not run extended H01/H15 physical same-name sequence; source mismatch is resolved. Close completed for this specific defect; new failure requires current-build reproducible evidence. |
| 21 | Product behavior accepted | StartupSettings defaults Invisible; temporary Receive does not automatically rearm on startup; user observed manual Receive/expiry/restart PASS. settings_service applies visibility only when values differ from active policy or explicit quick operation; potential future unexpected visibility replay needs a concrete reproduction, not indefinite open risk inference. Close completed. |
| 22 | GTK bug fixed | Unavailable payload displays Connecting…/Waiting for details rather than Transfer · 0 files; pure/widget tests and successful source CI, later installed 0.9.0-4 all main panel functions PASS. Close completed. |
| 25 | Historical symptom not reproducible in accepted candidate | Original Pixel “Success” on cancelled partial inbound was observed in older diagnostic; latest owner reports 0.9.0-3 and 0.9.0-4 bilateral Cancel/Decline correct with no false Success in observed attempts; code has explicit cancel state/publication checks. Close completed under tested normal paths, not a universal proof of every migration race; #26 contains exact migration fixture. |
| 26 | Root cause fixed in source | The specific false local_cancel_flushed when deferred_upgrade_writes retained is guarded with explicit failed-write error; worker keeps writer ownership through committed migration; bwu_cancel_tests covers four committed phases and failed writer. Current source CI green. Ordinary Pixel Cancel user PASS; no hardware proof of exact committed BWU race. Close completed for original false-positive trace/queued-frames defect, reopen with a new trace if the committed race manifests in current release. |
| 24 | Unsubstantiated performance hypothesis | Old debug candidate showed subjectively slower GTK progress without comparable bytes/time/transport samples. No measured production throughput regression can be demonstrated from existing issue evidence. Close not_planned (insufficient reproducible performance target), with explicit instructions to file current-version measurements before optimization; do not claim performance fixed. |

## Preserved actionable open work

- **#23** REAL confirmed historical Pixel first-try post-discovery BLE Weave Connecting stall (~50s); first/second-attempt behavior still lacks paired static-stage logs from a current installed release. Do not change timers/speculatively retry. Goal: produce a current-version dual-attempt opt-in redacted GATT/Weave/UKEY2 stage comparison and resolve the demonstrated stop stage, or explicitly document inability to reproduce. No duplicate generic cold-discovery issue (#18).
- **#27** Planned Windows native port, no Windows executable, DPI/geometry source checks or Windows 11 transfer evidence. Post-Linux 1.0.0 platform milestone, not a Linux product bug. Do not claim Windows supported.
- **#14** Actual Linux native/v1.0.0 public release delivery: exact tagged source Mode B, 1.0.0-1 verified immutable package, protected branch and annotated tag, signer-only apt-release, public signed R2/APT and HTTPS readback. Remains open until actual publication receipt; owner explicitly authorizes shipping with frozen #8/#16 unresolved and disclosed.
- **#8, #16** explicit owner-accepted, untouched known limitations that accompany release. Leave both open.

## Required issue hygiene

Keep original issue bodies/history; add issue-specific closure comment with code/test/owner evidence and explicit untested limits. Close issues individually with GitHub state_reason completed, duplicate or not_planned as appropriate. For remaining #23/#27/#14 add a *single concrete next-action/exit criterion* comment, not a duplicate open issue. Do not modify frozen #8/#16. Finalize counts and note that issue tracker hygiene does not replace the release's mandatory signing/immutable artifact gate.

## Executed issue changes (terminal, not a proposal)

All 12 individual closures have committed GitHub issue comments with source/test/owner evidence and explanatory limits:
- **Completed:** #9, #11, #12, #13, #17, #19, #21, #22, #25, #26.
- **Superseded/duplicate:** #18 → current reproducible post-discovery #23.
- **Not planned without a measurable regression:** #24, historical debug progress animation; future concrete throughput measurements should create a fresh performance issue rather than retaining an unbounded claim.
- **Open with specific exit criteria:** #14 (retitled to Native Linux 1.0.0 protected signed production release), #23 (first BLE Weave GATT/UKEY2 redacted stage trace and architectural fix), #27 (actual Windows native client, CI and physical Windows 11 acceptance).
- **Frozen, unchanged and explicitly accepted for release:** #8, #16.

Original issue descriptions/negative evidence were not deleted, and no transfer/network production code was changed. This MODE B issue audit is completed. No new test suite was invoked by documentation/issue administration; no public release tag, master update, signer/R2 mutation or artifact publication was performed. Closes do not silently assert 100% device interoperability, particularly the narrow committed BWU Cancel race.

Final backlog count expected: **5 open (#8, #14, #16, #23, #27)** versus initial **17**. Refresh live issue state if subsequent independent activity changes this list. #14 is the only required Linux distribution action remaining among these; the first-connection issue #23 is a disclosed/diagnostic reliability risk, and #27 is explicitly outside the Linux binary's platform claims.
