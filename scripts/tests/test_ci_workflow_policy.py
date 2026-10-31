"""Semantic workflow and real admission-shell contracts; no product mocks."""
from pathlib import Path
import json
import os
import re
import subprocess
import tempfile
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]
TRUSTED_CACHE_WRITER = "${{ github.ref == 'refs/heads/dev' && github.event_name != 'pull_request' }}"


def workflow(name):
    # BaseLoader preserves GitHub's `on` key and treats expressions as strings.
    return yaml.load((ROOT / '.github/workflows' / name).read_text(), Loader=yaml.BaseLoader)


class CIWorkflowPolicyTests(unittest.TestCase):
    def setUp(self):
        self.preflight = workflow('lint.yml')
        self.gtk = workflow('native-gtk-ui.yml')

    def pnpm_bootstraps(self):
        for filename in ('build.yml', 'feature-package-smoke.yml', 'lint.yml', 'release.yml'):
            matches = []
            for job in workflow(filename)['jobs'].values():
                for step in job.get('steps', []):
                    if step.get('name') == 'Enable pnpm':
                        matches.append(step)
                        nodes = [candidate for candidate in job['steps']
                                 if candidate.get('uses', '').startswith('actions/setup-node@')]
                        self.assertEqual(len(nodes), 1, filename)
                        self.assertEqual(nodes[0]['with']['node-version'], '26.13.0')
            self.assertEqual(len(matches), 1, filename)
            self.assertEqual(matches[0]['shell'], 'bash')
            yield filename, matches[0]['run']

    def run_pnpm_bootstrap(self, script, root, *, exit_code=0, missing_temp=False,
                           broken_path=False):
        bin_dir = root / 'bin'
        bin_dir.mkdir()
        npm = bin_dir / 'npm'
        npm.write_text(
            '#!/usr/bin/env python3\n'
            'import json,os,pathlib,sys\n'
            'args=sys.argv[1:]\n'
            'pathlib.Path(os.environ["NPM_TRACE"]).write_text(json.dumps(args))\n'
            'if int(os.environ["NPM_EXIT"]):sys.exit(int(os.environ["NPM_EXIT"]))\n'
            'prefix=pathlib.Path(args[args.index("--prefix")+1])\n'
            'if str(prefix)!=os.environ["EXPECTED_PREFIX"]:sys.exit(92)\n'
            'shim=prefix/"node_modules"/".bin"/"pnpm"\n'
            'shim.parent.mkdir(parents=True,exist_ok=True)\n'
            'shim.write_text("#!/bin/sh\\nprintf \'9.7.0\\\\n\'\\n")\n'
            'shim.chmod(0o755)\n')
        npm.chmod(0o755)
        trace = root / 'trace.json'
        github_path = root / ('missing-parent/path' if broken_path else 'github-path')
        env = {**os.environ, 'PATH': str(bin_dir) + os.pathsep + os.environ['PATH'],
               'RUNNER_TEMP': str(root), 'GITHUB_PATH': str(github_path),
               'NPM_TRACE': str(trace), 'NPM_EXIT': str(exit_code),
               'EXPECTED_PREFIX': str(root / 'rquickshare-pnpm')}
        if missing_temp:
            env.pop('RUNNER_TEMP')
        result = subprocess.run(['bash', '-c', script], env=env, cwd=root,
                                capture_output=True, text=True, timeout=10)
        return result, trace, github_path

    def test_GivenNode26AndPinnedPnpm_WhenBootstrapRuns_ThenJobLocalShimIsUsable(self):
        for filename, script in self.pnpm_bootstraps():
            for suffix in ('plain', 'space and Unicode-π'):
                with self.subTest(workflow=filename, path=suffix), tempfile.TemporaryDirectory() as temp:
                    # Arrange: npm is the external boundary; the workflow shell is real.
                    root = Path(temp) / suffix
                    root.mkdir()
                    # Act: execute the checked-in workflow bootstrap.
                    result, trace, github_path = self.run_pnpm_bootstrap(script, root)
                    # Assert: exact installation contract and exported executable.
                    self.assertEqual(result.returncode, 0, result.stderr)
                    prefix = root / 'rquickshare-pnpm'
                    self.assertEqual(json.loads(trace.read_text()), [
                        'install', '--prefix', str(prefix), '--no-save', '--ignore-scripts',
                        '--no-audit', '--no-fund', 'pnpm@9.7.0'])
                    self.assertEqual(github_path.read_text(), str(prefix / 'node_modules/.bin') + '\n')
                    env = {**os.environ, 'PATH': github_path.read_text().strip() + os.pathsep + os.environ['PATH']}
                    version = subprocess.run(['pnpm', '--version'], env=env, capture_output=True,
                                             text=True, timeout=10)
                    self.assertEqual(version.returncode, 0, version.stderr)
                    self.assertEqual(version.stdout, '9.7.0\n')

    def test_GivenNpmFailure_WhenBootstrapRuns_ThenExactFailureStopsPathExport(self):
        for filename, script in self.pnpm_bootstraps():
            for code in (1, 23):
                with self.subTest(workflow=filename, code=code), tempfile.TemporaryDirectory() as temp:
                    # Arrange / Act: inject only the external npm failure.
                    result, trace, github_path = self.run_pnpm_bootstrap(script, Path(temp), exit_code=code)
                    # Assert: no fallback/global install or successful PATH receipt.
                    self.assertEqual(result.returncode, code, result.stderr)
                    self.assertTrue(trace.is_file())
                    self.assertFalse(github_path.exists())

    def test_GivenMissingRunnerTemp_WhenBootstrapRuns_ThenNoInstallIsAttempted(self):
        for filename, script in self.pnpm_bootstraps():
            with self.subTest(workflow=filename), tempfile.TemporaryDirectory() as temp:
                # Arrange / Act: omit the job-owned installation boundary.
                result, trace, github_path = self.run_pnpm_bootstrap(script, Path(temp), missing_temp=True)
                # Assert: fail before npm or a path export can touch another location.
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(trace.exists())
                self.assertFalse(github_path.exists())

    def test_GivenPathExportFailure_WhenBootstrapRuns_ThenJobCannotReportSuccess(self):
        for filename, script in self.pnpm_bootstraps():
            with self.subTest(workflow=filename), tempfile.TemporaryDirectory() as temp:
                # Arrange / Act: make the runner's PATH receipt unwritable by construction.
                result, trace, github_path = self.run_pnpm_bootstrap(script, Path(temp), broken_path=True)
                # Assert: installation success alone is insufficient.
                self.assertNotEqual(result.returncode, 0)
                self.assertTrue(trace.is_file())
                self.assertFalse(github_path.exists())

    def test_GivenOrdinaryCI_WhenParsed_ThenJobsRemainIndependentAndBounded(self):
        expected = [
            (self.preflight, dict(scope=5, rust_format=10, frontend=10,
                                  core_tests=20, clippy=20, admission=5)),
            (self.gtk, dict(scope=5, native_gtk=20, admission=5)),
        ]
        for doc, budgets in expected:
            with self.subTest(workflow=doc['name']):
                self.assertEqual(set(doc['jobs']), set(budgets))
                self.assertEqual(doc['permissions'], {'contents': 'read'})
                self.assertEqual(set(doc['on']), {'push', 'pull_request', 'workflow_dispatch'})
                for name, minutes in budgets.items():
                    job = doc['jobs'][name]
                    self.assertEqual(job['timeout-minutes'], str(minutes))
                    self.assertNotIn('environment', job)
                    self.assertNotIn('continue-on-error', job)
                    if name not in ('scope', 'admission'):
                        self.assertEqual(job['needs'], 'scope')
                for job in doc['jobs'].values():
                    for step in job.get('steps', []):
                        self.assertNotIn('continue-on-error', step)
                        if step.get('uses', '').startswith('actions/checkout@'):
                            self.assertEqual(step['with']['persist-credentials'], 'false')
                self.assertNotIn('${{ secrets.', str(doc))
                self.assertEqual(doc['jobs']['admission']['if'], 'always()')
                self.assertEqual(doc['concurrency']['cancel-in-progress'], 'true')
        self.assertEqual(self.preflight['jobs']['admission']['needs'],
                         ['scope', 'rust_format', 'frontend', 'core_tests', 'clippy'])
        self.assertEqual(self.gtk['jobs']['admission']['needs'], ['scope', 'native_gtk'])
        self.assertEqual(self.preflight['jobs']['admission']['name'], 'Preflight admission')
        self.assertEqual(self.gtk['jobs']['admission']['name'], 'Native GTK admission')
        self.assertEqual(self.preflight['jobs']['admission']['env'], dict(
            SCOPE_RESULT='${{ needs.scope.result }}',
            PREFLIGHT_REQUIRED='${{ needs.scope.outputs.preflight }}',
            RUST_RESULT='${{ needs.rust_format.result }}',
            FRONTEND_RESULT='${{ needs.frontend.result }}',
            CORE_RESULT='${{ needs.core_tests.result }}',
            CLIPPY_RESULT='${{ needs.clippy.result }}'))
        self.assertEqual(self.gtk['jobs']['admission']['env'], dict(
            SCOPE_RESULT='${{ needs.scope.result }}',
            GTK_REQUIRED='${{ needs.scope.outputs.gtk }}',
            GTK_RESULT='${{ needs.native_gtk.result }}'))

    def test_GivenRustCaches_WhenRestored_ThenOnlyTrustedDevCanSaveAndConfigurationsStaySeparate(self):
        caches = []
        for doc in (self.preflight, self.gtk):
            for job in doc['jobs'].values():
                caches.extend(step['with'] for step in job.get('steps', [])
                              if step.get('uses') == 'Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0')
        self.assertEqual(len(caches), 3)
        self.assertEqual(len({cache['shared-key'] for cache in caches}), 3)
        for cache in caches:
            self.assertEqual(cache['save-if'], TRUSTED_CACHE_WRITER)
            self.assertIn('-> target', cache['workspaces'])

    def test_GivenOrdinaryCommands_WhenInspected_ThenLockedTestsRunOnceWithoutStressOrCoverage(self):
        for doc in (self.preflight, self.gtk):
            commands = '\n'.join(step.get('run', '') for job in doc['jobs'].values()
                                 for step in job.get('steps', []))
            self.assertNotIn('test-groups.sh stress', commands)
            self.assertNotIn('llvm-cov', commands)
            self.assertNotIn('--coverage', commands)
            selector = 'ci-core-test-groups.sh stable' if doc is self.preflight else 'ci-native-gtk-test-groups.sh stable'
            self.assertEqual(commands.count(selector), 1)
        contracts = '\n'.join(step.get('run', '') for step in
                              self.preflight['jobs']['rust_format']['steps'])
        self.assertIn('-m pip install -r scripts/requirements-ci.txt', contracts)
        self.assertEqual(contracts.count('-m unittest discover -s scripts/tests'), 1)
        core = (ROOT / 'scripts/ci-core-test-groups.sh').read_text()
        commands = [line for line in core.splitlines() if '$(cargo test ' in line]
        self.assertEqual(len(commands), 3)
        for command in commands:
            self.assertIn('cargo test --locked ', command)
        self.assertNotIn('--skip ', core)
        clippy_steps = self.preflight['jobs']['clippy']['steps']
        clippy = next(step['run'] for step in clippy_steps if step.get('name') == 'Clippy')
        desktop = next(step['run'] for step in clippy_steps if step.get('name') == 'Desktop Rust unit tests')
        self.assertIn('clippy --locked ', clippy)
        self.assertIn('test --locked ', desktop)

    def test_GivenManualSyntaxCheck_WhenInputsAreResolved_ThenAllWorkflowFilesExist(self):
        checker = workflow('native-release-check.yml')
        self.assertEqual(set(checker['on']), {'workflow_dispatch'})
        self.assertEqual(checker['jobs']['workflow-syntax']['timeout-minutes'], '10')
        command = checker['jobs']['workflow-syntax']['steps'][-1]['run']
        paths = re.findall(r'\.github/workflows/[\w.-]+\.yml', command)
        self.assertEqual(set(paths), {'.github/workflows/' + name for name in (
            'lint.yml', 'native-gtk-ui.yml', 'native-tag-publish.yml', 'native-apt-refresh.yml')})
        for path in paths:
            self.assertTrue((ROOT / path).is_file(), path)

    def run_gate(self, doc, values):
        gate = doc['jobs']['admission']
        self.assertEqual(len(gate['steps']), 1)
        return subprocess.run(['bash', '-c', gate['steps'][0]['run']],
                              env={**os.environ, **values}, capture_output=True,
                              text=True, timeout=10).returncode

    def test_GivenPreflightDependencies_WhenAdmissionRuns_ThenOnlyExactApplicableOutcomesPass(self):
        dependencies = ('RUST_RESULT', 'FRONTEND_RESULT', 'CORE_RESULT', 'CLIPPY_RESULT')
        for required, expected in (('true', 'success'), ('false', 'skipped')):
            values = dict(SCOPE_RESULT='success', PREFLIGHT_REQUIRED=required,
                          **dict.fromkeys(dependencies, expected))
            with self.subTest(required=required, case='all exact outcomes'):
                self.assertEqual(self.run_gate(self.preflight, values), 0)
            for dependency in dependencies:
                for result in ('success', 'failure', 'cancelled', 'skipped', ''):
                    if result == expected:
                        continue
                    with self.subTest(required=required, dependency=dependency, result=result):
                        self.assertNotEqual(self.run_gate(self.preflight,
                            {**values, dependency: result}), 0)

    def test_GivenGTKDependency_WhenAdmissionRuns_ThenOnlyExactApplicableOutcomesPass(self):
        for required, expected in (('true', 'success'), ('false', 'skipped')):
            for result in ('success', 'failure', 'cancelled', 'skipped', ''):
                with self.subTest(required=required, result=result):
                    code = self.run_gate(self.gtk, dict(SCOPE_RESULT='success',
                                         GTK_REQUIRED=required, GTK_RESULT=result))
                    self.assertEqual(code == 0, result == expected)

    def test_GivenInvalidScope_WhenEitherGateRuns_ThenNoDependencySuccessCanAuthorizeIt(self):
        for doc, output, dependencies in (
            (self.preflight, 'PREFLIGHT_REQUIRED', ['RUST_RESULT', 'FRONTEND_RESULT', 'CORE_RESULT', 'CLIPPY_RESULT']),
            (self.gtk, 'GTK_REQUIRED', ['GTK_RESULT']),
        ):
            self.assertEqual(doc['jobs']['admission']['env']['SCOPE_RESULT'], '${{ needs.scope.result }}')
            self.assertEqual(doc['jobs']['admission']['env'][output],
                             '${{ needs.scope.outputs.' + ('preflight' if output == 'PREFLIGHT_REQUIRED' else 'gtk') + ' }}')
            for scope_result, scope_output in (
                ('failure', 'true'), ('cancelled', 'true'), ('skipped', 'false'),
                ('', 'true'), ('success', ''), ('success', 'unknown'),
            ):
                with self.subTest(workflow=doc['name'], result=scope_result, output=scope_output):
                    values = dict.fromkeys(dependencies, 'success')
                    values.update(SCOPE_RESULT=scope_result, **{output: scope_output})
                    self.assertNotEqual(self.run_gate(doc, values), 0)


if __name__ == '__main__':
    unittest.main()
