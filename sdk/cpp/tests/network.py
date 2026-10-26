"""Coordinate native C++ cancellation with the shared fixture's request event."""
import os
import subprocess
import sys

consumer, loopback = sys.argv[1:]
names = ("PATH", "HOME", "SYSTEMROOT", "TMPDIR", "TEMP", "TMP")
env = {name: os.environ[name] for name in names if name in os.environ}
for backend in ("systemone", "openai"):
    with subprocess.Popen([loopback, "--held", "headers"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env) as fixture:
        try:
            port = int(fixture.stdout.readline())
            with subprocess.Popen([consumer, "cancel", backend, f"http://127.0.0.1:{port}/v1"], stdin=subprocess.PIPE, text=True, env=env) as child:
                fixture.stdin.write("arrived\n")
                fixture.stdin.flush()
                assert fixture.stdout.readline().strip() == "arrived"
                child.stdin.write("c")
                child.stdin.flush()
                assert child.wait(timeout=15) == 0
            fixture.stdin.write("closed\n")
            fixture.stdin.flush()
            assert fixture.stdout.readline().strip() == "closed"
            fixture.stdin.write("shutdown\n")
            fixture.stdin.flush()
            assert fixture.wait(timeout=15) == 0
        finally:
            if fixture.poll() is None:
                fixture.kill()
                fixture.wait()
print("installed C++: cancellation closes both backend wires")
