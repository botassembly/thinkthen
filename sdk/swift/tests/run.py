"""Drive the installed Swift consumer with the authoritative typed manifest."""
from contextlib import contextmanager
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from threading import Event, Thread
from urllib.request import Request, urlopen
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

consumer, root, loopback = sys.argv[1:]
root = Path(root)
not_applicable = 0
sys.path.insert(0, str(root / "scripts"))
from test_control import read_line, request as fixture_request
env = {key: os.environ[key] for key in ("PATH", "HOME", "TMPDIR", "LLVM_PROFILE_FILE", "SystemRoot", "THINKTHEN_SWIFT_AUDIT_LIBRARY") if key in os.environ}
subprocess.run([sys.executable, str(root / "sdk/c/tests/toolbox.py"), consumer, str(root)], env=env, check=True)
network_env = dict(env, THINKTHEN_API_KEY="loopback-test-key", OPENAI_API_KEY="loopback-test-key")


def command(server, text, *, timeout=30):
    return fixture_request(server, text, timeout=timeout)


@contextmanager
def fixture(*args):
    server = subprocess.Popen([loopback, *args], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env)
    try:
        url = f"http://127.0.0.1:{int(read_line(server.stdout))}/v1"
        yield server, url
        assert command(server, "shutdown") == ""
        assert server.wait(timeout=30) == 0
    finally:
        if server.poll() is None:
            server.kill()
            server.wait(timeout=30)


def normalize(row, backend, model):
    row = dict(row)
    if row["function"] == "filter":
        kept = row.pop("kept")
        assert type(kept) is bool and kept == (row.get("answer") is True), row
    source = row.pop("source", None)
    if source is not None:
        assert source["backend"] == backend and source["model"] == model, source
        identity = source["question_identity"]
        assert len(identity) == 64 and all(c in "0123456789abcdef" for c in identity), source
        remaining = {key: value for key, value in source.items() if key not in ("backend", "model", "question_identity")}
        if remaining:
            row["source"] = remaining
    if row.get("error"):
        row["error"].pop("message", None)
    if row["function"] == "annotate" and row.get("answer"):
        row["answer"] = {name: normalize(child, backend, model) for name, child in row["answer"].items()}
    return row


subprocess.run([str(Path(consumer).with_name("Readme"))], env=env, check=True, capture_output=True, timeout=120)
subprocess.run([sys.executable, str(root / "scripts/release/swift.py"), consumer,
                str(root / "construction/stub.json")], env=env, check=True)
manifest_path = root / "conformance/typed/manifest.json"
manifest = json.loads(manifest_path.read_text())
single_entry = "decide-precision.json"
assert single_entry in manifest["cases"]
single_cases = {"decide": (single_entry, json.loads((manifest_path.parent / single_entry).read_text()))}
with tempfile.TemporaryDirectory() as folder:
    request_path = Path(folder) / "case.json"
    for entry in manifest["cases"]:
        path = manifest_path.parent / entry
        case = json.loads(path.read_text())
        if "swift" in case.get("not_applicable", {}):
            not_applicable += 1
            print("swift canonical " + entry + ": not applicable: " + case["not_applicable"]["swift"])
            continue
        for backend in manifest["backends"]:
            settings = {**manifest["client"], **case.get("client", {}), "backend": backend}
            request_path.write_text(json.dumps({"client": settings, "request": case["request"], "cancelled": case.get("cancelled", False)}))
            with fixture(str(path)) as (server, url):
                result = subprocess.run([consumer, str(request_path), url], env=network_env, check=True, capture_output=True, text=True, timeout=120)
                actual = json.loads(result.stdout)
                if isinstance(actual, dict):
                    assert not actual.get("rows"), (entry, actual)
                    actual = {"error": actual["error"]}
                else:
                    actual = [normalize(row, backend, settings["model"]) for row in actual]
                assert actual == case["expected"], (entry, backend, actual, case["expected"])
                assert int(command(server, "count")) == case["requests"], (entry, backend)
    for function, (entry, case) in single_cases.items():
        for backend in manifest["backends"]:
            settings = {**manifest["client"], **case.get("client", {}), "backend": backend}
            request_path.write_text(json.dumps({"client": settings, "request": case["request"]}))
            with fixture(str(manifest_path.parent / entry)) as (server, url):
                result = subprocess.run([consumer, "single", str(request_path), url], env=network_env,
                                        check=True, capture_output=True, text=True, timeout=120)
                actual = [normalize(row, backend, settings["model"]) for row in json.loads(result.stdout)]
                assert actual == case["expected"], ("single", function, backend, actual, case["expected"])
                assert int(command(server, "count")) == case["requests"], ("single", function, backend)
print("Installed Swift: single-record entry preserves the shared precision case on both wires")

for index, case in enumerate(json.loads((root / "construction/options.json").read_text())):
    if "swift" in case.get("not_applicable", {}):
        not_applicable += 1
        print("swift options " + case["name"] + ": not applicable: " + case["not_applicable"]["swift"])
        continue
    if "expected_error" in case and case["expected_error"] != "usage.settings":
        saved = {"entries": {"option": {"question": case["request"]["question"]}}}
        result = subprocess.run([consumer, "toolbox-invalid", json.dumps(saved)],
                                env=env, check=True, capture_output=True, text=True, timeout=30)
        assert result.stdout.splitlines()[0] == case["expected_error"], (case["name"], result.stdout)
    else:
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "options.json"
            path.write_text(json.dumps(case))
            result = subprocess.run([consumer, str(path)], env=env, check=True,
                                    capture_output=True, text=True, timeout=30)
            actual = json.loads(result.stdout)
        expected = {"error": case["expected_error"]} if "expected_error" in case else case["expected"]
        assert actual == expected, (case["name"], actual, expected)
