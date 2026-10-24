"""Drive the external shared SQL ABI consumer using the shared held fixture."""
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment

consumer, loopback = sys.argv[1:]
env = child_environment()
subprocess.run([consumer], env=env, check=True)
for mode in ('headers', 'body', 'refusal-body'):
    with subprocess.Popen([loopback, '--held', mode], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            url = f'http://127.0.0.1:{server.stdout.readline().strip()}/v1'
            process_env = dict(env, THINKTHEN_BASE_URL=url, THINKTHEN_API_KEY='loopback-test-key')
            with subprocess.Popen([consumer, 'cancel'], stdin=subprocess.PIPE, env=process_env) as child:
                try:
                    server.stdin.write('arrived\n'); server.stdin.flush()
                    assert server.stdout.readline().strip() == 'arrived'
                    child.stdin.write(b'c'); child.stdin.flush()
                    server.stdin.write('closed\n'); server.stdin.flush()
                    assert server.stdout.readline().strip() == 'closed'
                    assert child.wait(timeout=30) == 0
                finally:
                    if child.poll() is None:
                        child.kill(); child.wait()
            server.stdin.write('shutdown\n'); server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill(); server.wait()
print('shared SQL C boundary passes')
