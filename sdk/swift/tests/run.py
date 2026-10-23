"""Drive a moved installed Swift consumer with shared cases and fixture events."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from contextlib import contextmanager

consumer, root, loopback = sys.argv[1:]
root = Path(root)
env = {key: os.environ[key] for key in ("PATH", "HOME", "TMPDIR", "LLVM_PROFILE_FILE") if key in os.environ}
network_env = dict(env, THINKTHEN_API_KEY="loopback-test-key")


def normalize(value):
    if isinstance(value, dict):
        error = value.get("error")
        if isinstance(error, dict):
            error.pop("message", None)
        for child in value.values():
            normalize(child)
    elif isinstance(value, list):
        for child in value:
            normalize(child)
    return value


def command(server, text):
    server.stdin.write(text + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()


@contextmanager
def fixture(*args):
    server = subprocess.Popen([loopback, *args], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env)
    try:
        url = f"http://127.0.0.1:{int(server.stdout.readline())}/v1"
        yield server, url
        server.stdin.write("shutdown\n")
        server.stdin.flush()
        assert server.wait() == 0
    finally:
        if server.poll() is None:
            server.kill()
            server.wait()


subprocess.run([str(Path(consumer).with_name("Readme"))], env=env, check=True, capture_output=True)

stub = root / "construction/stub.json"
result = subprocess.run([consumer, str(stub)], env=env, check=True, capture_output=True, text=True)
actual = [json.loads(line) for line in result.stdout.splitlines()]
expected = [case["expected"] for case in json.loads(stub.read_text())]
assert normalize(actual) == normalize(expected), (actual, expected)
# Typed options cannot express the refused threshold fixtures. Compare the
# admitted public calls, including saved domains and rank levels, as whole rows.
with tempfile.TemporaryDirectory() as folder:
    path = Path(folder) / "options.json"
    for case in json.loads((root / "construction/options.json").read_text()):
        if "equivalent_request" not in case:
            continue
        pair = [{"client": case["client"], "request": case[key]}
                for key in ("request", "equivalent_request")]
        path.write_text(json.dumps(pair))
        result = subprocess.run([consumer, str(path)], env=env, check=True, capture_output=True, text=True)
        actual, equivalent = map(json.loads, result.stdout.splitlines())
        assert isinstance(actual, list) and len(actual) == len(case["request"]["records"]), case["name"]
        assert actual == equivalent, (case["name"], actual, equivalent)

for path in sorted((root / "conformance").glob("*.json")):
    case = json.loads(path.read_text())
    with fixture(str(path)) as (server, url):
        result = subprocess.run([consumer, str(path), url], env=network_env, check=True, capture_output=True, text=True)
        actual = normalize(json.loads(result.stdout))
        if "expected_rows" in case:
            assert actual["rows"] == normalize(case["expected_rows"]), (path.name, actual)
        if isinstance(actual, dict):
            actual.pop("rows", None)
        assert actual == normalize(case["expected"]), (path.name, actual, case["expected"])
        assert int(command(server, "count")) == case["requests"], path.name
with fixture("--unavailable") as (server, url):
    subprocess.run([consumer, "backend", url], env=network_env, check=True)
    assert command(server, "count") == "0"
for stage in ("headers", "body", "refusal-body"):
    with fixture("--held", stage) as (server, url):
        child = subprocess.Popen([consumer, "cancel", url], stdin=subprocess.PIPE, env=network_env)
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
result = subprocess.run([consumer, "surface"], env=env, check=True, capture_output=True, text=True)
identifier, annotation = map(json.loads, result.stdout.splitlines())
assert identifier == {"é": 1, "e\u0301": 2, "a\0b": 18446744073709551615}, identifier
assert set(annotation) == {"é", "e\u0301"}, annotation
print("Installed Swift consumer: all ten stub functions, shared conformance, task cancellation, source lifetime, traversal ownership and raw JSON pass")
