#!/usr/bin/env python3
"""Retained, fail-closed owners for one future candidate. No work at import."""
import argparse
import hashlib
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(tempfile.gettempdir()) / 'thinkthen-0543-final-target'
BRANCH = 'ticket/0543-final'
REPO = 'botassembly/thinkthen'
REJECT = '3e774eca2'
COVERAGE = BASE / 'coverage-candidate'
LOCK = '/tmp/thinkthen-heavy.lock'


def atomic(path, value):
    atomic_text(path, json.dumps(value, indent=2) + '\n')


def atomic_text(path, value):
    fd, name = tempfile.mkstemp(prefix='.' + path.name + '-', dir=path.parent)
    temp = Path(name)
    try:
        with os.fdopen(fd, 'w') as stream:
            stream.write(value)
            stream.flush()
            os.fsync(stream.fileno())
        temp.replace(path)
    finally:
        temp.unlink(missing_ok=True)


def read(path):
    return json.loads(path.read_text())


def parse_job_pages(raw):
    def object_pairs(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError('duplicate JSON key: ' + key)
            result[key] = value
        return result
    def invalid_constant(value):
        raise ValueError('invalid JSON constant: ' + value)
    decoder = json.JSONDecoder(object_pairs_hook=object_pairs, parse_constant=invalid_constant)
    position, pages = 0, []
    while position < len(raw):
        if raw[position].isspace():
            position += 1
            continue
        page, position = decoder.raw_decode(raw, position)
        if (not isinstance(page, dict) or type(page.get('total_count')) is not int
                or page['total_count'] < 0 or not isinstance(page.get('jobs'), list)
                or any(not isinstance(job, dict) for job in page['jobs'])):
            raise RuntimeError('invalid jobs page')
        pages.append(page)
    if not pages or len({page['total_count'] for page in pages}) != 1:
        raise RuntimeError('missing or inconsistent jobs page totals')
    jobs = [job for page in pages for job in page['jobs']]
    ids = [job.get('id') for job in jobs]
    if (len(jobs) != pages[0]['total_count'] or any(type(i) is not int or i <= 0 for i in ids)
            or len(set(ids)) != len(ids)):
        raise RuntimeError('incomplete or duplicate jobs pages')
    return jobs


def output(args, cwd=ROOT, env=None):
    return subprocess.check_output(args, cwd=cwd, env=env, text=True).strip()


def candidate_tag(value):
    match = re.fullmatch(r'rc/0[.]2[.]0-rc[.]([1-9][0-9]*)', value)
    if not match or int(match[1]) < 13:
        raise argparse.ArgumentTypeError('need an explicit 0.2.0 candidate tag, rc.13 or later')
    return value


def campaign_folder(sha, tag, kind):
    candidate_tag(tag)
    return BASE / ('campaign-' + sha + '-' + tag.replace('/', '-') + '-' + kind)


def guard(sha, tag, allowed=()):
    candidate_tag(tag)
    if not re.fullmatch('[0-9a-f]{40}', sha) or sha.startswith(REJECT):
        raise RuntimeError('need a future repaired full SHA; failed 3e774eca2 is forbidden')
    if output(['git', 'rev-parse', 'HEAD']) != sha:
        raise RuntimeError('HEAD changed')
    if output(['git', 'branch', '--show-current']) != BRANCH:
        raise RuntimeError('wrong candidate branch')
    if output(['git', 'status', '--porcelain', '--untracked-files=no']):
        raise RuntimeError('tracked source is dirty')
    untracked = set(output(['git', 'ls-files', '--others', '--exclude-standard']).splitlines()) - {''}
    if untracked - set(allowed):
        raise RuntimeError('unexpected untracked source: ' + repr(sorted(untracked - set(allowed))))
    if output(['git', 'rev-parse', tag + '^{commit}']) != sha:
        raise RuntimeError('local candidate tag differs')
    remote = output(['git', 'ls-remote', 'origin', 'refs/heads/' + BRANCH,
                     'refs/tags/' + tag, 'refs/tags/' + tag + '^{}'])
    refs = {line.split()[1]: line.split()[0] for line in remote.splitlines()}
    if refs.get('refs/heads/' + BRANCH) != sha:
        raise RuntimeError('origin candidate branch differs')
    if refs.get('refs/tags/' + tag + '^{}', refs.get('refs/tags/' + tag)) != sha:
        raise RuntimeError('remote candidate tag differs')


def gate_inputs(sha):
    folder = BASE / ('0543-final-' + sha[:9])
    result = read(folder / 'results-final-core.json')
    if (result.get('revision') != sha or result.get('revision_after') != sha
            or result.get('state') != 'passed-local-platform-pending'
            or result.get('source_clean') is not True or result.get('remaining_children') != []
            or result.get('effective_failures') or 'build_setup_error' in result):
        raise RuntimeError('need complete clean successful same-SHA gate owner')
    required = set('cli-fresh test lint polars-nextest polars-clippy polars-clippy-nodefault native-c c-fmt c-clippy native-c-abi root-library rust-library polars-feature polars-library python python-import ruby typescript r-native jvm-java-kotlin-scala cpp-configure cpp cpp-install ada cobol php go-localize go-stage go zig swift-linux typescript-import dart-stage dart-dependencies dart flutter-dependencies flutter-linux ruby-import-correct-name csharp-offline-build python-pandas-polars-import r-bridge r-bridge-import c-consumer typescript-declarations objective-c-platform sqlite-baseline c-official c-complete c-installed'.split())
    gates = result.get('gates', {})
    effective = result.get('effective_routine', 'test')
    if effective not in ('test', 'test-retry') or not required <= gates.keys() or effective not in gates:
        raise RuntimeError('missing required final gates')
    for name, value in gates.items():
        if value.get('revision') != sha or value.get('remaining_children') != [] or not Path(value.get('cwd', '/')).resolve().is_relative_to(ROOT.resolve()):
            raise RuntimeError('gate identity/ownership differs: ' + name)
        if name == 'test' and effective == 'test-retry':
            continue
        if value.get('exit') != 0 and not (name == 'objective-c-platform' and value.get('exit') == 77):
            raise RuntimeError('required final gate failed: ' + name)
    cli = folder / 'thinkthen'
    artifact = result.get('artifacts', {}).get(str(cli), {})
    if (artifact.get('revision') != sha or not cli.is_file() or not os.access(cli, os.X_OK)
            or artifact.get('sha256') != hashlib.sha256(cli.read_bytes()).hexdigest()
            or artifact.get('bytes') != cli.stat().st_size):
        raise RuntimeError('same-SHA CLI artifact differs')
    return folder, result


def environment(folder, sha):
    gate, _ = gate_inputs(sha)
    env = read(gate / 'environment.json')
    # Drop the gate's wrappers and use only its verified CLI in our own folder.
    env['PATH'] = ':'.join(part for part in env['PATH'].split(':')
                           if part and not Path(part).resolve().is_relative_to(BASE.resolve()))
    for key in ('RUSTC_WRAPPER', 'THINKTHEN_COMPOSER_BIN', 'HOME', 'GATE_OUTPUT',
                'XDG_CONFIG_HOME', 'XDG_CACHE_HOME', 'XDG_STATE_HOME', 'THINKTHEN_HEAVY_LOCK_HELD'):
        env.pop(key, None)
    for key, value in env.items():
        if str(BASE) in value:
            raise RuntimeError('preserved environment references a campaign/gate path: ' + key)
    shutil.copy2(gate / 'thinkthen', folder / 'thinkthen')
    # Freeze all dependency locations before replacing HOME.
    home = Path.home()
    tools = str(home / '.cache/thinkthen-toolchains')
    env.update(THINKTHEN_TOOLCHAINS=tools, R_LIBS_USER=env['R_LIBS'],
               UV_PYTHON_INSTALL_DIR=str(home / '.local/share/uv/python'),
               npm_config_cache=env['NPM_CONFIG_CACHE'], RUSTUP_TOOLCHAIN='1.95.0',
               RUSTC_WRAPPER='', THINKTHEN_HEAVY_LOCK=LOCK,
               HOME=str(folder / 'home'))
    env['PATH'] = str(folder) + os.pathsep + str(Path(env['TT_FLUTTER']).parent) + os.pathsep + env['PATH']
    env['THINKTHEN_COMPOSER_BIN'] = str(folder / 'composer')
    env['JAVA_HOME'] = env['THINKTHEN_SESSION_JDK_HOME']
    for key in ('XDG_CONFIG_HOME', 'XDG_CACHE_HOME', 'XDG_STATE_HOME'):
        env[key] = str(folder / 'home' / key)
        Path(env[key]).mkdir(parents=True, exist_ok=True)
    shutil.copy2(read(gate / 'environment.json')['THINKTHEN_COMPOSER_BIN'], folder / 'composer')
    (folder / 'composer').chmod(0o700)
    for key in ('CARGO_HOME', 'RUSTUP_HOME', 'PUB_CACHE', 'UV_CACHE_DIR',
                'UV_PYTHON_INSTALL_DIR', 'THINKTHEN_TOOLCHAINS', 'R_LIBS_USER', 'JAVA_HOME'):
        if not Path(env[key]).exists():
            raise RuntimeError('missing preserved tool/cache ' + key)
    llvm = Path(env['RUSTUP_HOME']) / 'toolchains/1.95.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin'
    for name in ('llvm-profdata', 'llvm-cov'):
        if not (llvm / name).is_file():
            raise RuntimeError('missing pinned Rust LLVM tool ' + name)
    for key in ('TT_DART', 'TT_FLUTTER', 'RUBY', 'THINKTHEN_COMPOSER_BIN'):
        executable = Path(env[key])
        if not executable.is_absolute() or not executable.is_file() or not os.access(executable, os.X_OK):
            raise RuntimeError('missing executable override ' + key)
    if shutil.which('thinkthen', path=env['PATH']) != str(folder / 'thinkthen'):
        raise RuntimeError('campaign CLI does not resolve to verified same-SHA artifact')
    atomic(folder / 'environment.json', env)
    return env


def load_admission():
    spec = importlib.util.spec_from_file_location('admission', ROOT / 'sdlc/scripts/build-admission.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class Owner:
    def __init__(self, sha, kind, resume=False, *, tag):
        self.tag = candidate_tag(tag)
        self.sha, self.kind, self.resume = sha, kind, resume
        self.folder = campaign_folder(sha, self.tag, kind)
        self.other = campaign_folder(sha, self.tag, 'hosted' if kind == 'local' else 'local')
        self.result = {'sha': sha, 'tag': self.tag, 'kind': kind, 'state': 'running', 'phases': {}, 'exit': None}
        self.allowed = set()

    def save(self):
        atomic(self.folder / 'Result.json', self.result)

    def check(self):
        peer = self.other / 'Result.json'
        if peer.exists():
            value = read(peer)
            if value.get('sha') != self.sha or value.get('tag') != self.tag or value.get('state') == 'failed' or value.get('exit') not in (None, 0):
                raise RuntimeError('peer owner failed or changed identity')
        expected = self.folder / 'coverage-expected.json' if self.kind == 'local' else self.other / 'coverage-expected.json'
        if expected.exists():
            receipt = read(expected)
            if receipt.get('sha') != self.sha or receipt.get('tag') != self.tag or receipt.get('platform') != 'linux':
                raise RuntimeError('coverage summary identity differs')
            candidates = list((ROOT / 'sdlc/records/coverage').glob('*-' + self.sha[:9] + '-linux.md'))
            if len(candidates) > 1:
                raise RuntimeError('multiple candidate coverage summaries')
            if receipt.get('summary') is not None and not candidates:
                raise RuntimeError('completed coverage summary disappeared')
            if candidates:
                path = candidates[0]
                relative = str(path.relative_to(ROOT))
                if (not re.fullmatch(r'sdlc/records/coverage/\d{4}-\d{2}-\d{2}-' + self.sha[:9] + r'-linux\.md', relative)
                        or path.is_symlink() or not path.read_text().startswith('# Coverage at ' + self.sha + '\n')
                        or '\nlinux platform collection; require both platform reports before acceptance.\n' not in path.read_text()):
                    raise RuntimeError('unexpected generated coverage summary identity')
                payload = read(COVERAGE / 'totals.json')
                if payload.get('source') != self.sha or payload.get('platform_collection') != 'linux' or payload.get('unmeasured_collections') != []:
                    raise RuntimeError('coverage summary has no matching complete totals')
                if receipt.get('summary') is not None and (receipt['summary'] != relative or receipt.get('sha256') != hashlib.sha256(path.read_bytes()).hexdigest()):
                    raise RuntimeError('completed coverage summary changed')
                self.allowed.add(relative)
        guard(self.sha, self.tag, self.allowed)

    def wait_receipt(self, name, timeout=21600):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            self.check()
            path = self.other / name
            if path.exists():
                receipt = read(path)
                if receipt.get('sha') != self.sha or receipt.get('tag') != self.tag or receipt.get('exit') != 0:
                    raise RuntimeError('invalid peer receipt ' + name)
                return receipt
            peer = self.other / 'Result.json'
            if peer.exists() and read(peer).get('exit') not in (None, 0):
                raise RuntimeError('peer owner failed before ' + name)
            time.sleep(15)
        raise RuntimeError('peer receipt timeout ' + name)

    def phase(self, name, command):
        self.check()
        prior = self.folder / (name + '.json')
        if self.resume and prior.exists():
            receipt = read(prior)
            if (receipt.get('sha') == self.sha and receipt.get('tag') == self.tag and receipt.get('command') == command
                    and receipt.get('state') == 'completed' and receipt.get('exit') == 0
                    and receipt.get('command_exit') == 0 and receipt.get('guard_error') is None
                    and receipt.get('remaining_children') == []):
                self.result['phases'][name] = receipt
                self.save()
                return
            raise RuntimeError('cannot reuse incomplete or failed phase ' + name)
        admission = self.admission
        started = time.time()
        code = 1
        receipt = {'sha': self.sha, 'tag': self.tag, 'command': command,
                   'started': started, 'state': 'running'}
        atomic(self.folder / (name + '.json'), receipt)
        self.check()
        with (self.folder / (name + '.log')).open('wb') as log:
            saved = (os.dup(1), os.dup(2))
            os.dup2(log.fileno(), 1)
            os.dup2(log.fileno(), 2)
            try:
                code = admission.run_owned(command, self.env, None)
            finally:
                os.dup2(saved[0], 1)
                os.dup2(saved[1], 2)
                os.close(saved[0]); os.close(saved[1])
                signal.pthread_sigmask(signal.SIG_SETMASK, set())
        guard_error = None
        try:
            self.check()
        except BaseException as error:
            guard_error = repr(error)
        remaining = sorted(admission.child_pids())
        receipt.update(state='completed', command_exit=code, exit=code if not guard_error and not remaining else 1, guard_error=guard_error, finished=time.time(), remaining_children=remaining)
        atomic(self.folder / (name + '.json'), receipt)
        self.result['phases'][name] = receipt
        self.save()
        if receipt['exit'] != 0:
            raise RuntimeError(name + ' failed: exit ' + str(code) + '; remaining ' + repr(remaining))

    def validate_matrix(self):
        retained = self.folder / 'fullcases-parity'
        path = retained / 'matrix.json' if self.resume and retained.exists() else ROOT / 'target/parity/matrix.json'
        rows = read(path)
        spec = importlib.util.spec_from_file_location('candidate_parity', ROOT / 'conformance/parity.py')
        parity = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(parity)
        inventory = parity.inventory()
        expected = {r['id'] for r in (inventory['consumers'] + inventory.get('pending_consumers', []))}
        if {r['consumer'] for r in rows} != expected or len(rows) != len(expected):
            raise RuntimeError('parity matrix does not contain every consumer exactly once')
        pending = set()
        for row in rows:
            if set(row['cells']) != set(parity.required_cases(inventory, row['consumer'])):
                raise RuntimeError('required parity assertions missing: ' + row['consumer'])
            if row.get('platform_pending'):
                pending.add(row['consumer'])
                if row['consumer'] not in {'swift', 'objective-c'} or not row['cells'] or set(row['cells'].values()) != {'platform-pending'}:
                    raise RuntimeError('invalid delegated row')
            elif row['baseline_exit'] != 0 or row['baseline_error'] or not row['cells'] or set(row['cells'].values()) != {'pass'}:
                raise RuntimeError('required parity row failed/skipped: ' + row['consumer'])
        if pending != {'swift', 'objective-c'}:
            raise RuntimeError('must retain both delegated Apple rows explicitly')
        if path.parent != retained:
            shutil.copytree(path.parent, retained)
        atomic(self.folder / 'parity-qualified.json', {'sha': self.sha, 'tag': self.tag, 'exit': 0, 'delegated': sorted(pending)})

    def local(self):
        self.admission = load_admission()
        for signum in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
            signal.signal(signum, self.admission.interrupted)
        os.sched_setaffinity(0, sorted(os.sched_getaffinity(0))[:16])
        self.env = environment(self.folder, self.sha)
        proof = read(self.folder / 'native-proof.json')
        if proof.get('sha') != self.sha or proof.get('tag') != self.tag or proof.get('exit') != 0:
            raise RuntimeError('missing same-SHA native Apple proof')
        self.env['THINKTHEN_MACOS_APPLE_CHECKS'] = 'required'
        prior = ROOT / 'target/parity'
        if prior.exists() and not (self.resume and (self.folder / 'prior-parity').exists()):
            shutil.copytree(prior, self.folder / 'prior-parity')
        self.phase('fullcases', ['sh', 'sdlc/scripts/test-full-cases', '--run'])
        self.validate_matrix()
        self.phase('package', ['sh', 'sdlc/scripts/package'])
        self.phase('stress', ['sh', 'sdlc/scripts/test-stress', '--run'])
        reused_coverage = self.resume and (self.folder / 'coverage.json').exists()
        if COVERAGE.exists() and not reused_coverage:
            raise RuntimeError('canonical coverage output already exists; preserve it')
        if list((ROOT / 'sdlc/records/coverage').glob('*-' + self.sha[:9] + '-linux.md')) and not reused_coverage:
            raise RuntimeError('candidate coverage summary already exists; preserve it')
        if not reused_coverage:
            atomic(self.folder / 'coverage-expected.json', {'sha': self.sha, 'tag': self.tag, 'platform': 'linux'})
        self.phase('coverage', ['python3', 'sdlc/scripts/coverage', '--release', '--platform', 'linux', '--output', str(COVERAGE)])
        candidates = list((ROOT / 'sdlc/records/coverage').glob('*-' + self.sha[:9] + '-linux.md'))
        if len(candidates) != 1:
            raise RuntimeError('coverage did not create exactly one same-SHA Linux summary')
        summary = str(candidates[0].relative_to(ROOT))
        self.check()
        atomic(self.folder / 'coverage-expected.json', {'sha': self.sha, 'tag': self.tag, 'platform': 'linux', 'summary': summary, 'sha256': hashlib.sha256((ROOT / summary).read_bytes()).hexdigest()})
        shutil.copy2(ROOT / summary, self.folder / 'coverage-summary.md')
        payload = read(COVERAGE / 'totals.json')
        if payload.get('source') != self.sha or payload.get('platform_collection') != 'linux' or payload.get('unmeasured_collections'):
            raise RuntimeError('invalid Linux coverage report')
        atomic(self.folder / 'linux-ready.json', {'sha': self.sha, 'tag': self.tag, 'exit': 0, 'path': str(COVERAGE / 'totals.json'), 'summary': summary})
        self.wait_receipt('apple-ready.json')
        self.phase('coverage-join', ['python3', 'sdlc/scripts/coverage', '--release', '--join', str(COVERAGE / 'totals.json'), str(self.other / 'apple-coverage/totals.json')])
        atomic(self.folder / 'join-ready.json', {'sha': self.sha, 'tag': self.tag, 'exit': 0})

    def gh(self, *args):
        return output(['gh', *args])

    def cli_compatible(self):
        required = {
            ('api',): ('--paginate',),
            ('workflow', 'run'): ('--repo', '--ref', '-f'),
            ('run', 'view'): ('--repo', '--log'),
            ('run', 'download'): ('--repo', '--name', '--dir'),
        }
        for command, flags in required.items():
            help_text = self.gh(*command, '--help')
            tokens = set(re.findall(r'(?<![\w-])--?[a-z][a-z-]*', help_text))
            if not set(flags) <= tokens:
                raise RuntimeError('installed gh lacks required flags for ' + ' '.join(command))

    def api(self, path):
        for attempt in range(4):
            try:
                return json.loads(self.gh('api', 'repos/' + REPO + '/' + path))
            except subprocess.CalledProcessError:
                if attempt == 3:
                    raise
                time.sleep(5)

    def jobs(self, runid):
        # gh 2.45 emits one JSON document per page; decode the complete stream.
        raw = self.gh('api', '--paginate', 'repos/' + REPO + '/actions/runs/' + str(runid) + '/jobs?per_page=100')
        jobs = parse_job_pages(raw)
        if (type(runid) is not int or runid <= 0 or any(type(job.get('run_id')) is not int
                or job['run_id'] != runid or job.get('head_sha') != self.sha for job in jobs)):
            raise RuntimeError('hosted job run/revision differs')
        return jobs

    def validate_jobs(self, jobs):
        required = {'resolve', 'apple-coverage', 'wheels', 'gems', 'npm-pack', 'crate', 'registries', 'windows-sdks', 'draft'}
        publishing = {'crates', 'pypi', 'npm', 'rubygems', 'tap', 'nuget', 'maven', 'pub', 'publish'}
        all_names = [j['name'] for j in jobs]
        if len(all_names) != len(set(all_names)) or not publishing <= set(all_names):
            raise RuntimeError('duplicate jobs or missing publishing skip evidence')
        names = {j['name'] for j in jobs if j['conclusion'] == 'success'}
        if not required <= names:
            raise RuntimeError('missing/skipped required hosted jobs: ' + repr(sorted(required - names)))
        runners = {'x86_64-unknown-linux-gnu': 'ubuntu-24.04', 'aarch64-unknown-linux-gnu': 'ubuntu-24.04-arm', 'aarch64-apple-darwin': 'macos-15', 'x86_64-apple-darwin': 'macos-15-intel', 'x86_64-pc-windows-msvc': 'windows-2025'}
        for family in ('build', 'smoke'):
            for target in ('x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu', 'aarch64-apple-darwin', 'x86_64-apple-darwin', 'x86_64-pc-windows-msvc'):
                if not any(j['name'] == family + ' (' + target + ', ' + runners[target] + ')' and runners[target] in j.get('labels', []) and j['conclusion'] == 'success' for j in jobs):
                    raise RuntimeError('missing successful ' + family + ' ' + target)
        for j in jobs:
            if 'self-hosted' in [label.lower() for label in j['labels']]:
                raise RuntimeError('self-hosted job is not allowed')
            if j['name'].split(' (')[0] not in {'crates', 'pypi', 'npm', 'rubygems', 'tap', 'nuget', 'maven', 'pub', 'publish'} and j['conclusion'] != 'success':
                raise RuntimeError('required job failed/skipped: ' + j['name'])
            if j['name'].split(' (')[0] in {'crates', 'pypi', 'npm', 'rubygems', 'tap', 'nuget', 'maven', 'pub', 'publish'} and j['conclusion'] != 'skipped':
                raise RuntimeError('publishing job was not skipped')
        if not self.apple_success(jobs):
            raise RuntimeError('missing Apple architecture checks')

    def apple_success(self, jobs):
        return all(any(j['name'] == 'apple-packages (' + target + ', ' + runner + ')' and runner in j.get('labels', []) and j.get('conclusion') == 'success' for j in jobs)
                   for target, runner in (('aarch64-apple-darwin', 'macos-15'), ('x86_64-apple-darwin', 'macos-15-intel')))

    def resumed_run(self):
        run = read(self.folder / 'accepted.json')
        dispatch = read(self.folder / 'dispatch.json')
        previous = dispatch.get('previous_run_ids')
        if (dispatch.get('sha') != self.sha or dispatch.get('tag') != self.tag
                or dispatch.get('mode') != 'rehearse'
                or run.get('head_sha') != self.sha or run.get('head_branch') != self.tag
                or run.get('event') != 'workflow_dispatch'
                or run.get('path') != '.github/workflows/release.yml'
                or type(run.get('id')) is not int or run['id'] <= 0
                or not isinstance(previous, list)
                or any(type(value) is not int or value <= 0 for value in previous)
                or run['id'] in previous):
            raise RuntimeError('invalid existing rehearsal identity')
        for name in ('accepted.json', 'completed.json'):
            path = self.folder / name
            if not path.exists():
                continue
            value = read(path)
            if (value.get('id') != run['id'] or value.get('head_sha') != self.sha
                    or value.get('head_branch') != self.tag
                    or value.get('event') != 'workflow_dispatch'
                    or value.get('path') != '.github/workflows/release.yml'):
                raise RuntimeError('persisted hosted run identity differs')
            if value.get('conclusion') not in (None, 'success'):
                raise RuntimeError('known genuine hosted failure forbids recovery launch')
        for name in ('jobs-latest.json', 'jobs-completed.json'):
            path = self.folder / name
            if not path.exists():
                continue
            jobs = read(path)
            if not isinstance(jobs, list) or any(not isinstance(job, dict)
                    or type(job.get('run_id')) is not int or job['run_id'] != run['id']
                    or job.get('head_sha') != self.sha for job in jobs):
                raise RuntimeError('persisted hosted jobs identity differs')
            if any(job.get('conclusion') not in (None, 'success', 'skipped') for job in jobs):
                raise RuntimeError('known genuine hosted job failure forbids recovery launch')
            if name == 'jobs-completed.json':
                self.validate_jobs(jobs)
        return run

    def hosted(self):
        if self.resume:
            run = self.resumed_run()
            runid = run['id']
            deadline = time.monotonic() + 21600
        else:
            before = {r['id'] for r in self.api('actions/workflows/release.yml/runs?event=workflow_dispatch&per_page=100')['workflow_runs']}
            self.check()
            self.gh('workflow', 'run', 'release.yml', '--repo', REPO, '--ref', self.tag, '-f', 'mode=rehearse')
            atomic(self.folder / 'dispatch.json', {'sha': self.sha, 'tag': self.tag, 'mode': 'rehearse', 'previous_run_ids': sorted(before)})
            self.check()
            deadline = time.monotonic() + 21600
            run = None
            while time.monotonic() < deadline:
                self.check()
                matches = [r for r in self.api('actions/workflows/release.yml/runs?event=workflow_dispatch&per_page=100')['workflow_runs']
                           if r['id'] not in before and r['head_sha'] == self.sha and r['head_branch'] == self.tag]
                if len(matches) > 1:
                    raise RuntimeError('ambiguous same-SHA dispatch; refuse to choose another run')
                if matches:
                    run = matches[0]
                    break
                time.sleep(5)
            if run is None:
                raise RuntimeError('no unique same-SHA candidate-tag run')
            runid = run['id']
            atomic(self.folder / 'accepted.json', run)
        while time.monotonic() < deadline:
            self.check()
            run = self.api('actions/runs/' + str(runid))
            if (type(run.get('id')) is not int or run['id'] != runid
                    or run.get('path') != '.github/workflows/release.yml'
                    or run['head_sha'] != self.sha or run['head_branch'] != self.tag or run['event'] != 'workflow_dispatch'):
                raise RuntimeError('hosted run identity changed')
            jobs = self.jobs(runid)
            atomic(self.folder / 'jobs-latest.json', jobs)
            if self.apple_success(jobs):
                atomic(self.folder / 'apple-qualified.json', {'sha': self.sha, 'tag': self.tag, 'exit': 0, 'run_id': runid, 'jobs': [j for j in jobs if j['name'].startswith('apple-packages (')]})
            if run['status'] == 'completed':
                break
            time.sleep(30)
        else:
            raise RuntimeError('hosted completion timeout')
        atomic(self.folder / 'completed.json', run)
        atomic(self.folder / 'jobs-completed.json', jobs)
        with (self.folder / 'finished.log').open('w') as log:
            done = subprocess.run(['gh', 'run', 'view', str(runid), '--repo', REPO, '--log'], cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
        if done.returncode:
            raise RuntimeError('cannot capture completed hosted logs')
        if run['conclusion'] != 'success':
            raise RuntimeError('hosted rehearsal failed: ' + str(run['conclusion']))
        self.validate_jobs(jobs)
        self.check()
        self.gh('run', 'download', str(runid), '--repo', REPO, '--name', 'apple-coverage-' + self.sha, '--dir', str(self.folder / 'apple-coverage'))
        payload = read(self.folder / 'apple-coverage/totals.json')
        if payload.get('source') != self.sha or payload.get('platform_collection') != 'apple' or payload.get('unmeasured_collections'):
            raise RuntimeError('downloaded Apple report differs')
        atomic(self.folder / 'apple-ready.json', {'sha': self.sha, 'tag': self.tag, 'exit': 0, 'run_id': runid})
        self.wait_receipt('linux-ready.json')
        self.wait_receipt('join-ready.json')
        self.result['run_id'] = runid

    def run(self):
        os.chdir(ROOT)
        self.owner_lock = open(self.folder / 'owner.lock', 'a+b')
        fcntl.flock(self.owner_lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        previous = self.folder / 'Result.json'
        if previous.exists():
            if not self.resume:
                raise RuntimeError('existing campaign requires explicit --resume; never redispatch')
            saved = read(previous)
            if saved.get('hosted_failures'):
                raise RuntimeError('known genuine hosted failures forbid recovery launch')
            if saved.get('sha') != self.sha or saved.get('tag') != self.tag or saved.get('kind') != self.kind:
                raise RuntimeError('resume identity differs')
            prior_pid = int((self.folder / 'PID').read_text().strip())
            if prior_pid <= 0:
                raise RuntimeError('invalid prior campaign owner PID')
            try:
                os.kill(prior_pid, 0)
            except ProcessLookupError:
                pass
            else:
                raise RuntimeError('prior campaign owner is still alive')
            if self.kind == 'hosted':
                self.resumed_run()
            stamp = str(time.time_ns())
            for path in list(self.folder.iterdir()):
                if (path.is_file() and '.before-resume-' not in path.name
                        and (path.suffix in ('.json', '.log') or path.name in ('PID', 'OWNER'))):
                    shutil.copy2(path, self.folder / (path.name + '.before-resume-' + stamp))
            self.result['phases'] = saved.get('phases', {})
            self.result['resumed_from'] = 'Result.json.before-resume-' + stamp
        elif self.resume:
            raise RuntimeError('resume requires preserved campaign receipt')
        atomic_text(self.folder / 'PID', str(os.getpid()) + '\n')
        atomic_text(self.folder / 'OWNER', 'Ticket: 0543\nRole: ' + self.kind + ' candidate campaign owner\nRevision: ' + self.sha + '\nCandidate tag: ' + self.tag + '\nPID: ' + str(os.getpid()) + '\nResult: ' + str(self.folder / 'Result.json') + '\n')
        self.save()
        def interrupted(signum, _frame):
            raise InterruptedError(signum)
        for signum in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
            signal.signal(signum, interrupted)
        try:
            self.cli_compatible()
            self.check()
            getattr(self, self.kind)()
            self.check()
            self.result.update(state='completed', exit=0)
        except BaseException as error:
            self.result.update(state='failed', exit=1, error=repr(error))
            for receipt_path in self.folder.glob('*.json'):
                value = read(receipt_path)
                if isinstance(value, dict) and value.get('state') == 'running' and 'command' in value:
                    value.update(state='failed', exit=1, error=repr(error), finished=time.time())
                    atomic(receipt_path, value)
                    self.result['phases'][receipt_path.stem] = value
        if self.kind == 'local' and hasattr(self, 'admission'):
            self.result['remaining_children'] = sorted(self.admission.child_pids())
            if self.result['remaining_children']:
                self.result.update(state='failed', exit=1)
        self.save()
        return self.result['exit']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('kind', choices=('local', 'hosted'))
    parser.add_argument('sha')
    parser.add_argument('tag', type=candidate_tag)
    parser.add_argument('--resume', action='store_true', help='rejoin the persisted run or reuse successful local phases; never dispatch')
    args = parser.parse_args()
    return Owner(args.sha, args.kind, args.resume, tag=args.tag).run()


if __name__ == '__main__':
    sys.exit(main())
