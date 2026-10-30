import assert from "node:assert/strict";
import test from "node:test";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const classifier = fileURLToPath(new URL("../ci-pr-scope.sh", import.meta.url));

function git(cwd, ...args) {
  return execFileSync("git", ["-c", "commit.gpgsign=false",
    "-c", "user.name=Test Fixture", "-c", "user.email=fixture@invalid.example",
    ...args], { cwd, encoding: "utf8", stdio: "pipe" }).trim();
}

function exercise(paths, event = "pull_request", emptyMerge = false) {
  const cwd = mkdtempSync(join(tmpdir(), "rquickshare-pr-scope-"));
  try {
    git(cwd, "init", "-q", "-b", "main");
    writeFileSync(join(cwd, "initial.txt"), "base\n");
    git(cwd, "add", "-A");
    git(cwd, "commit", "-qm", "base");
    if (event === "pull_request") {
      git(cwd, "checkout", "-qb", "changes");
      for (const filename of paths) {
        const target = join(cwd, filename);
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, "changed\n");
      }
      if (!emptyMerge) git(cwd, "add", "-A");
      git(cwd, "commit", ...(emptyMerge ? ["--allow-empty"] : []), "-qm", "changes");
      git(cwd, "checkout", "-q", "main");
      git(cwd, "merge", "--no-ff", "-qm", "test merge", "changes");
    }
    const outputFile = join(cwd, "ci-output.txt");
    const result = spawnSync("bash", [classifier], {
      cwd,
      encoding: "utf8",
      env: { ...process.env, GITHUB_EVENT_NAME: event, GITHUB_OUTPUT: outputFile },
    });
    return {
      status: result.status,
      stderr: result.stderr,
      outputs: result.status === 0
        ? Object.fromEntries(readFileSync(outputFile, "utf8").trim().split("\n").map(line => line.split("=")))
        : null,
    };
  } finally {
    rmSync(cwd, { recursive: true, force: true });
  }
}

test("Given PR changes When scope is classified Then only applicable suites run", () => {
  const cases = [
    { paths: ["docs/guide.md", "README.md", "internal/docs/note.txt"], preflight: "false", gtk: "false" },
    { paths: ["native_desktop/src/main.rs"], preflight: "true", gtk: "true" },
    { paths: ["core_lib/src/lib.rs"], preflight: "true", gtk: "true" },
    { paths: ["app/main/src/index.ts"], preflight: "true", gtk: "false" },
    { paths: [".github/workflows/native-tag-publish.yml"], preflight: "true", gtk: "true" },
    { paths: [".github/workflows/lint.yml"], preflight: "true", gtk: "true" },
    { paths: ["scripts/ci-pr-scope.sh"], preflight: "true", gtk: "true" },
    { paths: ["packaging/release.toml"], preflight: "true", gtk: "false" },
    { paths: ["native_desktop/untrusted\nname.rs"], preflight: "true", gtk: "true" },
    { paths: ["docs/readme.md", "core_lib/src/lib.rs"], preflight: "true", gtk: "true" },
  ];
  for (const { paths, preflight, gtk } of cases) {
    const result = exercise(paths);
    assert.equal(result.status, 0, JSON.stringify({ paths, stderr: result.stderr }));
    assert.deepEqual(result.outputs, { preflight, gtk }, JSON.stringify(paths));
  }
});

test("Given an explicit marker push or dispatch When classified Then run full ordinary checks", () => {
  for (const event of ["push", "workflow_dispatch"]) {
    const result = exercise([], event);
    assert.equal(result.status, 0);
    assert.deepEqual(result.outputs, { preflight: "true", gtk: "true" });
  }
});

test("Given an unsupported event or unchanged PR merge Then no successful admission is produced", () => {
  for (const result of [exercise([], "schedule"), exercise([], "pull_request", true)]) {
    assert.notEqual(result.status, 0);
    assert.equal(result.outputs, null);
  }
});
