"""Run the installed mechanical Zig bridge against canonical typed fixtures."""
import copy
import json
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

consumer, root_arg, loopback = sys.argv[1:]
root = Path(root_arg)
not_applicable = 0
sys.path.insert(0, str(root / "scripts"))
from test_control import read_line, request as fixture_request
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
            url = f"http://127.0.0.1:{int(read_line(server.stdout))}/v1"
            yield server, url
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait(timeout=30)

def command(server, value, *, timeout=30):
    return fixture_request(server, value, timeout=timeout)

def call(settings, request, cancelled=False, single=False):
    return json.loads(subprocess.check_output(
        [consumer, "single" if single else "cancelled" if cancelled else "run",
         json.dumps(settings), json.dumps(request)], env=env, timeout=120))

def normalize(row, backend, model):
    if row["function"] == "filter":
        kept = row.pop("kept")
        assert type(kept) is bool and kept == (row.get("answer") is True), row
    if "source" in row:
        source = row.pop("source")
        assert source["backend"] == backend, source
        assert source["model"] == model, source
        assert len(source["question_identity"]) == 64, source
        remaining = {key: value for key, value in source.items() if key not in ("backend", "model", "question_identity")}
        if remaining:
            row["source"] = remaining
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
directory = root / "conformance/typed"
manifest = json.loads((directory / "manifest.json").read_text())
seen = set()
single_seen = set()
for filename in manifest["cases"]:
    case = json.loads((directory / filename).read_text())
    if "zig" in case.get("not_applicable", {}):
        not_applicable += 1
        print("zig canonical " + filename + ": not applicable: " + case["not_applicable"]["zig"])
        continue
    seen.add(case["request"]["question"]["function"])
    for backend in manifest["backends"]:
        with fixture(str(directory / filename)) as (server, url):
            settings = dict(manifest["client"], **case.get("client", {}))
            settings.update(backend=backend, url=url)
            actual = call(settings, case["request"], case.get("cancelled", False))
            for row in actual if isinstance(actual, list) else []:
                normalize(row, backend, settings["model"])
            expected = copy.deepcopy(case["expected"])
            if isinstance(expected, list) and case["request"]["question"]["function"] in ("find", "recognize", "relate"):
                for row in expected:
                    record = case["request"]["records"][row["position"] - 1]
                    pointer = case["request"]["question"].get("evidence")
                    if pointer:
                        for part in pointer.split("/")[1:]:
                            key = part.replace("~1", "/").replace("~0", "~")
                            record = record[int(key)] if isinstance(record, list) else record[key]
                    assert isinstance(record, str), (filename, "offset fixture must select text")
                    byte_offsets(row.get("answer"), record)
                    byte_offsets(row.get("detail"), record)
            assert actual == expected, (filename, backend, actual, expected)
            assert int(command(server, "count")) == case["requests"], filename
        function = case["request"]["question"]["function"]
        if filename == "decide-precision.json":
            with fixture(str(directory / filename)) as (server, url):
                settings["url"] = url
                actual = call(settings, case["request"], single=True)
                for row in actual:
                    normalize(row, backend, settings["model"])
                assert actual == expected, ("single", filename, backend, actual, expected)
                assert int(command(server, "count")) == case["requests"], ("single", filename, backend)
                single_seen.add(function)
assert len(seen) == 10, seen
assert single_seen == {"decide"}, single_seen
print("Installed Zig: single-record entry preserves the shared precision case on both wires")
for case in json.loads((root / "construction/options.json").read_text()):
    if "zig" in case.get("not_applicable", {}):
        not_applicable += 1
        print("zig options " + case["name"] + ": not applicable: " + case["not_applicable"]["zig"])
        continue
    if case.get("expected_error") == "usage.settings":
        request = dict(case["request"], records=case["request"].get("records", []))
        assert call(case["client"], request) == {"error": case["expected_error"]}, case["name"]
    elif "expected_error" in case:
        # Typed Zig signatures exclude malformed question fields. Exercise their
        # public saved-JSON boundary so no serialized option case is skipped.
        value = {"entries": {"option": {"question": case["request"]["question"]}}}
        process = subprocess.run([consumer, "toolbox-invalid", json.dumps(value)],
                                 env=env, check=True, capture_output=True, text=True, timeout=30)
        assert process.stdout.splitlines()[0] == case["expected_error"], (case["name"], process.stdout)
    else:
        actual = call(case["client"], case["request"])
        assert actual == case["expected"], (case["name"], actual, case["expected"])
print("installed Zig: every shared construction option case passed")
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
        with subprocess.Popen([consumer, "held", json.dumps({"backend": "systemone", "url": url, "retry": {"max_retries": 0, "attempt_timeout_ms": 120000}})], stdin=subprocess.PIPE, env=env) as child:
            try:
                assert command(server, "arrived") == "arrived"
                child.stdin.write(b"c")
                child.stdin.flush()
                assert command(server, "closed", timeout=2) == "closed"
                assert child.wait(timeout=2) == 0
                assert command(server, "count") == "1"
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait(timeout=2)
print("installed Zig: canonical typed cases on both wires, ownership and held cancellation passed")

# Replace only the private installed runtime; the shared fixture bundle is immutable.
import os
import shutil
import tempfile

checkpoint = os.environ.get("THINKTHEN_ZIG_DEBUG_CHECKPOINT")
abi_fixtures = os.environ.get("THINKTHEN_ZIG_ABI_FIXTURES")
if not abi_fixtures and not checkpoint:
    raise RuntimeError("Zig gate requires its ABI fixture manifest")
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
print(f"Zig: {not_applicable} shared cases not applicable")
