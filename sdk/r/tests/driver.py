"""Exercise the installed R package against shared rows and held local requests."""
import argparse
import contextlib
import http.server
import hashlib
import json
import os
import pathlib
import signal
import shutil
import socket
import subprocess
import threading
import sys

parser = argparse.ArgumentParser()
parser.add_argument("--root", type=pathlib.Path, required=True)
parser.add_argument("--library", type=pathlib.Path, required=True)
parser.add_argument("--scratch", type=pathlib.Path, required=True)
parser.add_argument("--module-abi-only", action="store_true")
parser.add_argument("--native-module", type=pathlib.Path)
args = parser.parse_args()
families = {entry["name"]: entry["family"] for entry in
            json.loads((args.root / "schemas/contract.json").read_text())["codes"]}
if args.module_abi_only and args.native_module is None:
    parser.error("--module-abi-only requires --native-module")
# Copy prepared optional table packages before isolating the installed consumer.
# Discover their actual dependency closure instead of keeping a second list.
prepared_environment = dict(os.environ)
packages = subprocess.run(["Rscript", "--vanilla", "-e", '''
packages <- c("jsonlite", "tibble", "rlang", "cli", "dplyr", "data.table")
installed <- installed.packages()
dependencies <- tools::package_dependencies(packages, db = installed,
  which = c("Depends", "Imports", "LinkingTo"), recursive = TRUE)
packages <- unique(c(packages, unlist(dependencies, use.names = FALSE)))
packages <- packages[!packages %in% installed[!is.na(installed[, "Priority"]), "Package"]]
writeLines(jsonlite::toJSON(setNames(lapply(packages, find.package), packages), auto_unbox = TRUE), useBytes = TRUE)
'''], env=prepared_environment, capture_output=True, text=True)
if packages.returncode:
    diagnostic = "\n".join(packages.stderr.splitlines()[-20:])[-4000:]
    raise RuntimeError("Prepared R dependency discovery failed:\n" + diagnostic)
for name, source in json.loads(packages.stdout).items():
    if not (args.library / name).exists():
        shutil.copytree(source, args.library / name)

environment = {name: os.environ[name] for name in ("PATH", "R_HOME", "THINKTHEN_LOOPBACK", "LLVM_PROFILE_FILE") if name in os.environ}
environment.update(HOME=str(args.scratch / "home"), R_LIBS_USER=str(args.library), R_LIBS_SITE="", THINKTHEN_API_KEY="synthetic-local-test-key", OPENAI_API_KEY="synthetic-local-test-key")


def rscript(script, *values, env=None, argument_error=False):
    result = subprocess.run(["Rscript", "--vanilla", str(script), *map(str, values)], env=env or environment, cwd=args.scratch, capture_output=True, text=True)
    if argument_error:
        assert result.returncode != 0 and "unused arguments" in result.stderr, result.stderr
        return
    assert result.returncode == 0, (str(script), values, result.stdout + result.stderr)
    return result.stdout


def guide():
    folder = args.scratch / "guide"
    helper = args.root / "scripts/surface-example"
    subprocess.run([sys.executable, str(helper), "prepare", "r", str(folder)], check=True)
    subprocess.run([sys.executable, str(helper), "check", "r", "Rscript", "--vanilla", "example.R"],
                   env=environment, cwd=folder, check=True)


def module_abi():
    subprocess.run([sys.executable, str(args.root / "scripts/module-abi-fixtures"),
                    "r", str(args.library / "thinkthen")],
                   env=environment, cwd=args.scratch, check=True)


if args.module_abi_only:
    # Reuse the module from the installed-package gate; install only R's facade.
    module = args.native_module.resolve(strict=True)
    digest = hashlib.sha256(module.read_bytes()).hexdigest()
    stage = args.scratch / "package" / "thinkthen"
    stage.mkdir(parents=True)
    for name in ("DESCRIPTION", "NAMESPACE", "LICENSE"):
        shutil.copy2(args.root / "sdk/r" / name, stage / name)
    shutil.copytree(args.root / "sdk/r/R", stage / "R")
    (stage / "inst/libs").mkdir(parents=True)
    shutil.copy2(module, stage / "inst/libs/thinkthen.so")
    subprocess.run(["R", "CMD", "INSTALL", "--library=" + str(args.library), str(stage)],
                   env=environment, cwd=args.scratch, check=True)
    installed = args.library / "thinkthen/libs/thinkthen.so"
    assert hashlib.sha256(installed.read_bytes()).hexdigest() == digest
    assert hashlib.sha256(module.read_bytes()).hexdigest() == digest
    print("R native module reused unchanged:", module, digest, flush=True)
    module_abi()
    subprocess.run(["Rscript", "--vanilla", "-e",
                    'library(thinkthen); stopifnot(tt_decide("text", "Ready?", '
                    'client=tt_client(backend="stub"))$status == "answered")'],
                   env=environment, cwd=args.scratch, check=True)
    sys.exit(0)


