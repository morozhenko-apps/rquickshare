import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";

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
  assert.ok(!mode.includes("\n  coverage:"), "Release Mode B must never run coverage");
  assert.ok(!mode.includes("llvm-cov"), "Release must not install instrumentation");
  const gtk = mode.split("\n  native_gtk:\n")[1];
  assert.ok(gtk, "Native GTK validation job must exist");
  assert.ok(!gtk.includes("needs: full_suite"), "Independent Core and GTK checks should run in parallel");
  for (const path of [
    ".github/workflows/coverage.yml",
    ".github/workflows/native-gtk-coverage.yml",
  ]) {
    assert.equal(existsSync(new URL("../../" + path, import.meta.url)), false,
      "Coverage is offline analysis, not another GitHub Actions workflow: " + path);
  }
  assert.match(mode, /workflow_dispatch:/);
  assert.doesNotMatch(mode, /workflow_call:|^  push:/m);
  assert.doesNotMatch(release, /mode-b|mode_b|\.\/\.github\/workflows\/mode-b\.yml/);
  assert.ok(release.includes("needs: admission"));
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

test("Given Mode B admitted one commit When packaging Then no source suite is repeated before signing", () => {
  const workflow = read(".github/workflows/native-tag-publish.yml");
  const modeB = read(".github/workflows/mode-b.yml");
  const packageStep = workflow.split("\n  package:\n")[1]?.split("\n  publish:\n")[0];
  const publishStep = workflow.split("\n  publish:\n")[1]?.split("\n  cleanup:\n")[0];
  assert.ok(packageStep && publishStep, "Package and publish jobs must exist");
  assert.match(packageStep, /needs: admission/);
  assert.doesNotMatch(workflow, /node --test scripts\/tests\/\*\.test\.mjs/);
  assert.doesNotMatch(workflow, /mode-b/);
  assert.match(packageStep, /runs-on: ubuntu-26\.04/);
  assert.match(packageStep, /desktop-file-validate/);
  assert.match(packageStep, /python3 scripts\/native_deb\.py build/);
  assert.match(packageStep, /actions\/upload-artifact@v6/);
  assert.match(packageStep, /native-packages\/\*\.deb\.sha256/);
  assert.doesNotMatch(packageStep, /cargo (?:\+stable )?(?:test|clippy)\b/);
  assert.doesNotMatch(packageStep, /unittest discover|pip install|native_apt\.py prepare/);
  assert.doesNotMatch(packageStep, /xvfb-run|dbus-run-session|bwrap --die-with-parent/);
  assert.match(modeB, /scripts\/ci-native-gtk-test-groups\.sh stress/);
  assert.match(modeB, /scripts\/ci-native-gtk-test-groups\.sh stable/);
  assert.match(modeB, /unittest discover -s scripts\/tests/);
  assert.match(publishStep, /native_apt\.py prepare/);
  assert.match(publishStep, /environment: apt-release/);
  assert.match(publishStep, /native_apt\.py verify/);
  assert.match(publishStep, /scripts\/native_r2\.py publish/);
  assert.equal((workflow.match(/native_apt\.py prepare/g) || []).length, 1);
});

test("Given normal dev changes Then only explicit verification marker or PR can launch CI", () => {
  const core = read(".github/workflows/lint.yml");
  const native = read(".github/workflows/native-gtk-ui.yml");
  for (const workflow of [core, native]) {
    assert.match(workflow, /push:\n    branches: \[dev\]\n    paths:\n      - 'ci\/run-preflight\.txt'/);
    assert.match(workflow, /pull_request:\n    branches: \[dev, master\]/);
    assert.match(workflow, /workflow_dispatch:/);
    const pushBlock = workflow.split("\n  push:\n")[1]?.split("\n  pull_request:\n")[0];
    assert.ok(pushBlock && !pushBlock.includes("master"), "A master push must not create redundant CI");
  }
  assert.doesNotMatch(core, /if: github\.event_name != 'pull_request'/);
});
