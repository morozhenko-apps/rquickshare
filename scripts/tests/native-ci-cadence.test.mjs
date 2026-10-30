import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const read = path => readFileSync(new URL("../../" + path, import.meta.url), "utf8");

test("Given release Mode B When inspected Then stable suites do not run ten times", () => {
  const mode = read(".github/workflows/mode-b.yml");
  const release = read(".github/workflows/native-tag-publish.yml");
  for (const selector of [
    "scripts/ci-core-test-groups.sh stable",
    "scripts/ci-core-test-groups.sh stress",
    "scripts/ci-native-gtk-test-groups.sh stable",
    "scripts/ci-native-gtk-test-groups.sh stress",
  ]) assert.ok(mode.includes(selector), selector);
  assert.ok(!mode.includes("for attempt in $(seq 1 10)"));
  assert.ok(mode.includes("pnpm exec vitest run"));
  assert.ok(mode.includes("unittest discover -s scripts/tests"));
  assert.ok(mode.includes("needs: [full_suite, native_gtk]"));
  assert.ok(release.includes("needs: [admission, mode-b]"));
  assert.ok(release.includes("environment: apt-release"));
});

test("Given native GTK selectors When inspected Then test admission is exact and fail closed", () => {
  const script = read("scripts/ci-native-gtk-test-groups.sh");
  const ui = read(".github/workflows/native-gtk-ui.yml");
  const preflight = read(".github/workflows/lint.yml");
  for (const testName of [
    "given_external_process_failures_when_cancelled_or_malformed_then_owned_children_are_reaped",
    "given_actual_sandbox_helper_when_decoding_fixture_then_exact_rgba_is_verified",
    "given_signal_and_close_racing_when_both_complete_then_shutdown_is_idempotently_cancelled",
  ]) assert.ok(script.includes(testName), testName);
  assert.ok(script.includes("grep -Fxc"));
  assert.ok(script.includes('target_args=(--lib)'));
  assert.ok(script.includes('target_args=(--bin rquickshare)'));
  assert.ok(script.includes('"${cargo_cmd[@]}" "${target_args[@]}"'));
  assert.ok(!script.includes('"--$target"'));
  assert.ok(script.includes("--exact"));
  assert.ok(script.includes("test result: ok. 1 passed; 0 failed; 0 ignored;"));
  for (const group of ["stable", "stress"]) {
    assert.ok(ui.includes("scripts/ci-native-gtk-test-groups.sh " + group));
    assert.ok(modeHas(group), group);
    assert.ok(preflight.includes("scripts/ci-core-test-groups.sh " + group));
  }
  function modeHas(group) {
    return read(".github/workflows/mode-b.yml").includes("scripts/ci-native-gtk-test-groups.sh " + group);
  }
});

test("Given broken stdin and oversized decoder stdout Then output bounds win deterministically", () => {
  const decoder = read("native_desktop/src/preview.rs");
  const regressions = read("native_desktop/src/preview/tests.rs");
  assert.ok(!decoder.includes("tokio::try_join!(write, read)"));
  assert.ok(decoder.includes("tokio::pin!(write, read)"));
  assert.ok(decoder.includes("let output = read.await?"));
  assert.ok(decoder.includes("write_result?"));
  assert.ok(regressions.includes("os.close(0)"));
  assert.ok(regressions.includes("input.resize(MAX_ENCODED, 0)"));
  assert.ok(regressions.includes("probe.stdout.len() == MAX_OUTPUT + 1"));
});
