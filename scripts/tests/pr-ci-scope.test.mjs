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

function exercise(paths, event = "pull_request", emptyMerge = false, operation = "add") {
  const cwd = mkdtempSync(join(tmpdir(), "rquickshare-pr-scope-"));
  try {
    git(cwd, "init", "-q", "-b", "main");
    writeFileSync(join(cwd, "initial.txt"), "base\n");
    if (operation !== "add") {
      const target = join(cwd, paths[0]);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(target, "original input\n".repeat(20));
    }
    git(cwd, "add", "-A");
    git(cwd, "commit", "-qm", "base");
    if (event === "pull_request") {
      git(cwd, "checkout", "-qb", "changes");
      for (const filename of operation === "add" ? paths : []) {
        const target = join(cwd, filename);
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, "changed\n");
      }
      if (operation === "delete") rmSync(join(cwd, paths[0]));
      if (operation === "rename") {
        mkdirSync(dirname(join(cwd, paths[1])), { recursive: true });
        git(cwd, "mv", paths[0], paths[1]);
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
      env: { ...process.env,
        GITHUB_EVENT_NAME: event === "unmerged_pr" ? "pull_request" : event,
        GITHUB_OUTPUT: outputFile },
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
    { paths: ["docs/guide.md", "README.md", ".github/agents/guide.md"], preflight: "false", gtk: "false" },
    { paths: ["native_desktop/src/main.rs"], preflight: "true", gtk: "true" },
    { paths: ["core_lib/src/lib.rs"], preflight: "true", gtk: "true" },
    { paths: ["app/main/src/index.ts"], preflight: "true", gtk: "false" },
    { paths: [".github/workflows/native-tag-publish.yml"], preflight: "true", gtk: "true" },
    { paths: [".github/workflows/lint.yml"], preflight: "true", gtk: "true" },
    { paths: ["scripts/ci-pr-scope.sh"], preflight: "true", gtk: "true" },
    { paths: ["packaging/release.toml"], preflight: "true", gtk: "true" },
    { paths: ["docs/prototypes/preview-bus.conf"], preflight: "true", gtk: "true" },
    { paths: ["docs/prototypes/helper.py"], preflight: "true", gtk: "true" },
    { paths: ["native_desktop/packaging/README.md"], preflight: "true", gtk: "true" },
    { paths: ["core_lib/README.md"], preflight: "true", gtk: "true" },
    { paths: ["internal/docs/note.txt"], preflight: "true", gtk: "true" },
    { paths: ["unknown.md"], preflight: "true", gtk: "true" },
    { paths: ["LICENSE"], preflight: "true", gtk: "true" },
    { paths: [".unknown-config"], preflight: "true", gtk: "true" },
    { paths: ["app/main/README.md"], preflight: "true", gtk: "false" },
    { paths: ["native_desktop/untrusted\nname.rs"], preflight: "true", gtk: "true" },
    { paths: ["docs/readme.md", "core_lib/src/lib.rs"], preflight: "true", gtk: "true" },
  ];
  for (const { paths, preflight, gtk } of cases) {
    const result = exercise(paths);
    assert.equal(result.status, 0, JSON.stringify({ paths, stderr: result.stderr }));
    assert.deepEqual(result.outputs, { preflight, gtk }, JSON.stringify(paths));
  }
});

test("Given deletion or rename across scope boundaries When classified Then old inputs remain visible", () => {
  const cases = [
    { paths: ["native_desktop/input.conf", "docs/guide.md"], operation: "rename", preflight: "true", gtk: "true" },
    { paths: ["docs/guide.md", "native_desktop/input.conf"], operation: "rename", preflight: "true", gtk: "true" },
    { paths: ["app/main/input.ts", "docs/guide.md"], operation: "rename", preflight: "true", gtk: "false" },
    { paths: ["docs/old.md", "docs/new.md"], operation: "rename", preflight: "false", gtk: "false" },
    { paths: ["native_desktop/packaging/README.md"], operation: "delete", preflight: "true", gtk: "true" },
    { paths: ["docs/guide.md"], operation: "delete", preflight: "false", gtk: "false" },
  ];
  for (const { paths, operation, preflight, gtk } of cases) {
    const result = exercise(paths, "pull_request", false, operation);
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
  for (const result of [exercise([], "schedule"), exercise([], "pull_request", true), exercise([], "unmerged_pr")]) {
    assert.notEqual(result.status, 0);
    assert.equal(result.outputs, null);
  }
});

test("Given missing job output When classified Then no successful admission is produced", () => {
  const env = { ...process.env, GITHUB_EVENT_NAME: "workflow_dispatch" };
  delete env.GITHUB_OUTPUT;
  const result = spawnSync("bash", [classifier], { encoding: "utf8", env });
  assert.notEqual(result.status, 0);
  assert.ok(result.stderr.includes("GITHUB_OUTPUT"));
});
