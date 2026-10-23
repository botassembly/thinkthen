"""Exercise an installed C consumer with the shared backend fixture protocol."""

import json
import os
import subprocess
import sys
from contextlib import contextmanager
from pathlib import Path

consumer, cases, loopback, driver, shared_cases = sys.argv[1:]
# The installed consumers receive only host paths and a synthetic fixture key.
names = ("PATH", "HOME", "SYSTEMROOT", "TMPDIR", "TEMP", "TMP", "LLVM_PROFILE_FILE")
env = {name: os.environ[name] for name in names if name in os.environ}
subprocess.run([consumer, cases], check=True, env=env)
subprocess.run([driver, "options", str(Path(cases).with_name("options.json"))], check=True, env=env)
network_env = dict(env, THINKTHEN_API_KEY="loopback-test-key")


@contextmanager
def fixture(*arguments):
    with subprocess.Popen([loopback, *arguments], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            assert server.stdout is not None and server.stdin is not None
            port = int(server.stdout.readline())
            yield server, f"http://127.0.0.1:{port}/v1"
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait() == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


def command(server, value):
    server.stdin.write(value + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()


with fixture("--unavailable") as (server, url):
    subprocess.run([consumer, "backend", url], check=True, env=network_env)
    assert command(server, "count") == "0"
for reply in ("headers", "body", "refusal-body"):
    with fixture("--held", reply) as (server, url):
        with subprocess.Popen([consumer, "cancel", url], stdin=subprocess.PIPE,
                              env=network_env) as child:
            try:
                assert command(server, "arrived") == "arrived"
                child.stdin.write(b"c")
                child.stdin.flush()
                assert command(server, "closed") == "closed"
                assert child.wait() == 0
                assert command(server, "count") == "1"
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait()
print("installed C boundary passes without a service")


def byte_offsets(value, text):
    """Shared fixtures count code points; the public C API counts UTF-8 bytes."""
    if isinstance(value, dict):
        if "start" in value and "end" in value:
            for key in ("start", "end"):
                value[key] = len(text[:value[key]].encode("utf-8"))
        for child in value.values():
            byte_offsets(child, text)
    elif isinstance(value, list):
        for child in value:
            byte_offsets(child, text)


paths = sorted(Path(shared_cases).glob("*.json"))
assert paths, "no shared cases"
for path in paths:
    case = json.loads(path.read_text())
    expected = case["expected"]
    request = case["request"]
    if isinstance(expected, list) and request["function"] in ("find", "recognize", "relate"):
        for row, record in zip(expected, request["records"]):
            if isinstance(record, str):
                # Offsets occur in both the answer and raw detail.
                byte_offsets(row.get("answer"), record)
                byte_offsets(row.get("detail"), record)
    with fixture(str(path)) as (server, url):
        # Jansson parses NUL string values but rejects object keys in JSON.
        # Its counted public setter can construct that same input for the SDK.
        arguments = ([consumer, "nul-key", url] if path.name == "usage-nul-question-name.json"
                     else [driver, str(path), url])
        result = subprocess.run(arguments, check=True, env=network_env,
                                capture_output=True, text=True)
        actual = json.loads(result.stdout)
        assert actual == expected, (path.name, actual, expected)
        assert int(command(server, "count")) == case["requests"], path.name
print(f"installed C SDK: {len(paths)} shared conformance cases passed")
