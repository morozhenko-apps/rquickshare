# Pixel-origin Quick Share discovery latency: external scan-time measurement

Date: 2026-10-24. Primary issue [#18](https://github.com/morozhenko-apps/rquickshare/issues/18). This supersedes the narrow idea that daemon-only runtime milestones can measure the user's observed time to *first Android UI discovery*. It does not replace the existing [cold discovery phase plan](native-cold-discovery-timeline-2026-10-24.md) or the [active backlog](active-backlog.md).

## Operator evidence and correct meaning of timings

Latest installed diagnostic trace across two separate daemon lifetimes yielded **6/6 Pixel→KDE explicit declines displayed as Declined**, though an earlier build yielded 3/3 Failed. The separate unresolved issue is **slow initial Android discovery** and a transient GTK premetadata placeholder tracked in #22.

User-provided fixed stages (elapsed milliseconds from start of each **Linux process**, not from Android search):

| Host marker | First daemon | Second daemon |
| --- | ---: | ---: |
| runtime_core_ready | 54 | 21 |
| runtime_discovery_worker_ready | 55 | 24 |
| mdns_browse_started | 55 | 25 |
| gatt_header_ready | 110 | 72 |
| first_mdns_route_admitted | not observed | 10727 |
| first visible (manual Receive) | 2427 | 14749 |
| first advertisement_started after Visible | 2454 | 14765 |
| first gatt_slot0_read | 55644 | 32336 |
| visible→advertisement_started | **27 ms** | **16 ms** |
| first advertisement_started→first gatt_slot0_read | **53190 ms** | **17571 ms** |

Interpretation: Core became ready in <60ms for both; first explicit visibility activation was **not at process start** and varied (2.427s vs 14.749s). Once manual visibility occurred, `advertisement_started` was 27/16ms later. Linux then observed a first GATT read 53.19/17.571 seconds after this advertisement. This is a **cross-marker interval, not proven Google Pixel scan latency**: Android scanning might start after the Linux advertisement, the Pixel might have listed the PC earlier/later, a GATT read could be from another nearby peer, and initial routes might be unrelated. The second daemon reported a reachable mDNS route at 10.727s, which must NOT be assigned to the Pixel without route proof. No claim that the Pixel was continuously searching for the whole interval. The GTK countdown `Visible to nearby devices · 0:12` means the **remaining** temporary visibility window, not measured discovery duration.

## Why Android-side evidence is essential

Our Linux process cannot observe the moment the **user opens Quick Share's sender/search UI**, the Google service's internal BLE scan scheduling, or the exact moment rQuickShare first appears on the Android screen. Neither `advertisement_started` nor `gatt_slot0_read` asserts that UI state. Android Quick Share's internals are not an app we control; `adb logcat` may expose limited scan callbacks and has no stable documented Google Quick Share event tag. Do not interpret absence of a guessed logcat tag as absence of scanning.

## Minimal zero-app Pixel instrumentation

Record only the **Pixel screen**, from *before* opening Quick Share's outbound chooser until Linux is first listed. Use Android's built-in Screen Record quick-setting (no Android dev setting or app install needed). For a reproducible trial:

1. Set the computer's manual Receive to `Hidden`, keep radios and networks unchanged. Start Pixel screen recording locally. Open Quick Share's Send picker from a real share intent (same tiny test file across attempts), observing when the picker/search becomes active.
2. Enable `Receive` manually on Linux at a clearly visible moment relative to the recording, or do so first for a separate experiment. Record both **Android scan UI opened** and **Linux first visible** times from video frames. Note if Linux appears before/after a visible advertiser start, and whether another device is listed. Do not send file or reveal PIN just to measure discovery.
3. Repeat as a controlled comparison in **warm** operation without restarting daemon or Quick Share, then restart only one component for **cold** trial. Do not rotate all conditions simultaneously; at least one cold and one warm attempt, ideally three each.
4. Record only timing summary: `t_android_scan_open`, `t_linux_visible_on_android`, whether *manual Receive* preceded scan, startup/restart condition, and whether any GATT marker happened during the window. Videos remain local unless owner voluntarily shares an edited/cropped reproduction. Do not collect screenshots of contact lists or private notifications.

**Optional ADB screen record (supported by Android):** `adb devices`, then `adb shell screenrecord --bit-rate 4000000 --time-limit 120 /sdcard/rqs-discovery.mp4`; use Ctrl+C, `adb pull /sdcard/rqs-discovery.mp4 ./`. This is video evidence, **not** Google internal diagnostic logs. Native UI recorder is equivalent for phase-duration purposes. Published platform documentation: https://developer.android.com/tools/adb#screenrecord .

To correlate computer phases, use already installed opt-in `RQUICKSHARE_RECEIVER_TRACE=1` **before native daemon starts**; extract *only allowlisted* `runtime_*`, `visible`, `advertisement_*`, `gatt_*`, `mdns_browse_started`, `first_mdns_route_admitted` and elapsed_ms in process order. Since the host event clock and video frame time origin are different, do not subtract one timestamp directly from the other without a deliberate synchronized event. If stronger correlation becomes necessary, add a bounded test-harness UI marker/command on the Pixel/desktop, not raw GMS traffic scraping.

## Decision matrix after evidence

- If scan starts well **after** the advertiser event, the host advertisement→GATT interval is uninformative; investigate user/app scan-start timing before changing Linux code.
- If scan is already active but Linux appears only after tens of seconds while the host continuously advertises, investigate advertisement cadence/rotation, BLE service hash, Android scanning filters, Bluetooth permissions/power scheduling and GATT state. Establish the source contract first. Avoid blind retries/timeouts.
- If Linux appears promptly in Pixel but authenticated connection/metadata is slow, investigate BLE Weave handshake, GATT, BWU and lifecycle separately; `Transfer · 0 files` is #22 premetadata display, **not itself device discovery evidence**.
- If Linux first appears only on warm runs, compare cold vs warm advertisement and scan state plus device restart; do not claim network or BLE root cause without evidence.

Do not install diagnostic Quick Share clones to infer Google scan behavior, do not require root, and do not ask the owner for unfiltered `adb logcat` or raw system Bluetooth logs containing device data. This is a measurement plan only; **no production code changes**.

## Completion gate

- [x] Previous host trace interpreted without assuming unknown Pixel search start.
- [x] Android screen evidence method selected and documented.
- [ ] Owner supplies at least one cold and one warm **Pixel UI** scan-to-device-appearance timing (screen recording or stopwatch), with host marker context if possible.
- [ ] Evidence-driven root-cause and regression tests; physical installed acceptance.
- [ ] #18 remains open until first-discovery timing/reliability and bilateral decline gates; master, tag and R2 untouched.
