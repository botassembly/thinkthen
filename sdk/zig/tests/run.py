"""Run installed Zig public functions against shared fixtures and host edges."""
import json
import re
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

root = Path(sys.argv[2])
sys.path.insert(0, str(root / "databases"))
from testing import child_environment

consumer, _, loopback = sys.argv[1:]
if sys.platform == "linux":
    dynamic = subprocess.check_output(["readelf", "-d", consumer], text=True)
    search_paths = re.findall(r"\((?:RUNPATH|RPATH)\).*?\[(.*?)\]", dynamic)
    assert search_paths == ["$ORIGIN"], search_paths
else:
    dynamic = subprocess.check_output(["otool", "-l", consumer], text=True)
    search_paths = re.findall(r"cmd LC_RPATH\s+cmdsize \d+\s+path (\S+)", dynamic)
    assert search_paths == ["@loader_path"], search_paths
env = child_environment()
env["THINKTHEN_API_KEY"] = "loopback-test-key"

@contextmanager
def fixture(executable, *arguments):
    with subprocess.Popen([executable, *arguments], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            url = f"http://127.0.0.1:{int(server.stdout.readline())}/v1"
            yield server, url
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()

def command(server, value):
    server.stdin.write(value + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()

def call(settings, request, cancelled=False):
    return json.loads(subprocess.check_output(
        [consumer, "run", json.dumps(settings), json.dumps(request)] + (["cancelled"] if cancelled else []), env=env))

def clean(value, function):
    if isinstance(value, list):
        for row in value:
            if isinstance(row.get("error"), dict):
                row["error"].pop("message", None)
            if function == "annotate" and isinstance(row.get("answer"), dict):
                for entry in row["answer"].values():
                    if isinstance(entry.get("error"), dict):
                        entry["error"].pop("message", None)
    return value

def offsets(value, text):
    if isinstance(value, dict):
        if "start" in value and "end" in value:
            for key in ("start", "end"):
                value[key] = len(text[:value[key]].encode())
        for child in value.values():
            offsets(child, text)
    elif isinstance(value, list):
        for child in value:
            offsets(child, text)

for case in json.loads((root / "construction/stub.json").read_text()):
    assert call({"adapter": "stub"}, case["request"]) == case["expected"]
for case in json.loads((root / "construction/options.json").read_text()):
    actual = call(case["client"], case["request"])
    if "equivalent_request" in case:
        expected = call(case["client"], case["equivalent_request"])
        assert len(actual) == len(case["equivalent_request"]["records"]), case["name"]
        assert actual == expected, case["name"]
    else:
        assert actual == {"error": case["expected_error"]}, (case["name"], actual)
subprocess.run([consumer, "ownership"], env=env, check=True)
for path in sorted((root / "conformance").glob("*.json")):
    case = json.loads(path.read_text())
    expected = case["expected"]
    request = case["request"]
    if isinstance(expected, list) and request["function"] in ("find", "recognize", "relate"):
        for row, record in zip(expected, request["records"]):
            if isinstance(record, str):
                offsets(row.get("answer"), record)
                offsets(row.get("detail"), record)
    with fixture(loopback, str(path)) as (server, url):
        actual = call(dict(case.get("client", {}), url=url), request, case.get("cancelled", False))
        if "expected_rows" in case:
            assert clean(actual["rows"], request["function"]) == case["expected_rows"], (path.name, actual)
        if isinstance(actual, dict):
            actual.pop("rows", None)
        assert clean(actual, request["function"]) == expected, (path.name, actual, expected)
        assert int(command(server, "count")) == case["requests"], path.name
with fixture(loopback, "--unavailable") as (server, url):
    actual = call({"url": url}, {"function": "decide", "question": {"type": "inline", "value": "Ready?"}, "records": ["evidence"]})
    assert actual["error"] == "backend.transport", actual
    assert len(actual["rows"]) == 1 and actual["rows"][0]["error"]["code"] == "backend.transport", actual
for phase in ("headers", "body", "refusal-body"):
    with fixture(loopback, "--held", phase) as (server, url):
        with subprocess.Popen([consumer, "held", json.dumps({"url": url})], stdin=subprocess.PIPE, env=env) as child:
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
print("installed Zig: all ten stub functions, shared cases, ownership, allocation and held cancellation passed")