print("Installed Swift: every shared construction option case passed")

# Finite array admission retains batching around an invalid structured record.
case_path = manifest_path.parent / "tag-batching.json"
case = json.loads(case_path.read_text())
case["request"]["records"].insert(1, {"id": {"invalid": True}})
with tempfile.TemporaryDirectory() as folder:
    request_path = Path(folder) / "finite.json"
    for backend in manifest["backends"]:
        settings = {**manifest["client"], **case["client"], "backend": backend}
        request_path.write_text(json.dumps({"client": settings, "request": case["request"], "cancelled": case.get("cancelled", False)}))
        with fixture(str(case_path)) as (server, url):
            result = subprocess.run([consumer, str(request_path), url], env=network_env,
                                    check=True, capture_output=True, text=True, timeout=120)
            rows = [normalize(row, backend, settings["model"]) for row in json.loads(result.stdout)]
            assert len(rows) == 4 and rows[1]["position"] == 2, rows
            assert rows[1]["status"] == "failed" and rows[1]["error"]["code"] == "record.invalid", rows
            good = rows[:1] + rows[2:]
            for index, row in enumerate(good, 1):
                row["position"] = index
            assert good == case["expected"], good
            assert int(command(server, "count")) == case["requests"]

for stage in ("headers", "body", "refusal-body"):
    with fixture("--held", stage) as (server, url):
        child = subprocess.Popen([consumer, "cancel", url], stdin=subprocess.PIPE, env=network_env)
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
subprocess.run([sys.executable, str(root / "scripts/stress-adapters.py"), "--surface", "swift", *(["--full"] if os.environ.get("THINKTHEN_STRESS_FULL") else []), "--", sys.executable, str(root / "sdk/c/tests/stress.py"), "swift", consumer], env=network_env | {"THINKTHEN_LOOPBACK": loopback}, check=True)
# Hold the backend reply until the caller's second input is suspended. This
# forces output readiness to wake the driver independently of caller input.
with fixture(str(manifest_path.parent / "decide-precision.json")) as (_, upstream):
    input_entered = Event()

    class GatedReply(BaseHTTPRequestHandler):
        def do_POST(self):
            body = self.rfile.read(int(self.headers["Content-Length"]))
            assert input_entered.wait(120), "caller never requested its next input"
            request = Request(upstream.split("/v1")[0] + self.path, data=body,
                              headers={"Content-Type": "application/json"})
            with urlopen(request, timeout=30) as response:
                payload = response.read()
            self.send_response(200)
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def log_message(self, *_):
            pass

    with HTTPServer(("127.0.0.1", 0), GatedReply) as gate:
        serving = Thread(target=gate.serve_forever)
        serving.start()
        child = subprocess.Popen([consumer, "output-input", f"http://127.0.0.1:{gate.server_port}/v1"],
                                 stdout=subprocess.PIPE, text=True, env=network_env)
        try:
            assert read_line(child.stdout) == "input\n"
            input_entered.set()
            assert child.wait(timeout=120) == 0
        finally:
            input_entered.set()
            if child.poll() is None:
                child.kill()
                child.wait(timeout=2)
            gate.shutdown()
            serving.join(timeout=30)
            assert not serving.is_alive()

subprocess.run([sys.executable, str(root / "sdk/swift/tests/thread-start-audit.py"), consumer, "lifetime"], env=env, check=True, timeout=120)
result = subprocess.run([consumer, "surface"], env=env, capture_output=True, text=True, timeout=120)
assert result.returncode == 0, (result.stdout, result.stderr)
assert result.stdout == "", result.stdout

# The library has no path override. Replace the installed resource in each fresh
# process, so refused fixtures abort if layout access precedes the handshake.
fixture_path = os.environ["THINKTHEN_NATIVE_FIXTURES"]
cases = json.loads((Path(fixture_path) / "manifest.json").read_text())["cases"]
filename = "libthinkthen_ffi.dylib" if sys.platform == "darwin" else "thinkthen_ffi.dll" if sys.platform == "win32" else "libthinkthen_ffi.so"
resources = list(Path(consumer).parent.rglob(filename))
assert len(resources) == 1, resources
library = resources[0]


def replace_library(source):
    # Give signed code a new inode before the next process loads it.
    with tempfile.TemporaryDirectory(dir=library.parent) as folder:
        staged = Path(folder) / library.name
        shutil.copy2(source, staged)
        os.replace(staged, library)


with tempfile.TemporaryDirectory() as folder:
    original = Path(folder) / library.name
    shutil.copy2(library, original)
    try:
        for case in cases:
            replace_library(case["library"])
            result = subprocess.run([consumer, "abi"], env=env, capture_output=True, text=True, check=True)
            if case["accept"]:
                assert result.stdout.strip() == "accepted", (case["name"], result.stdout)
            elif case.get("missing"):
                assert case["missing"] in result.stdout, (case["name"], result.stdout)
            else:
                assert result.stdout.strip() == "version", (case["name"], result.stdout)
    finally:
        replace_library(original)
print("Installed Swift consumer: typed manifest through both wires, exact IDs, cancellation, owned rows and ABI checks pass")
print(f"Swift: {not_applicable} shared cases not applicable")
