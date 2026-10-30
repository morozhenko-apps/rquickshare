#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

fail() {
  printf 'CI policy violation: %s\n' "$*" >&2
  exit 1
}

# Expensive one-off regression, coverage and Debian packaging workflows
# must never be activated by editing their YAML during a dev push.
for name in \
  p0-bwu-router.yml p0-client-introduction.yml p0-weave-worker.yml \
  native-gtk-coverage.yml native-package-smoke.yml native-release-check.yml
do
  file=".github/workflows/$name"
  [[ -f "$file" ]] || fail "missing $file"
  if grep -Eq '^  push:' "$file"; then
    fail "$name still triggers on push"
  fi
  grep -Eq '^  workflow_dispatch:' "$file" ||
    fail "$name is no longer manually accessible"
done

mode_b=".github/workflows/mode-b.yml"
grep -Eq '^  workflow_call:' "$mode_b" || fail 'Mode B reusable entry is missing'
if grep -Eq '^  (push|workflow_dispatch):' "$mode_b"; then
  fail 'Full 10x Mode B must not run directly from development'
fi
# Pinned actionlint predates the newly available GitHub-hosted Ubuntu 26.04.
# Admit only this verified label through its narrow compatibility config;
# do not disable runner-label checking or the other actionlint rules.
grep -Fqx 'self-hosted-runner:' .github/actionlint.yaml ||
  fail 'actionlint compatibility label configuration is missing'
grep -Fqx '    - ubuntu-26.04' .github/actionlint.yaml ||
  fail 'actionlint does not recognize the verified Ubuntu 26.04 runner'
grep -Fq -- '-config-file .github/actionlint.yaml' .github/workflows/native-release-check.yml ||
  fail 'release actionlint does not explicitly read its runner compatibility config'
grep -Fq 'runs-on: ubuntu-26.04' .github/workflows/native-tag-publish.yml ||
  fail 'native release no longer builds against the verified Ubuntu 26.04 runner'
grep -Fq 'runs-on: ubuntu-26.04' .github/workflows/mode-b.yml ||
  fail 'Native Mode B no longer tests on the verified Ubuntu 26.04 runner'
grep -Fq 'tags: ["native/v*"]' .github/workflows/native-tag-publish.yml ||
  fail 'Native release must trigger exclusively on its independent tag namespace'
grep -Fq 'native_release_tag.py version "$GITHUB_REF_NAME"' .github/workflows/native-tag-publish.yml ||
  fail 'Native release admission lacks canonical tag parsing'
grep -Fq 'git fetch --no-tags origin "refs/tags/$GITHUB_REF_NAME:refs/release-validation/native-tag"' .github/workflows/native-tag-publish.yml ||
  fail 'Native release admission must fetch an independent remote tag object'
grep -Fq 'test "$(git cat-file -t refs/release-validation/native-tag)" = tag' .github/workflows/native-tag-publish.yml ||
  fail 'Native release admission must require a remote annotated tag'
grep -Fq 'git rev-parse refs/release-validation/native-tag^{commit}' .github/workflows/native-tag-publish.yml ||
  fail 'Native release admission must check the remote tag target commit'
grep -Fq '== "$GITHUB_SHA"' .github/workflows/native-tag-publish.yml ||
  fail 'Native release admission must match the triggering tag commit'
grep -Fq 'native_release_tag.py version "$TAG"' .github/workflows/native-apt-refresh.yml ||
  fail 'APT refresh does not verify the Native tag namespace'
grep -Fq 'release_tag.tag_from_version' scripts/native_ci_archive.py ||
  fail 'APT archive does not derive the Native source tag'
grep -Fq 'needs: admission' .github/workflows/native-tag-publish.yml ||
  fail 'Release Mode B lacks tag-admission dependency'
grep -Fq 'uses: ./.github/workflows/mode-b.yml' .github/workflows/native-tag-publish.yml ||
  fail 'Release no longer calls full Mode B'
grep -Fq "cancel-in-progress: false" .github/workflows/native-tag-publish.yml ||
  fail 'Release publication may cancel a previous tag'

preflight=".github/workflows/lint.yml"
grep -Fq 'scripts/ci-core-test-groups.sh stable' "$preflight" ||
  fail 'Dev core stable tests have disappeared'
grep -Fq 'scripts/ci-core-test-groups.sh stress' "$preflight" ||
  fail 'Dev core risk-probe stress 10x has disappeared'
for workflow in ".github/workflows/native-gtk-ui.yml" "$mode_b"; do
  grep -Fq 'scripts/ci-native-gtk-test-groups.sh stable' "$workflow" ||
    fail "$workflow does not run native GTK stable once"
  grep -Fq 'scripts/ci-native-gtk-test-groups.sh stress' "$workflow" ||
    fail "$workflow does not run native GTK selected stress ten times"
done
grep -Fq 'scripts/ci-core-test-groups.sh stable' "$mode_b" ||
  fail 'Release Mode B omits Core stable once'
grep -Fq 'scripts/ci-core-test-groups.sh stress' "$mode_b" ||
  fail 'Release Mode B omits Core risk-probe stress ten times'
if grep -Fq 'for attempt in $(seq 1 10)' "$mode_b"; then
  fail 'Release Mode B still repeats stable suites ten times'
fi

for file in .github/workflows/p0-*.yml; do
  grep -Fq "if: github.ref_type == 'tag'" "$file" ||
    fail "$file may run the ten-pass job without a tag"
  grep -Fq 'git rev-parse origin/master^{commit}' "$file" ||
    fail "$file lacks mainline HEAD verification"
done

printf 'PASS: dev fast-test and protected tag-only stress workflow contracts\n'
