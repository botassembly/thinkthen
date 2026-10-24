"""Run installed C++ consumers against the shared construction and backend cases."""
import json
import os
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

consumer, construction, loopback, cases = sys.argv[1:]
names = ("HOME", "PATH", "TMPDIR", "TMP", "TEMP", "LLVM_PROFILE_FILE")
env = {key: os.environ[key] for key in names if key in os.environ}
subprocess.run([consumer, "stub", construction], check=True, env=env)
subprocess.run([consumer, "options", str(Path(construction).with_name("options.json"))], check=True, env=env)
network_env = dict(env, THINKTHEN_API_KEY="synthetic-loopback-key")


def command(server, text):
    server.stdin.write(text + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()


@contextmanager
def fixture(*arguments):
    with subprocess.Popen([loopback, *arguments], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            url = "http://127.0.0.1:" + server.stdout.readline().strip() + "/v1"
            yield server, url
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait() == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


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


def byte_offsets(value, text):
    if isinstance(value, dict):
        if "start" in value and "end" in value:
            for key in ("start", "end"):
                value[key] = len(text[:value[key]].encode("utf-8"))
        for child in value.values():
            byte_offsets(child, text)
    elif isinstance(value, list):
        for child in value:
            byte_offsets(child, text)


paths = sorted(Path(cases).glob("*.json"))
assert paths
for path in paths:
    test = json.loads(path.read_text())
    expected = test["expected"]
    request = test["request"]
    if isinstance(expected, list) and request["function"] in ("find", "recognize", "relate"):
        for row, record in zip(expected, request["records"]):
            if isinstance(record, str):
                byte_offsets(row.get("answer"), record)
                byte_offsets(row.get("detail"), record)
    with fixture(str(path)) as (server, url):
        result = subprocess.run([consumer, "case", str(path), url], env=network_env,
                                check=True, capture_output=True, text=True)
        assert json.loads(result.stdout) == expected, (path.name, result.stdout, expected)
        assert int(command(server, "count")) == test["requests"], path.name
print(f"installed C++ SDK: all ten stub functions, boundary cases and {len(paths)} shared cases pass")
