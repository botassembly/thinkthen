"""Run the installed mechanical Zig bridge against canonical typed fixtures."""
import copy
import json
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

consumer, root_arg, loopback = sys.argv[1:]
root = Path(root_arg)
sys.path.insert(0, str(root / "databases"))
from testing import child_environment

env = child_environment()
subprocess.run([sys.executable, str(root / "sdk/c/tests/toolbox.py"), consumer, str(root)], env=env, check=True)
env["THINKTHEN_API_KEY"] = env["OPENAI_API_KEY"] = "loopback-test-key"

@contextmanager
def fixture(*arguments):
    with subprocess.Popen([loopback, *arguments], stdin=subprocess.PIPE,
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

def call(settings, request):
    return json.loads(subprocess.check_output(
        [consumer, "run", json.dumps(settings), json.dumps(request)], env=env))

def normalize(row, backend, model):
    if "source" in row:
        source = row.pop("source")
        assert source["backend"] == backend, source
        assert source["model"] == model, source
        assert len(source["question_identity"]) == 64, source
    if "error" in row:
        row["error"].pop("message", None)
    if row["function"] == "annotate" and isinstance(row.get("answer"), dict):
        for child in row["answer"].values():
            normalize(child, backend, model)

def byte_offsets(value, text):
    if isinstance(value, dict):
        if "start" in value and "end" in value:
            for field in ("start", "end"):
                value[field] = len(text[:value[field]].encode("utf-8"))
        for child in value.values():
            byte_offsets(child, text)
    elif isinstance(value, list):
        for child in value:
            byte_offsets(child, text)

subprocess.run([sys.executable, str(root / "scripts/stress-adapters.py"), "--surface", "zig", *(["--full"] if __import__("os").environ.get("THINKTHEN_STRESS_FULL") else []), "--", sys.executable, str(root / "sdk/c/tests/stress.py"), "zig", consumer], env=env | {"THINKTHEN_LOOPBACK": loopback}, check=True)
subprocess.run([consumer, "ownership"], env=env, check=True)
directory = root / "conformance/typed"
manifest = json.loads((directory / "manifest.json").read_text())
seen = set()
for filename in manifest["cases"]:
    case = json.loads((directory / filename).read_text())
    seen.add(case["request"]["question"]["function"])
    for backend in manifest["backends"]:
        with fixture(str(directory / filename)) as (server, url):
            settings = dict(manifest["client"], **case.get("client", {}))
            settings.update(backend=backend, url=url)
            actual = call(settings, case["request"])
            for row in actual if isinstance(actual, list) else []:
                normalize(row, backend, settings["model"])
            expected = copy.deepcopy(case["expected"])
            if case["request"]["question"]["function"] in ("find", "recognize", "relate"):
                for row in expected:
                    record = case["request"]["records"][row["position"] - 1]
                    assert isinstance(record, str), (filename, "offset fixture must supply text")
                    byte_offsets(row.get("answer"), record)
                    byte_offsets(row.get("detail"), record)
            assert actual == expected, (filename, backend, actual, expected)
            assert int(command(server, "count")) == case["requests"], filename
assert len(seen) == 10, seen
case = json.loads((directory / "tag-batching.json").read_text())
for backend in manifest["backends"]:
    with fixture(str(directory / "tag-batching.json")) as (server, url):
        settings = dict(manifest["client"], **case["client"], backend=backend, url=url)
        actual = json.loads(subprocess.check_output(
            [consumer, "finite-array", json.dumps(settings)], env=env))
        for row in actual:
            normalize(row, backend, settings["model"])
        assert actual == case["expected"], ("finite array order", backend, actual)
        assert int(command(server, "count")) == case["requests"], ("finite array batching", backend)
for phase in ("headers", "body", "refusal-body"):
    with fixture("--held", phase) as (server, url):
        with subprocess.Popen([consumer, "held", json.dumps({"backend": "systemone", "url": url})], stdin=subprocess.PIPE, env=env) as child:
            try:
                assert command(server, "arrived") == "arrived"
                child.stdin.write(b"c")
                child.stdin.flush()
                assert command(server, "closed") == "closed"
                assert child.wait(timeout=30) == 0
                assert command(server, "count") == "1"
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait()
print("installed Zig: canonical typed cases on both wires, ownership and held cancellation passed")

# Replace only the private installed runtime; the shared fixture bundle is immutable.
import os
import shutil
import tempfile

checkpoint = os.environ.get("THINKTHEN_ZIG_DEBUG_CHECKPOINT")
abi_fixtures = os.environ.get("THINKTHEN_ZIG_ABI_FIXTURES")
if abi_fixtures or checkpoint:
    base = Path(abi_fixtures) if abi_fixtures else Path(checkpoint) / "abi-fixtures"
    native_manifest = json.loads((base / "manifest.json").read_text())
    runtime = next(Path(consumer).parent.glob("libthinkthen_ffi.*"))
    request = {"question": {"function": "decide", "question": "Ready?"}, "records": []}

    def replace_runtime(source):
        # A loaded signed Mach-O must acquire a new inode when its bytes change.
        replacement = runtime.with_name(runtime.name + ".replacement")
        shutil.copy2(source, replacement)
        os.replace(replacement, runtime)

    with tempfile.TemporaryDirectory(prefix="zig-runtime-backup-") as temporary:
        backup = Path(temporary) / runtime.name
        shutil.copy2(runtime, backup)
        try:
            for case in native_manifest["cases"]:
                replace_runtime(case["library"])
                process = subprocess.run([consumer, "run", '{"backend":"stub"}', json.dumps(request)],
                                         text=True, capture_output=True, env=env)
                assert process.returncode == 0, (case["name"], process.returncode, process.stderr)
                actual = json.loads(process.stdout)
                expected = [] if case["accept"] else {"error": "usage.abi"}
                assert actual == expected, (case["name"], actual, expected)
        finally:
            replace_runtime(backup)
    print("installed Zig: all six shared ABI fixtures accept or refuse without entering incompatible engine operations")
