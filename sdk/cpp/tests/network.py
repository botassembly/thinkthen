"""Exercise every installed C++ network mode through real provider wires."""
from contextlib import contextmanager
import json
import os
from pathlib import Path
import queue
import subprocess
import sys
import tempfile
import threading
import time

consumer, loopback = sys.argv[1:]
names = ('PATH', 'HOME', 'SYSTEMROOT', 'TMPDIR', 'TEMP', 'TMP')
env = {name: os.environ[name] for name in names if name in os.environ}

def line(stream, timeout=30):
    result = queue.Queue()
    threading.Thread(target=lambda: result.put(stream.readline()), daemon=True).start()
    return result.get(timeout=timeout).strip()

@contextmanager
def fixture(*arguments):
    with subprocess.Popen([loopback, *map(str, arguments)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            url = 'http://127.0.0.1:' + line(server.stdout) + '/v1'
            yield server, url
            server.stdin.write('shutdown\n')
            server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait(timeout=30)

def command(server, text, timeout=30):
    server.stdin.write(text + '\n')
    server.stdin.flush()
    return line(server.stdout, timeout)

with tempfile.TemporaryDirectory(prefix='thinkthen-cpp-network-') as temporary:
    case = Path(temporary) / 'choose.json'
    case.write_text(json.dumps({'replies':[{'distribution':{'billing':0.7000000000000001,'shipping':0.29999999999999993}}]}))
    for backend in ('systemone', 'openai'):
        with fixture(case) as (server, url):
            subprocess.run([consumer, 'projection', backend, url], env=env, check=True, timeout=30)
            assert command(server, 'count') == '1'
        with fixture('--held', 'reset') as (server, url):
            subprocess.run([consumer, 'failed-stream', backend, url], env=env, check=True, timeout=30)
            assert command(server, 'count') == '2'
        limits = json.loads(subprocess.check_output([loopback, '--limits'], text=True, timeout=30))
        for mode in ('cancel', 'read-ahead'):
            with fixture('--held', 'headers') as (server, url):
                with subprocess.Popen([consumer, mode, backend, url, *([str(limits['read_ahead_window'] + 1)] if mode == 'read-ahead' else [])], stdin=subprocess.PIPE, text=True, env=env) as child:
                    try:
                        assert command(server, 'arrived ' + str(limits['workers']) if mode == 'read-ahead' else 'arrived') == 'arrived'
                        paid = command(server, 'count')
                        if mode == 'cancel': assert paid == '1'
                        else:
                            assert 1 <= int(paid) <= limits['workers']
                            try:
                                child.wait(timeout=2)
                            except subprocess.TimeoutExpired:
                                pass
                            else:
                                raise AssertionError('held pull stream ended before cancellation')
                            paid = command(server, 'count')
                        started = time.monotonic()
                        child.stdin.write('c')
                        child.stdin.flush()
                        assert command(server, 'closed', timeout=2) == 'closed'
                        assert time.monotonic() - started < 2, 'held cancellation exceeded its contract'
                        assert child.wait(timeout=30) == 0
                        assert command(server, 'count') == paid, 'requests continued after cancellation'
                    finally:
                        if child.poll() is None:
                            child.kill()
                            child.wait(timeout=30)
print('installed C++: projection, failed streams and bounded cancellation pass on both wires')