@contextlib.contextmanager
def backend(path):
    server = subprocess.Popen([environment["THINKTHEN_LOOPBACK"], str(path)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=environment)
    try:
        url = f"http://127.0.0.1:{server.stdout.readline().strip()}/v1"
        yield server, url
    finally:
        server.stdin.write("shutdown\n")
        server.stdin.flush()
        _, error = server.communicate()
        assert server.returncode == 0, error


def expected_r(value):
    if isinstance(value, list):
        return [expected_r(item) for item in value]
    if not isinstance(value, dict):
        return value
    result = {key: expected_r(item) for key, item in value.items()}
    if "start" in result and "end" in result:
        result["start"] += 1
    if type(result.get("id")) is int:
        result["id"] = {"integer": str(result["id"])}
    return result


def assert_fields(actual, expected, context):
    if isinstance(expected, dict):
        assert isinstance(actual, dict), (context, actual, expected)
        for key, value in expected.items():
            assert key in actual, (context, "missing", key, actual)
            assert_fields(actual[key], value, context)
    elif isinstance(expected, list):
        assert isinstance(actual, list) and len(actual) == len(expected), (context, actual, expected)
        for got, want in zip(actual, expected):
            assert_fields(got, want, context)
    else:
        assert type(actual) is type(expected) or isinstance(actual, (int, float)) and isinstance(expected, (int, float)), (context, actual, expected)
        assert actual == expected, (context, actual, expected)


manifest = json.loads((args.root / "conformance/typed/manifest.json").read_text())
coverage = {name: set() for name in manifest["backends"]}
count_failures = []
for name in manifest["cases"]:
    original = json.loads((args.root / "conformance/typed" / name).read_text())
    for provider in manifest["backends"]:
        case = dict(original)
        case["client"] = dict(manifest["client"], **original.get("client", {}), backend=provider)
        case["integer_ids"] = {
            str(index): str(record["id"])
            for index, record in enumerate(case["request"]["records"], 1)
            if isinstance(record, dict) and type(record.get("id")) is int
            and -(2**63) <= record["id"] <= 2**64 - 1
        }
        case["invalid_integer_ids"] = [
            index for index, record in enumerate(case["request"]["records"], 1)
            if isinstance(record, dict) and type(record.get("id")) is int
            and not -(2**63) <= record["id"] <= 2**64 - 1
        ]
        path = args.scratch / f"{provider}-{name}"
        path.write_text(json.dumps(case))
        with backend(path) as (server, url):
            actual = json.loads(rscript(args.root / "sdk/r/tests/run.R", path,
                                        env=dict(environment, THINKTHEN_TEST_URL=url)))
            if isinstance(case["expected"], dict):
                assert actual.get("error") == case["expected"]["error"], (name, provider, actual)
                assert actual["family"] == f"thinkthen_{families[actual['error']]}_error", actual
                assert not actual.get("rows"), (name, provider, actual)
                server.stdin.write("count\n")
                server.stdin.flush()
                assert int(server.stdout.readline()) == case["requests"], (name, provider)
                continue
            assert "rows" in actual, (name, provider, actual)
            rows = actual["rows"]
            expected = expected_r(case["expected"])
            assert len(rows) == len(expected), (name, rows, expected)
            assert_fields(rows, expected, (name, provider))
            for row in rows:
                assert row["source"]["backend"] == provider
                assert row["source"]["model"] == case["client"]["model"]
                assert len(row["source"]["question_identity"]) == 64
                assert "p" not in row
                if row["status"] == "failed":
                    assert "answer" not in row and "probability" not in row
                if row["status"] == "unsure":
                    assert "answer" not in row
            server.stdin.write("count\n")
            server.stdin.flush()
            actual_count = int(server.stdout.readline())
            if actual_count != case["requests"]:
                failure = (name, provider, actual_count, case["requests"])
                count_failures.append(failure)
                print("R request count mismatch:", failure, flush=True)
        coverage[provider].add(case["request"]["question"]["function"])
functions = {"decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"}
assert all(seen == functions for seen in coverage.values()), coverage
print("R canonical cases passed through both backend wires", flush=True)
# Regenerated construction cases retain the installed stub's exact public rows.
for index, case in enumerate(json.loads((args.root / "construction/stub.json").read_text())):
    path = args.scratch / f"stub-{index}.json"
    path.write_text(json.dumps(case))
    with backend(path) as (server, url):
        actual = json.loads(rscript(args.root / "sdk/r/tests/run.R", path,
                                    env=dict(environment, THINKTHEN_TEST_URL=url)))
        assert "rows" in actual, (index, actual)
        assert len(actual["rows"]) == len(case["expected"]), index
        assert_fields(actual["rows"], expected_r(case["expected"]), ("stub", index))
        server.stdin.write("count\n")
        server.stdin.flush()
        assert int(server.stdout.readline()) == 0, ("stub made an HTTP call", index)
print("R regenerated stub construction cases passed", flush=True)
# Conversion failures must finish before any backend request, on either route.
context_case = args.scratch / "conversion-contexts.json"
context_case.write_text(json.dumps({"replies": [], "requests": 0}))
with backend(context_case) as (server, url):
    for script in ("context.R", "typed-ids.R"):
        print(rscript(args.root / "sdk/r/tests" / script,
                      env=dict(environment, THINKTHEN_TEST_URL=url)), end="", flush=True)
    server.stdin.write("count\n")
    server.stdin.flush()
    assert int(server.stdout.readline()) == 0, "host conversion contexts made a backend call"
print(rscript(args.root / "sdk/r/tests/toolbox-json.R", args.root / "conformance/toolbox"), end="", flush=True)
for script in ("contract.R", "typed-surface.R", "typed-frames.R", "surface.R", "frames.R", "frame-serialization.R"):
    print(rscript(args.root / "sdk/r/tests" / script), end="", flush=True)

print(rscript(args.root / "sdk/r/tests/release-cases.R", args.root / "construction/stub.json"),
      end="", flush=True)

print(rscript(args.root / "sdk/r/tests/readme.R", args.root / "sdk/r/README.md"), end="", flush=True)

# Keep a bound, non-listening port for the entire unreachable-backend call.
with socket.socket() as reservation:
    reservation.bind(("127.0.0.1", 0))
    child = dict(environment, THINKTHEN_TEST_URL=f"http://127.0.0.1:{reservation.getsockname()[1]}/v1")
    case = {"client": {"retry": {"max_retries": 0}}, "request": {"question": {"function": "filter", "question": "Ready?"}, "records": ["alpha"]}, "expected": [{"status": "failed"}]}
    path = args.scratch / "unreachable.json"
    path.write_text(json.dumps(case))
    actual = json.loads(rscript(args.root / "sdk/r/tests/run.R", path, env=child))
    assert actual["rows"][0]["status"] == "failed" and actual["rows"][0]["error"]["code"] == "backend.transport", actual

seen = threading.Event()
closed = threading.Event()


class Held(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.rfile.read(int(self.headers["Content-Length"]))
        seen.set()
        # Cancellation closes the active connection. Read the event, not a clock.
        assert self.rfile.read(1) == b""
        closed.set()

    def log_message(self, *_):
        pass


with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Held) as server:
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    expression = '''library(thinkthen)
client <- tt_client(url=Sys.getenv("THINKTHEN_TEST_URL"), api_key="synthetic-local-test-key")
interrupted <- tryCatch(tt_decide("held", "Ready?", client=client), thinkthen_cancelled_error=identity)
stopifnot(inherits(interrupted, "thinkthen_cancelled_error"), interrupted$code == "local.cancelled")
stopifnot(tt_decide("alpha", "Ready?", client=tt_client(backend="stub"))$status == "answered")
cat("interrupted and still usable\\n")'''
    child = subprocess.Popen(["Rscript", "--vanilla", "-e", expression], cwd=args.scratch, env=dict(environment, THINKTHEN_TEST_URL=f"http://127.0.0.1:{server.server_port}/v1"), stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    seen.wait()
    child.send_signal(signal.SIGINT)
    output, error = child.communicate()
    assert child.returncode == 0 and "interrupted and still usable" in output, (output, error)
    closed.wait()
    server.shutdown()
    thread.join()
print("R unreachable code and held-request interrupt passed", flush=True)
subprocess.run([sys.executable, str(args.root / "scripts/stress-adapters.py"), "--surface", "r", "--sequential", *(["--full"] if os.environ.get("THINKTHEN_STRESS_FULL") else []), "--", sys.executable, str(args.root / "sdk/c/tests/stress.py"), "r", "Rscript", "--vanilla", str(args.root / "sdk/r/tests/stress.R")], env=environment, cwd=args.scratch, check=True)
guide()
module_abi()
assert not count_failures, ("shared push-input request counts", count_failures)
