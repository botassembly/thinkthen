"""Bounded public witnesses for held input and byte-distinct saved names."""
from http.server import BaseHTTPRequestHandler, HTTPServer
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from threading import Event, Thread
from urllib.request import Request, urlopen

root = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(root / 'scripts'))
from test_control import Fixture, read_line

consumer = sys.argv[1]
baseline = len(sys.argv) > 2 and sys.argv[2] == '--baseline'
env = dict(os.environ, THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key')
result = subprocess.run([consumer, 'saved-name-probe'], env=env, text=True, capture_output=True, timeout=30)
print('saved-name witness:', result.returncode, result.stdout.strip(), result.stderr.strip(), flush=True)
result.check_returncode()
if not baseline and '--names-baseline' not in sys.argv:
    assert result.stdout.strip() == 'saved name collision refused', result.stdout

with tempfile.TemporaryDirectory(prefix='swift-held-input-') as temporary:
    folder = Path(temporary)
    library = folder / ('audit.dylib' if sys.platform == 'darwin' else 'audit.so')
    flags = ['-dynamiclib'] if sys.platform == 'darwin' else ['-shared', '-fPIC', '-ldl']
    subprocess.run([shutil.which('cc') or 'clang', '-Wall', '-Wextra', '-Werror', *flags,
                    str(root / 'scripts/tests/thread-start-audit.c'), '-o', str(library)], check=True)
    for mode in ('drop', 'cancel'):
        fixture = Fixture([str(root / 'conformance/typed/decide-precision.json')])
        entered = Event()

        class GatedReply(BaseHTTPRequestHandler):
            def do_POST(self):
                body = self.rfile.read(int(self.headers['Content-Length']))
                assert entered.wait(30), 'caller did not suspend next input'
                request = Request(fixture.url.split('/v1')[0] + self.path, data=body,
                                  headers={'Content-Type': 'application/json'})
                with urlopen(request, timeout=30) as reply:
                    payload = reply.read()
                self.send_response(200)
                self.send_header('Content-Length', str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)

            def log_message(self, *_):
                pass

        with HTTPServer(('127.0.0.1', 0), GatedReply) as gate:
            serving = Thread(target=gate.serve_forever)
            serving.start()
            probe_env = dict(env, THINKTHEN_THREAD_AUDIT=str(folder / f'{mode}.log'),
                             THINKTHEN_THREAD_NAME='ThinkThen.row', THINKTHEN_THREAD_IDENTITIES='1',
                             THINKTHEN_THREAD_ENTRY_IMAGE='Foundation' if sys.platform == 'darwin' else 'libFoundation.so')
            probe_env['DYLD_INSERT_LIBRARIES' if sys.platform == 'darwin' else 'LD_PRELOAD'] = str(library)
            child = subprocess.Popen([consumer, 'held-input-cleanup',
                                      f'http://127.0.0.1:{gate.server_port}/v1', mode],
                                     env=probe_env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
            closed = False
            try:
                assert read_line(child.stdout, timeout=30) == 'input\n'
                entered.set()
                assert read_line(child.stdout, timeout=30) == 'row\n', 'first result did not arrive'
                try:
                    closed = read_line(child.stdout, timeout=8) == 'closed\n'
                except TimeoutError:
                    pass
                print(f'held-input {mode}: bridge closed before releasing caller={closed}', flush=True)
            finally:
                # Release caller-owned input even when the native-lifetime assertion fails.
                entered.set()
                child.stdin.write('r')
                child.stdin.flush()
                try:
                    child.wait(timeout=8)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait(timeout=8)
                gate.shutdown()
                serving.join(timeout=30)
                fixture.close()
            assert closed != baseline, (mode, closed, baseline)
            if not baseline:
                assert child.returncode == 0, child.returncode
