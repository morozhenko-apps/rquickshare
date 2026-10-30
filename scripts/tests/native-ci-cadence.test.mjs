import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";

const read = path => readFileSync(new URL("../../" + path, import.meta.url), "utf8");

test("Given ordinary CI and a release Then Mode B is not a GitHub Actions workflow", () => {
  const preflight = read(".github/workflows/lint.yml");
  const gtk = read(".github/workflows/native-gtk-ui.yml");
  const release = read(".github/workflows/native-tag-publish.yml");
  for (const path of [
    ".github/workflows/mode-b.yml",
    ".github/workflows/coverage.yml",
    ".github/workflows/native-gtk-coverage.yml",
  ]) assert.equal(existsSync(new URL("../../" + path, import.meta.url)), false,
    "Periodic Mode B and coverage must not be GitHub Actions jobs: " + path);
  for (const workflow of [preflight, gtk, release]) {
    assert.ok(!workflow.includes("ci-core-test-groups.sh stress"));
    assert.ok(!workflow.includes("ci-native-gtk-test-groups.sh stress"));
    assert.ok(!workflow.includes("uses: ./.github/workflows/mode-b.yml"));
  }
  assert.ok(preflight.includes("scripts/ci-core-test-groups.sh stable"));
  assert.ok(gtk.includes("scripts/ci-native-gtk-test-groups.sh stable"));
  for (const script of [
    "scripts/ci-core-test-groups.sh",
    "scripts/ci-native-gtk-test-groups.sh",
  ]) {
    const source = read(script);
    assert.ok(source.includes("for attempt in $(seq 1 10)"));
    assert.ok(!source.includes('--skip "$test_name"'));
    assert.ok(!source.includes('--skip "$method"'));
  }
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
  assert.ok(ui.includes("scripts/ci-native-gtk-test-groups.sh stable"));
  assert.ok(preflight.includes("scripts/ci-core-test-groups.sh stable"));
  assert.ok(!ui.includes("scripts/ci-native-gtk-test-groups.sh stress"));
  assert.ok(!preflight.includes("scripts/ci-core-test-groups.sh stress"));

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

test("Given successful source review When packaging Then no source suite is repeated before signing", () => {
  const workflow = read(".github/workflows/native-tag-publish.yml");
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
  assert.match(publishStep, /native_apt\.py prepare/);
  assert.match(publishStep, /environment: apt-release/);
  assert.match(publishStep, /native_apt\.py verify/);
  assert.match(publishStep, /scripts\/native_r2\.py publish/);
  assert.equal((workflow.match(/native_apt\.py prepare/g) || []).length, 1);
});

test("Given a signing keyring path from CI Then it is normalized before strict key admission", () => {
  const workflow = read(".github/workflows/native-tag-publish.yml");
  const signingStep = workflow.split("      - name: Import encrypted signing subkey, forbid primary secret\n")[1]
    ?.split("      - name: Restore authenticated previous archive from private R2\n")[0];
  assert.ok(signingStep, "Signing key admission step must be present");
  assert.ok(signingStep.includes("from pathlib import Path"), "CI must import Path");
  assert.ok(signingStep.includes("apt.public_key(Path(sys.argv[1]), sys.argv[2])"),
    "The Python API requires a Path rather than the raw argv string");
  assert.ok(signingStep.includes("scripts/native_ci_signing_key.py"),
    "The secret subkey-only authorization gate must remain intact");
  assert.ok(!signingStep.includes("apt.public_key(sys.argv[1],"),
    "A raw argv string must never reach the pathlib-only keyring validator");
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
