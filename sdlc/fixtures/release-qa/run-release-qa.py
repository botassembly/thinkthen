#!/usr/bin/python3
"""Run installed approved-candidate runtime QA after the completed candidate campaign.

No builds, downloads, dispatch, publication, or provider calls. Source is 0425's
public-contract fixtures, then 0543's candidate identity. Requires Linux bwrap.
The campaign identity JSON is supplied from the completed artifact campaign:
  source_commit, candidate_tag, version, build_profile (release), artifact_kind
  (release-command), archive_name, archive_sha256, fixture_source_commit,
  fixture_sha256. It must describe the actual final artifacts, never a rehearsal
  or an older checkpoint. Its authenticity is the executing agent's responsibility.
The command archive does not itself embed a commit. A checksum alone proves no
source identity. This input must come from the campaign, not be inferred from
--version or written merely to satisfy this runner.

The fixture backend is an existing candidate-matched conformance-backend; it is
only a local deterministic responder, never the installed product under test.
All required MCP conformance cells run once, including existing image assertions.
Image results establish transport/order/provenance, never recognition accuracy.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tarfile
import tempfile
import traceback

BASE = Path('/tmp/thinkthen-0425-rc13-qa')
OLD_SHA = '25a0fdef5d281a25e627f0c42ae93fe0de2be622a84de57010efed521c5d5224'
OLD_MTIME = 1792030602041556619
DB = BASE / 'cache/thinkthen.sqlite'


def digest(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def identity(path):
    stat = path.stat()
    return {'sha256': digest(path), 'bytes': stat.st_size, 'mtime_ns': stat.st_mtime_ns}


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args], text=True).strip()


def cli():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    for name in ('repo', 'candidate-sha', 'candidate-tag', 'version', 'archive', 'archive-sha256',
                 'binary', 'campaign-identity', 'fixture-backend', 'fixture-sha256'):
        parser.add_argument('--' + name, required=True)
    return parser.parse_args()


def preflight(a):
    repo, archive, binary, backend = map(Path, (a.repo, a.archive, a.binary, a.fixture_backend))
    for path in (repo, archive, binary, backend, Path(a.campaign_identity)):
        assert path.is_absolute() and path.exists(), 'all artifact/source paths must be absolute and present'
    assert (BASE / 'OWNER').is_file(), 'existing owned fixture required'
    assert re.fullmatch('[0-9a-f]{40}', a.candidate_sha), 'full final candidate SHA required'
    tag_match = re.fullmatch(r'rc/0[.]2[.]0-rc[.]([1-9][0-9]*)', a.candidate_tag)
    assert tag_match and int(tag_match[1]) >= 14, 'explicit approved rc.14 or later tag required'
    assert a.version == '0.2.0', 'release version must be 0.2.0'
    assert git(repo, 'rev-parse', 'HEAD') == a.candidate_sha, 'fixture checkout differs from candidate'
    assert not git(repo, 'status', '--porcelain'), 'fixture checkout is dirty'
    assert git(repo, 'rev-parse', a.candidate_tag + '^{commit}') == a.candidate_sha, 'approved candidate tag absent or different'
    assert git(repo, 'merge-base', 'e7048c85d', a.candidate_sha) == git(repo, 'rev-parse', 'e7048c85d'), 'candidate predates rc.13 repair starting point'
    assert a.candidate_sha != git(repo, 'rev-parse', 'e7048c85d'), 'unrepaired checkpoint refused'
    name = 'thinkthen-0.2.0-x86_64-unknown-linux-musl.tar.gz'
    assert archive.name == name, 'only actual Linux release command archive accepted'
    for path in (archive, binary):
        assert not any(word in str(path).lower() for word in ('rehearsal', 'checkpoint', '/debug/')), 'old/debug/rehearsal artifact refused'
    assert re.fullmatch('[0-9a-f]{64}', a.archive_sha256) and digest(archive) == a.archive_sha256, 'archive checksum differs'
    receipt = json.loads(Path(a.campaign_identity).read_text())
    required = {'source_commit': a.candidate_sha, 'candidate_tag': a.candidate_tag, 'version': a.version,
                'build_profile': 'release', 'artifact_kind': 'release-command',
                'archive_name': name, 'archive_sha256': a.archive_sha256,
                'fixture_source_commit': a.candidate_sha, 'fixture_sha256': a.fixture_sha256}
    assert all(receipt.get(k) == v for k, v in required.items()), 'campaign identity does not describe the final release artifacts'
    assert re.fullmatch('[0-9a-f]{64}', a.fixture_sha256) and digest(backend) == a.fixture_sha256, 'fixture checksum differs'
    with tarfile.open(archive, 'r:gz') as packed:
        members = [m for m in packed.getmembers() if m.name.removeprefix('./') == 'thinkthen']
        assert len(members) == 1 and members[0].isfile(), 'archive lacks one regular release binary'
        with packed.extractfile(members[0]) as stream:
            assert hashlib.file_digest(stream, 'sha256').hexdigest() == digest(binary), 'installed binary differs from archive'
    original = json.loads((BASE / 'result.json').read_text())
    assert digest(DB) == OLD_SHA and DB.stat().st_mtime_ns == OLD_MTIME, 'original cache has changed'
    for entry in original['original_question_files']:
        path = Path(entry['path'])
        assert identity(path) == {k: entry[k] for k in ('sha256', 'bytes', 'mtime_ns')}, 'original question changed'
    return required


def preserved_inputs():
    original = json.loads((BASE / 'result.json').read_text())
    paths = [BASE / 'result.json', DB, *[Path(e['path']) for e in original['original_question_files']]]
    return {str(path): identity(path) for path in paths}


def assert_preserved(before):
    assert all(identity(Path(path)) == expected for path, expected in before.items()), 'original fixture bytes or mtime changed'


def reap_owned(processes):
    # Every launch has its own session. Retire descendants even if their leader
    # already exited, then reap every direct child before reporting completion.
    for proc in reversed(processes):
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait()


def runtime(a, result):
    # Child_env explicitly removes real keys/proxies; only synthetic loopback keys
    # are supplied through the repository's existing conformance helper.
    repo = Path(a.repo)
    sys.path[:0] = [str(repo / 'libraries/mcp'), str(repo / 'conformance'), str(repo / 'conformance/children')]
    from children import child_env
    import installed
    import conformance as mcp
    from c_parity import Backend
    import parity
    tempfile.tempdir = str(BASE)
    env = child_env(home=BASE / 'runtime-home', LANG='C.UTF-8', LC_ALL='C.UTF-8',
                    THINKTHEN_API_KEY='sk-conformance-loopback',
                    LIQUIDAI_API_KEY='sk-conformance-loopback',
                    PERPLEXITY_API_KEY='sk-conformance-loopback',
                    OPENROUTER_API_KEY='sk-conformance-loopback')
    for directory in ('runtime-home', 'runtime-config', 'runtime-state', 'runtime-cache'):
        (BASE / directory).mkdir(exist_ok=True, mode=0o700)
    env.update(XDG_CONFIG_HOME=str(BASE / 'runtime-config'), XDG_STATE_HOME=str(BASE / 'runtime-state'),
               XDG_CACHE_HOME=str(BASE / 'runtime-cache'), PYTHONDONTWRITEBYTECODE='1')
    def command(args, label):
        completed = subprocess.run([a.binary, *args], cwd=BASE, env=env, capture_output=True, timeout=60)
        (BASE / (label + '.stdout')).write_bytes(completed.stdout)
        (BASE / (label + '.stderr')).write_bytes(completed.stderr)
        return completed
    version = command(['--version'], 'runtime-version')
    assert version.returncode == 0 and version.stdout.strip() == b'thinkthen 0.2.0', 'installed version differs'
    fixture = Backend(Path(a.fixture_backend), env)
    try:
        url = 'http://127.0.0.1:%s/arm/full/capture/v1' % fixture.port
        for filename in ('original-refund.json', 'original-probe-question.json'):
            for form, selector in (('absolute', '@' + str(BASE / filename)), ('relative', '@' + filename)):
                before = int(fixture.read('count'))
                run = command(['decide', selector, '--cache', str(BASE / 'cache'), '--input', str(BASE / 'input.txt'),
                               '--url', url, '--model', 'jev-latest', '--facts', '--max-retries', '0'],
                              'runtime-refusal-' + filename + '-' + form)
                assert run.returncode == 5 and run.stdout == b'', 'old cache must refuse locally with exit 5 and empty stdout'
                assert b'cache' in run.stderr.lower(), 'refusal must identify cache'
                assert int(fixture.read('count')) == before, 'old cache refusal sent a request'
                assert digest(DB) == OLD_SHA and DB.stat().st_mtime_ns == OLD_MTIME, 'old DB bytes or mtime changed'
                # The original 0.1.2 documents must also resolve on a clean current store.
                before = int(fixture.read('count'))
                run = command(['decide', selector, '--no-cache', '--input', str(BASE / 'input.txt'), '--url', url,
                               '--model', 'jev-latest', '--max-retries', '0'], 'runtime-question-' + filename + '-' + form)
                assert run.returncode == 0 and run.stdout.strip() == b'true', 'original question did not resolve'
                assert int(fixture.read('count')) == before + 1, 'question check must make exactly one deterministic local request'
    finally:
        fixture.close()
    result['old_cache'] = {'refusal_exit': 5, 'stdout_bytes': 0, 'loopback_requests': 0, **identity(DB)}
    result['original_questions'] = 'both original files, absolute and relative selectors'
    installed.run(a.binary)
    result['installed_mcp_replay'] = 'pass'
    # Reuse the canonical runner and assertions. Only replace its hard-coded
    # source-tree backend selection with the campaign-supplied owned fixture.
    mcp.Backend = lambda unused, child: Backend(Path(a.fixture_backend), child)
    assert not mcp.run('1', a.binary), 'installed MCP conformance failed'
    result['mcp_required_cells'] = len(parity.required_cases(parity.inventory(), 'mcp'))
    result['image_scope'] = 'canonical deterministic image pack, order and provenance; no model accuracy claim'
    subprocess.run(['/usr/bin/python3', str(BASE / 'read-catalog.py'), a.binary, str(repo)],
                   env=env, cwd=BASE, check=True, timeout=45)
    result['catalog'] = json.loads((BASE / 'runtime-catalog.json').read_text())


def main(*, isolated=False):
    a = cli()
    if not isolated:
        # Retire any prior result before execution, so an interrupted rerun can
        # never leave a prior PASS presented as its result. Preserve result.json.
        pending = BASE / 'runtime-result.json.tmp'
        pending.write_text(json.dumps({'result': 'INCOMPLETE', 'published': False,
                                      'candidate_sha': a.candidate_sha}) + '\n')
        os.replace(pending, BASE / 'runtime-result.json')
        # A new network namespace contains every descendant. bwrap configures its
        # loopback interface; no host/provider network is reachable. The original
        # cache directory stays writable so this tests actual application refusal.
        env = {'PATH': '/usr/bin:/bin', 'HOME': str(BASE / 'runtime-home'), 'LANG': 'C.UTF-8',
               'PYTHONDONTWRITEBYTECODE': '1'}
        # Exercise the actual kernel namespace and loopback support before QA.
        try:
            probe = subprocess.run(['/usr/bin/bwrap', '--unshare-net', '--unshare-pid',
                                    '--die-with-parent', '--new-session', '--ro-bind', '/', '/',
                                    '--proc', '/proc', '--', '/usr/bin/python3', '-c',
                                    'import socket; s=socket.socket(); s.bind(("127.0.0.1",0))'],
                                   env=env, capture_output=True, timeout=15)
            probe_error = probe.stderr.decode(errors='replace') if probe.returncode else None
        except (OSError, subprocess.TimeoutExpired) as error:
            probe_error = str(error)
        if probe_error is not None:
            failure = {'result': 'FAIL', 'published': False, 'error': 'network namespace preflight failed',
                       'detail': probe_error, 'candidate_not_run': True}
            pending.write_text(json.dumps(failure, indent=2) + '\n')
            os.replace(pending, BASE / 'runtime-result.json')
            raise SystemExit('network namespace preflight failed; candidate not run')
        cmd = ['/usr/bin/bwrap', '--unshare-net', '--unshare-pid', '--die-with-parent', '--new-session',
               '--ro-bind', '/', '/', '--bind', str(BASE), str(BASE), '--proc', '/proc',
               '--', '/usr/bin/python3', '-c',
               'import runpy; runpy.run_path(' + repr(str(Path(__file__).resolve())) + ')["main"](isolated=True)',
               *sys.argv[1:]]
        with open(BASE / 'runtime.log', 'w') as log:
            proc = subprocess.Popen(cmd, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                code = proc.wait(timeout=3600)
            except BaseException as error:
                os.killpg(proc.pid, signal.SIGTERM)
                try:
                    proc.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(proc.pid, signal.SIGKILL)
                    proc.wait()
                failed = BASE / 'runtime-result.json.tmp'
                failed.write_text(json.dumps({'result': 'FAIL', 'published': False,
                                              'error': type(error).__name__, 'owned_namespace_reaped': True}) + '\n')
                os.replace(failed, BASE / 'runtime-result.json')
                raise
        # Namespace exit also retires any grandchildren whose direct owner was
        # interrupted before its own finally block could run.
        final = json.loads((BASE / 'runtime-result.json').read_text())
        final['owned_namespace_reaped'] = True
        if code or final.get('result') != 'PASS':
            final['result'] = 'FAIL'
            code = code or 1
        pending.write_text(json.dumps(final, indent=2) + '\n')
        os.replace(pending, BASE / 'runtime-result.json')
        raise SystemExit(code)
    result = {'result': 'FAIL', 'published': False, 'network': 'owned network namespace; loopback only'}
    code = 1
    before = preserved_inputs()
    processes = []
    real_popen = subprocess.Popen

    class OwnedProcess(real_popen):
        def __init__(self, *args, **kwargs):
            kwargs['start_new_session'] = True
            super().__init__(*args, **kwargs)
            processes.append(self)

    subprocess.Popen = OwnedProcess
    try:
        result['identity'] = preflight(a)
        runtime(a, result)
        result['result'] = 'PASS'
        code = 0
    except BaseException as error:
        result['error'] = type(error).__name__ + ': ' + str(error)
        traceback.print_exc()
    finally:
        subprocess.Popen = real_popen
        try:
            reap_owned(processes)
            result['backend_and_cli_processes_reaped'] = True
            assert_preserved(before)
            result['original_fixture_preserved'] = True
        except BaseException as error:
            code = 1
            result['result'] = 'FAIL'
            result['preservation_or_cleanup_error'] = str(error)
        result['final_database'] = identity(DB)
        temp = BASE / 'runtime-result.json.tmp'
        temp.write_text(json.dumps(result, indent=2) + '\n')
        os.replace(temp, BASE / 'runtime-result.json')
    raise SystemExit(code)


if __name__ == '__main__':
    main()
