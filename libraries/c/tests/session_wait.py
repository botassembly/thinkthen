"""Exercise installed C intake, prompt cancellation and joined settlement."""
from pathlib import Path
import subprocess
import sys
import threading

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from backend import Backend
from children import child_env, read_process_line
from installed_cases import static_flags


def run(native, scratch, *, rounds=1):
    scratch = Path(scratch)
    env = child_env(home=scratch / 'home')
    binary = scratch / 'session-wait'
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pthread',
                    '-I', str(ROOT / 'libraries/c/tests/c'),
                    str(ROOT / 'libraries/c/tests/c/session_wait.c'),
                    *static_flags(native, env), '-o', str(binary)], env=env, check=True)
    for _ in range(rounds):
        for case in ('push', 'try', 'cancel', 'held'):
            arrived = threading.Event()
            class ObservedBackend(Backend):
                def finish_request(self, request, address):
                    arrived.set()
                    super().finish_request(request, address)
            barrier = scratch / 'barrier-wait'
            barrier.mkdir(exist_ok=True)
            release = barrier / 'release-hold-session'
            release.unlink(missing_ok=True)
            backend = ObservedBackend(barrier if case == 'held' else None, key='sk-session-fixture')
            try:
                selected = {**env, 'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.server_port}/generic/v1',
                            'THINKTHEN_API_KEY': 'sk-session-fixture'}
                process = subprocess.Popen([str(binary), case], env=selected,
                                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                           stderr=subprocess.PIPE, text=True)
                try:
                    if case == 'held':
                        assert read_process_line(process.stdout) == 'submitted\n'
                        assert arrived.wait(5), 'provider never received a connection'
                        process.stdin.write('c'); process.stdin.flush()
                        assert read_process_line(process.stdout) == 'cancelled\n'
                        release.touch()
                        process.stdin.write('r'); process.stdin.flush()
                    stdout, stderr = process.communicate(timeout=10)
                    assert process.returncode == 0, (case, stdout, stderr)
                    assert not stderr, (case, stderr)
                    assert len(backend.arrivals) == (0 if case == 'cancel' else 1 if case == 'held' else 24)
                finally:
                    release.touch()
                    if process.poll() is None:
                        process.kill(); process.wait()
            finally:
                release.touch()
                backend.close()
    print('installed C: bounded push-all, try-push, cancel before wait, held cancel and settlement PASS')
