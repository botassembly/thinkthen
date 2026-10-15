#!/usr/bin/env python3
"""Isolated owner control flow and a tiny real compiler; no product checks/GH."""
import importlib.util
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
from unittest.mock import patch

HERE = Path(__file__).resolve().parent

def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result

m = module('campaign_owner_test', 'campaign-owner.py')
l = module('campaign_launcher_test', 'launch-candidate-campaign.py')
SHA = 'a' * 40
TAG = 'rc/0.2.0-rc.14'
admission = m.load_admission()
admission.load_allows_build = lambda: True


def assert_raises(fn):
    try:
        fn()
    except RuntimeError:
        return
    raise AssertionError('required refusal missing')


with tempfile.TemporaryDirectory(prefix='thinkthen-campaign-owner-stubs-') as scratch:
    root = Path(scratch)
    base = root / 'outputs'; base.mkdir()
    fake = root / 'fake-repo'; fake.mkdir()
    cov = base / 'coverage-candidate'
    with patch.multiple(m, ROOT=fake, BASE=base, COVERAGE=cov, LOCK=str(root / 'isolated.lock')):
        # Real owned child execution: success, 77 refusal, signals, daemon reaping.
        for name, source, expected in (
            ('ok', 'print("synthetic success")', 0),
            ('missing', 'raise SystemExit(77)', 77),
            ('signal', 'import os,signal;os.kill(os.getpid(),signal.SIGTERM)', 143),
            ('daemon', 'import os,time; p=os.fork(); os.setsid() if not p else None; time.sleep(60) if not p else None', 0)):
            item = m.Owner(SHA, 'local', tag=TAG); item.folder = base / name; item.folder.mkdir()
            item.admission = admission; item.env = os.environ.copy(); item.check = lambda: None
            command = [sys.executable, '-c', source]
            if expected:
                assert_raises(lambda: item.phase(name, command))
            else:
                item.phase(name, command)
            receipt = m.read(item.folder / (name + '.json'))
            assert receipt['command_exit'] == expected and not receipt['remaining_children'], receipt
        # Actual compiler admission nested below the real campaign phase boundary.
        if m.shutil.which('cc'):
            item = m.Owner(SHA, 'local', tag=TAG)
            item.folder = base / 'nested-compiler'; item.folder.mkdir()
            item.admission = admission
            item.env = {'PATH': os.defpath, 'THINKTHEN_HEAVY_LOCK': m.LOCK}
            item.check = lambda: None
            source = item.folder / 'main.c'
            binary = item.folder / 'main'
            source.write_text('int main(void) { return 0; }\n')
            item.phase('nested', [sys.executable, str(HERE / 'build-admission.py'),
                                 '--timeout', '3', '--', 'cc', str(source), '-o', str(binary)])
            subprocess.run([str(binary)], check=True, timeout=3)
            assert not m.read(item.folder / 'nested.json')['remaining_children']
        item = m.Owner(SHA, 'local', tag=TAG); item.folder = base / 'afterguard'; item.folder.mkdir()
        item.admission = admission; item.env = os.environ.copy()
        calls = []
        def guarded():
            calls.append(1)
            if len(calls) == 3:
                raise RuntimeError('synthetic SHA movement')
        item.check = guarded
        assert_raises(lambda: item.phase('guard', [sys.executable, '-c', 'pass']))
        assert m.read(item.folder / 'guard.json')['exit'] == 1
        # Local phase order, unique output, summary preservation, and strict join.
        item = m.Owner(SHA, 'local', tag=TAG); item.folder.mkdir(); item.check = lambda: None
        m.atomic(item.folder / 'native-proof.json', {'sha': SHA, 'tag': TAG, 'exit': 0})
        phases = []
        def phase(name, command):
            phases.append((name, command))
            if name == 'fullcases':
                # The orchestrator holds no build reservation; children admit themselves.
                probe = """import fcntl, os, subprocess, sys
lock = os.environ['THINKTHEN_HEAVY_LOCK']
assert 'THINKTHEN_HEAVY_LOCK_HELD' not in os.environ
with open(lock, 'a+b') as stream:
    fcntl.flock(stream.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    for slot in range(4):
        fcntl.lockf(stream.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB, 1, slot)
"""
                m.Owner.phase(item, name, [sys.executable, '-c', probe])
            if name == 'coverage':
                cov.mkdir()
                m.atomic(cov / 'totals.json', {'source': SHA, 'platform_collection': 'linux', 'unmeasured_collections': []})
                summary = 'sdlc/records/coverage/2099-01-02-' + SHA[:9] + '-linux.md'
                destination = fake / summary; destination.parent.mkdir(parents=True); destination.write_text('# Coverage at ' + SHA + '\n\nlinux platform collection; require both platform reports before acceptance.\n')
        item.phase = phase; item.validate_matrix = lambda: None; item.wait_receipt = lambda name: {'sha': SHA, 'tag': TAG, 'exit': 0}
        with patch.object(m, 'environment', return_value={'THINKTHEN_HEAVY_LOCK': m.LOCK}), patch.object(m, 'load_admission', return_value=admission):
            item.local()
        assert [v[0] for v in phases] == ['fullcases', 'package', 'stress', 'coverage', 'coverage-join']
        assert (item.folder / 'coverage-summary.md').read_text().startswith('# Coverage at ' + SHA)
        assert phases[0][1] == ['sh', 'sdlc/scripts/test-full-cases', '--run']
        assert phases[2][1] == ['sh', 'sdlc/scripts/test-stress', '--run']
        # The summary date comes from the unchanged collector, including next day.
        summary_path = fake / ('sdlc/records/coverage/2099-01-02-' + SHA[:9] + '-linux.md')
        with patch.object(m, 'guard'):
            checked = m.Owner(SHA, 'local', tag=TAG)
            checked.check()
            assert str(summary_path.relative_to(fake)) in checked.allowed
            summary_path.write_text(summary_path.read_text() + 'changed')
            assert_raises(checked.check)
            summary_path.write_text((item.folder / 'coverage-summary.md').read_text())
            extra_summary = summary_path.with_name('2099-01-03-' + SHA[:9] + '-linux.md')
            extra_summary.write_text(summary_path.read_text())
            assert_raises(checked.check)
            extra_summary.unlink()
            peer = checked.other; peer.mkdir()
            m.atomic(peer / 'Result.json', {'sha': SHA, 'state': 'failed', 'exit': 1})
            assert_raises(lambda: checked.wait_receipt('never.json', timeout=1))
            (peer / 'Result.json').unlink()
        # Real stored gate schema, with synthetic same-SHA identities and CLI bytes.
        required = 'cli-fresh test lint polars-nextest polars-clippy polars-clippy-nodefault native-c c-fmt c-clippy native-c-abi root-library rust-library polars-feature polars-library python python-import ruby typescript r-native jvm-java-kotlin-scala cpp-configure cpp cpp-install ada cobol php go-localize go-stage go zig swift-linux typescript-import dart-stage dart-dependencies dart flutter-dependencies flutter-linux ruby-import-correct-name csharp-offline-build python-pandas-polars-import r-bridge r-bridge-import c-consumer typescript-declarations objective-c-platform sqlite-baseline c-official c-complete c-installed'.split()
        fixture = {'remaining_children': [], 'gates': {name: {'exit': 0, 'remaining_children': []} for name in required}}
        gate = base / ('0543-final-' + SHA[:9]); gate.mkdir()
        cli = gate / 'thinkthen'; cli.write_text('#!/bin/sh\nexit 0\n'); cli.chmod(0o700)
        fixture.update(revision=SHA, revision_after=SHA, source_clean=True,
                       state='passed-local-platform-pending', effective_failures=[])
        for value in fixture['gates'].values():
            value.update(revision=SHA, cwd=str(fake))
        fixture['gates']['test-retry'] = dict(fixture['gates']['test'])
        fixture['artifacts'] = {str(cli): {'revision': SHA, 'bytes': cli.stat().st_size,
                                         'sha256': m.hashlib.sha256(cli.read_bytes()).hexdigest()}}
        m.atomic(gate / 'results-final-core.json', fixture)
        m.gate_inputs(SHA)
        missing = fixture['gates'].pop('lint')
        m.atomic(gate / 'results-final-core.json', fixture)
        assert_raises(lambda: m.gate_inputs(SHA))
        fixture['gates']['lint'] = missing
        m.atomic(gate / 'results-final-core.json', fixture)
        cli.write_text('stale artifact')
        assert_raises(lambda: m.gate_inputs(SHA))
        cli.write_text('#!/bin/sh\nexit 0\n')
        # Finished hosted run stubs. Every required architecture is checked.
        jobs = [{'name': name, 'conclusion': 'success', 'labels': ['ubuntu-24.04']} for name in
                ('resolve', 'apple-coverage', 'wheels', 'gems', 'npm-pack', 'crate', 'registries', 'windows-sdks', 'draft')]
        targets = ('x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu', 'aarch64-apple-darwin', 'x86_64-apple-darwin', 'x86_64-pc-windows-msvc')
        runners = dict(zip(targets, ('ubuntu-24.04', 'ubuntu-24.04-arm', 'macos-15', 'macos-15-intel', 'windows-2025')))
        jobs += [{'name': family + ' (' + target + ', ' + runners[target] + ')', 'conclusion': 'success', 'labels': [runners[target]]} for family in ('build', 'smoke') for target in targets]
        jobs += [{'name': 'apple-packages (' + target + ', ' + runners[target] + ')', 'conclusion': 'success', 'labels': [runners[target]]} for target in targets if 'apple' in target]
        jobs += [{'name': name, 'conclusion': 'skipped', 'labels': []} for name in ('crates', 'pypi', 'npm', 'rubygems', 'tap', 'nuget', 'maven', 'pub', 'publish')]
        hosted = m.Owner(SHA, 'hosted', tag=TAG); hosted.folder.mkdir(exist_ok=True); hosted.check = lambda: None
        hosted.validate_jobs(jobs)
        for broken in (
            [j for j in jobs if not j['name'].startswith('smoke (x86_64-pc-windows')],
            [{**j, 'conclusion': 'skipped'} if j['name'] == 'apple-coverage' else j for j in jobs],
            [{**j, 'conclusion': 'success'} if j['name'] == 'publish' else j for j in jobs],
            [j for j in jobs if j['name'] != 'publish'],
            [{**j, 'labels': ['macos-15']} if 'x86_64-apple-darwin' in j['name'] else j for j in jobs]):
            assert_raises(lambda: hosted.validate_jobs(broken))
        calls = []
        run = {'id': 123, 'path': '.github/workflows/release.yml', 'head_sha': SHA, 'head_branch': TAG, 'event': 'workflow_dispatch', 'status': 'completed', 'conclusion': 'success'}
        runlists = iter([{'workflow_runs': []}, {'workflow_runs': [run]}])
        def api(path):
            return next(runlists) if 'workflows/' in path else run
        def gh(*args):
            calls.append(args)
            if args[:2] == ('run', 'download'):
                dest = Path(args[args.index('--dir') + 1]); dest.mkdir()
                m.atomic(dest / 'totals.json', {'source': SHA, 'platform_collection': 'apple', 'unmeasured_collections': []})
            return ''
        hosted.api = api; hosted.gh = gh; hosted.jobs = lambda runid: jobs
        hosted.wait_receipt = lambda name: {'sha': SHA, 'tag': TAG, 'exit': 0}
        with patch.object(m.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0)) as completed_log:
            hosted.hosted()
        assert calls[0] == ('workflow', 'run', 'release.yml', '--repo', m.REPO, '--ref', TAG, '-f', 'mode=rehearse')
        assert m.read(hosted.folder / 'completed.json')['status'] == 'completed'
        assert m.read(hosted.folder / 'apple-ready.json')['sha'] == SHA
        assert completed_log.call_args.args[0][-1] == '--log'
        # Native receipts allow an optional website skip, reject SHA drift and 77.
        path = root / 'native.json'
        native = {'head_sha': SHA, 'conclusion': 'success', 'exit': 0, 'completed_log_exit': 0,
                  'jobs': [{'name': 'foundation', 'conclusion': 'success', 'labels': ['macos-15'], 'failed_steps': []}, {'name': 'website', 'conclusion': 'skipped'}]}
        m.atomic(path, native); l.native_receipt(path, SHA, 'objective-c')
        assert_raises(lambda: l.native_receipt(path, 'b' * 40, 'objective-c'))
        native['exit'] = 77; m.atomic(path, native)
        assert_raises(lambda: l.native_receipt(path, SHA, 'objective-c'))
        native['exit'] = 0
        native['jobs'].append({'name': 'extra-required', 'conclusion': 'skipped'})
        m.atomic(path, native)
        assert_raises(lambda: l.native_receipt(path, SHA, 'objective-c'))
        # Guard uses synthetic command answers: clean matches pass; wrong tag fails.
        def git(args, **kwargs):
            if args[1:3] == ['rev-parse', 'HEAD']: return SHA
            if args[1:3] == ['branch', '--show-current']: return m.BRANCH
            if args[1] in ('status', 'ls-files'): return ''
            if args[1] == 'rev-parse': return SHA
            if args[1] == 'ls-remote': return SHA + '\trefs/heads/' + m.BRANCH + '\n' + SHA + '\trefs/tags/' + TAG
            raise AssertionError(args)
        with patch.object(m, 'output', side_effect=git):
            m.guard(SHA, TAG)
            assert_raises(lambda: m.guard('3e774eca2' + 'a' * 31, TAG))
        with patch.object(m, 'output', side_effect=lambda args, **kwargs: 'b' * 40 if args[1] == 'rev-parse' and args[-1].endswith('^{commit}') else git(args)):
            assert_raises(lambda: m.guard(SHA, TAG))
print('PASS: isolated synthetic commands exercise ownership/reaping, exits 0/77/143, phase order, guards, strict hosted architectures, no publishing, exact-SHA receipts and coverage join handoff; real nested compiler; no product checks or dispatch')
