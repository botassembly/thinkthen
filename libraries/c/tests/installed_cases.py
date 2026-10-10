"""Run canonical cases through the installed complete C packet graph."""
import json
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'libraries/cpp/fixtures'), str(ROOT / 'conformance/children')]
from children import child_env
import session_cases as shared_cases
from session_views import render

def run(native, scratch):
    scratch = Path(scratch)
    (scratch / 'session_views.c').write_text(render())
    binary = scratch / 'session-cases'
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-I', str(native / 'include'),
                    '-I', str(scratch), str(ROOT / 'libraries/c/tests/c/session_cases.c'),
                    str(native / 'lib/libthinkthen.a'), '-lgcc_s', '-lutil', '-lrt', '-lpthread', '-lm', '-ldl',
                    '-o', str(binary)], env=child_env(home=scratch / 'home'), check=True)
    def checked(output):
        if output.returncode == 0 and not output.stderr:
            payload = json.loads(output.stdout)
            if 'packets' in payload:
                assert payload['packets'] == payload['native'], 'typed C view differs from canonical native packet'
        return output
    def invoke(step, settings, env, home, backend):
        source = shared_cases.descriptor(step, home)
        source['call'] = {key: source[key] for key in ('question', 'input', 'options')}
        source['call']['function'] = source['verb']
        path = home / 'session-request.json'
        path.write_text(json.dumps({'schema': 'thinkthen.request/1', 'call': source['call']}))
        args = [str(binary), str(path), json.dumps(settings), 'cancel' if source['cancel'] else '-', 'held' if source['held_cancel'] else '-']
        if not source['held_cancel']:
            return checked(subprocess.run(args, env=env, cwd=home, capture_output=True, text=True, timeout=120))
        process = subprocess.Popen(args, env=env, cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!'); process.stdin.flush()
            assert process.stdout.readline() == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout, stderr = process.communicate(timeout=60)
            return checked(subprocess.CompletedProcess(args, process.returncode, stdout, stderr))
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None: process.kill(); process.wait()
    shared_cases.native_cases(binary, consumer='c', invoke=invoke)
