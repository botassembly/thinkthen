"""Exercise the installed R package against shared rows and held local requests."""
import argparse
import contextlib
import http.server
import json
import os
import pathlib
import signal
import shutil
import socket
import subprocess
import threading

parser = argparse.ArgumentParser()
parser.add_argument("--root", type=pathlib.Path, required=True)
parser.add_argument("--library", type=pathlib.Path, required=True)
parser.add_argument("--scratch", type=pathlib.Path, required=True)
args = parser.parse_args()
# Copy prepared optional table packages before isolating the installed consumer.
# Discover their actual dependency closure instead of keeping a second list.
prepared_environment = dict(os.environ)
packages = subprocess.run(["Rscript", "--vanilla", "-e", '''
packages <- c("tibble", "dplyr", "data.table")
installed <- installed.packages()
dependencies <- tools::package_dependencies(packages, db = installed,
  which = c("Depends", "Imports", "LinkingTo"), recursive = TRUE)
packages <- unique(c(packages, unlist(dependencies, use.names = FALSE)))
packages <- packages[!packages %in% installed[!is.na(installed[, "Priority"]), "Package"]]
cat(jsonlite::toJSON(setNames(lapply(packages, find.package), packages), auto_unbox = TRUE))
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


def normalize_row(row):
    if "error" in row:
        row["error"].pop("message", None)
    if isinstance(row.get("answer"), dict):
        for outcome in row["answer"].values():
            if isinstance(outcome, dict) and isinstance(outcome.get("error"), dict):
                outcome["error"].pop("message", None)
    return row


def run_case(case, path):
    with backend(path) as (server, url):
        child = dict(environment, THINKTHEN_TEST_URL=url)
        actual = json.loads(rscript(args.root / "sdk/r/tests/run.R", path, env=child))
        expected = case["expected"]
        if isinstance(expected, dict):
            assert actual.get("error") == expected["error"], (path.name, actual, expected)
        elif case["request"]["function"] == "filter":
            selected = [record for record, row in zip(case["request"]["records"], expected) if row.get("status") == "answered" and row.get("answer") is True]
            assert actual == {"selected": selected}, (path.name, actual, selected)
        else:
            actual = [normalize_row(row) for row in actual["rows"]]
            assert actual == expected, (path.name, actual, expected)
        server.stdin.write("count\n")
        server.stdin.flush()
        assert int(server.stdout.readline()) == case["requests"], path.name


conformance_count = 0
interrupt_controls = []
unrepresentable_controls = []
nul_cases = {
    "record-nul-text.json", "usage-nul-choice-label.json", "usage-nul-kind.json",
    "usage-nul-question-name.json", "usage-nul-relation.json", "usage-nul-tag-label.json",
}
for path in sorted((args.root / "conformance").glob("*.json")):
    case = json.loads(path.read_text())
    if case.get("cancelled"):
        # R has ordinary interrupts, not a public pre-cancelled handle. The held
        # request below exercises its public cancellation boundary instead.
        assert path.name == "cancel-before-call.json", ("unmapped R cancellation control", path.name)
        interrupt_controls.append(path.name)
        continue
    if path.name in nul_cases:
        # R cannot construct strings containing NUL. surface.R proves this
        # public host boundary once; these calls cannot reach the R API.
        assert "\0" in json.dumps(case["request"], ensure_ascii=False).replace("\\u0000", "\0"), path.name
        unrepresentable_controls.append(path.name)
        continue
    run_case(case, path)
    conformance_count += 1
for index, case in enumerate(json.loads((args.root / "construction/stub.json").read_text())):
    path = args.scratch / f"stub-{index}.json"
    path.write_text(json.dumps(case))
    run_case(case, path)
# Refused threshold options have no R parameter; .tt_dots rejects them before
# admission. These paired calls check the domains that the public API accepts.
for case in json.loads((args.root / "construction/options.json").read_text()):
    if "equivalent_request" not in case:
        path = args.scratch / "options.json"
        path.write_text(json.dumps(case))
        rscript(args.root / "sdk/r/tests/run.R", path, argument_error=True)
        continue
    outputs = []
    for key in ("request", "equivalent_request"):
        path = args.scratch / "options.json"
        path.write_text(json.dumps({"client": case["client"], "request": case[key]}))
        outputs.append(json.loads(rscript(args.root / "sdk/r/tests/run.R", path)))
    actual, equivalent = outputs
    assert len(actual.get("rows", [])) == len(case["request"]["records"]), (case["name"], actual)
    assert actual == equivalent, (case["name"], actual, equivalent)
print(f"R shared conformance: {conformance_count} cases passed; "
      f"{len(interrupt_controls)} precancel control mapped to held R interrupt; "
      f"{len(unrepresentable_controls)} NUL cases mapped to R string construction refusal; "
      "all ten stub functions passed", flush=True)
print(rscript(args.root / "sdk/r/tests/surface.R", args.root / "construction/stub.json"), end="", flush=True)
print(rscript(args.root / "sdk/r/tests/frames.R", args.root / "construction/stub.json"), end="", flush=True)

# Keep a bound, non-listening port for the entire unreachable-backend call.
with socket.socket() as reservation:
    reservation.bind(("127.0.0.1", 0))
    child = dict(environment, THINKTHEN_TEST_URL=f"http://127.0.0.1:{reservation.getsockname()[1]}/v1")
    case = {"request": {"function": "filter", "question": {"type": "inline", "value": "Ready?"}, "records": ["alpha"]}}
    path = args.scratch / "unreachable.json"
    path.write_text(json.dumps(case))
    actual = json.loads(rscript(args.root / "sdk/r/tests/run.R", path, env=child))
    assert actual == {"error": "backend.transport", "family": "thinkthen_backend_error"}, actual

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
client <- tt_client(url=Sys.getenv("THINKTHEN_TEST_URL"), key_source="THINKTHEN_API_KEY")
interrupted <- tryCatch({tt_decide("held", "Ready?", client=client); FALSE}, interrupt=function(e) TRUE)
stopifnot(interrupted)
stopifnot(tt_decide("alpha", "Ready?", client=tt_client(adapter="stub"))$status == "answered")
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
