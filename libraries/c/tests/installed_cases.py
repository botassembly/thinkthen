"""Run canonical cases through the installed C session packet graph."""
import json
from pathlib import Path
import shlex
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'conformance/children')]
from children import child_env, read_process_line
import session_cases as shared_cases
from session_views import render


def static_flags(native, env):
    """Use the installed package's platform-specific static link contract."""
    env = {**env, 'PKG_CONFIG_PATH': str(native / 'lib/pkgconfig')}
    flags = shlex.split(subprocess.check_output(
        ['pkg-config', '--cflags', '--libs', '--static', 'thinkthen'],
        env=env, text=True))
    # Name the archive explicitly so no installed case can select the shared library.
    return [str(native / 'lib/libthinkthen.a') if flag == '-lthinkthen' else flag
            for flag in flags]


def run(native, scratch, *, header=None, library=None):
    if bool(header) != bool(library):
        raise ValueError('installed C cases require both header and library')
    header = Path(header).resolve(strict=True) if header else native / 'include/thinkthen.h'
    library = Path(library).resolve(strict=True) if library else native / 'lib/libthinkthen.a'
    if not header.is_file() or not library.is_file():
        raise ValueError('installed C cases header or library is missing')
    scratch = Path(scratch)
    (scratch / 'session_views.c').write_text(render())
    binary = scratch / 'session-cases'
    if sys.platform == 'linux' and '.so' in library.name:
        (scratch / 'libthinkthen.so.0').symlink_to(library)
    env = child_env(home=scratch / 'home')
    flags = static_flags(native, env) if native else [str(library)]
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-I', str(header.parent),
                    '-I', str(scratch), str(ROOT / 'libraries/c/tests/c/session_cases.c'),
                    *flags,
                    '-Wl,-rpath,' + str(scratch), '-Wl,-rpath,' + str(library.parent),
                    '-o', str(binary)], env=env, check=True)
    def checked(output):
        if output.returncode == 0 and not output.stderr:
            payload = json.loads(output.stdout)
            if 'packets' in payload:
                assert payload['packets'] == payload['native'], 'typed C view differs from canonical native packet'
        return output
    def invoke(step, settings, env, home, backend):
        source = shared_cases.descriptor(step, home)
        feed = []
        if step.get('incremental') and source['input']['kind'] != 'source':
            feed = [json.dumps({'item':item}) for item in source['input']['items']]
            source['input'] = {'kind':'feed','name':'records'}
        source['call'] = {key: source[key] for key in ('question', 'input', 'options')}
        source['call']['function'] = source['verb']
        path = home / 'session-request.json'
        path.write_text(json.dumps({'schema': 'thinkthen.request/1', 'call': source['call']}))
        args = [str(binary), str(path), json.dumps(settings), 'cancel' if source['cancel'] else '-', 'held' if source['held_cancel'] else '-'] + feed
        if not source['held_cancel']:
            return checked(subprocess.run(args, env=env, cwd=home, capture_output=True, text=True, timeout=120))
        process = subprocess.Popen(args, env=env, cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!'); process.stdin.flush()
            assert read_process_line(process.stdout) == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout, stderr = process.communicate(timeout=60)
            return checked(subprocess.CompletedProcess(args, process.returncode, stdout, stderr))
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None: process.kill(); process.wait()
    expected = json.loads((ROOT / 'conformance/settings-errors.json').read_text())
    refusal = scratch / 'refused-settings.json'
    refusal.write_text('{}')
    output = subprocess.run([str(binary), str(refusal), '{"throttle":33}', '-', '-'],
                            env=env, capture_output=True, text=True, check=True)
    assert not output.stderr, output.stderr
    admission = json.loads(output.stdout)['admission']
    assert admission['code'] == 1, admission
    assert f"thinkthen usage: {admission['message']} (retryable: no)" == expected['throttle'], admission
    shared_cases.native_cases(binary, consumer='c', invoke=invoke)


def smoke(native, scratch):
    """Replay the seed through the same installed typed consumer as shared cases."""
    import os
    scratch = Path(scratch)
    (scratch / 'session_views.c').write_text(render())
    binary = scratch / 'session-smoke'
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror',
                    '-I', str(native / 'include'), '-I', str(scratch),
                    str(ROOT / 'libraries/c/tests/c/session_cases.c'),
                    '-L', str(native / 'lib'), '-lthinkthen',
                    '-Wl,-rpath,' + str(native / 'lib'), '-o', str(binary)],
                   env=child_env(home=scratch / 'home'), check=True)
    request = {'schema': 'thinkthen.request/1', 'call': {'function': 'decide',
               'question': {'kind': 'text', 'text': os.environ['THINKTHEN_TEST_SMOKE_QUESTION']},
               'input': {'kind': 'text', 'text': os.environ['THINKTHEN_TEST_SMOKE_TEXT']}}}
    path = scratch / 'request.json'
    path.write_text(json.dumps(request))
    env = child_env(home=scratch / 'home',
                    THINKTHEN_BASE_URL=os.environ['THINKTHEN_BASE_URL'],
                    THINKTHEN_CACHE=os.environ['THINKTHEN_CACHE'],
                    THINKTHEN_API_KEY='sk-smoke-loopback')
    result = subprocess.run([str(binary), str(path), '{}', '-', '-'],
                            env=env, capture_output=True, text=True, check=True)
    assert not result.stderr, result.stderr
    payload = json.loads(result.stdout)
    assert payload['packets'] == payload['native'], payload
    packets = payload['packets']
    terminal, = [packet for packet in packets if packet['kind'] == 'terminal']
    assert 'failure' not in terminal, terminal
    row, = [packet for packet in packets if packet['kind'] == 'row' and packet['function'] == 'decide']
    answer = row['value']['value']
    assert isinstance(answer, bool), row
    print('smoke:', json.dumps(answer))
