"""Future installed-package tests. Shared fixture and ABI artifacts are required."""
import argparse
import contextlib
import http.server
import json
import os
import pathlib
import signal
import subprocess
import threading

parser = argparse.ArgumentParser()
parser.add_argument("--root", type=pathlib.Path, required=True)
parser.add_argument("--library", type=pathlib.Path, required=True)
parser.add_argument("--scratch", type=pathlib.Path, required=True)
parser.add_argument("--id-values", type=pathlib.Path, required=True)
parser.add_argument("--abi-major-library", type=pathlib.Path, required=True)
parser.add_argument("--abi-minor-library", type=pathlib.Path, required=True)
args = parser.parse_args()
tests = args.root / "sdk/r/tests"
# Dependencies must already be installed by package preparation. Keep the
# operating system variables required by its R runtime, and isolate user state.
environment = {key: value for key, value in os.environ.items() if key in {
    "PATH", "SystemRoot", "SYSTEMROOT", "WINDIR", "COMSPEC", "PATHEXT",
    "R_HOME", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH", "THINKTHEN_LOOPBACK",
    "LLVM_PROFILE_FILE", "TMP", "TEMP", "TMPDIR",
}}
home = args.scratch / "typed-home"
home.mkdir(parents=True, exist_ok=True)
environment.update(HOME=str(home), USERPROFILE=str(home),
                   XDG_CONFIG_HOME=str(home / "config"), XDG_CACHE_HOME=str(home / "cache"),
                   R_LIBS_USER=str(args.library), R_LIBS_SITE="",
                   THINKTHEN_API_KEY="synthetic-systemone-key",
                   OPENAI_API_KEY="synthetic-openai-key")


def run(script, *values, env=None):
    result = subprocess.run(["Rscript", "--vanilla", str(script), *map(str, values)],
                            env=env or environment, cwd=args.scratch,
                            capture_output=True, text=True)
    assert result.returncode == 0, (script.name, result.stdout, result.stderr)
    return result.stdout


@contextlib.contextmanager
def backend(path):
    server = subprocess.Popen([environment["THINKTHEN_LOOPBACK"], str(path)],
                              stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE, text=True, env=environment)
    try:
        port = server.stdout.readline().strip()
        assert port.isdecimal(), ("loopback readiness", port)
        yield server, f"http://127.0.0.1:{port}/v1"
    finally:
        if server.poll() is None:
            server.stdin.write("shutdown\n")
            server.stdin.flush()
        _, error = server.communicate()
        assert server.returncode == 0, error


def count(server):
    server.stdin.write("count\n")
    server.stdin.flush()
    return int(server.stdout.readline())


# Feature: every function returns the same typed R rows from both wire formats.
functions = {"decide", "choose", "tag", "score", "filter", "rank", "find",
             "annotate", "recognize", "relate"}
coverage = {name: set() for name in ("systemone", "openai")}
signatures = {name: {} for name in coverage}
precision = False
for path in sorted((args.root / "conformance").glob("*.json")):
    case = json.loads(path.read_text())
    # Native R cannot construct NUL strings and has no pre-cancel token. Their
    # host boundaries are exercised below rather than silently skipped.
    if case.get("cancelled"):
        assert path.name == "cancel-before-call.json"
        continue
    if "\\u0000" in json.dumps(case["request"]):
        continue
    q = case["request"]["question"]
    assert "function" in q, (path.name, "requires regenerated flat-question fixtures")
    backend_name = case.get("client", {}).get("backend", "systemone")
    assert backend_name in coverage or backend_name == "stub", path.name
    numeric_ids = {
        str(index): str(record["id"])
        for index, record in enumerate(case["request"].get("records", []), 1)
        if isinstance(record, dict) and type(record.get("id")) is int
        and abs(record["id"]) > 2**53
    }
    with backend(path) as (server, url):
        output = json.loads(run(tests / "typed-case.R", path,
                                env=dict(environment, THINKTHEN_TEST_URL=url,
                                         THINKTHEN_TEST_NUMERIC_IDS=json.dumps(numeric_ids),
                                         THINKTHEN_TEST_ID_VALUES=str(args.id_values))))
        assert count(server) == case["requests"], path.name
    if "signature" in output and backend_name in coverage:
        coverage[backend_name].add(q["function"])
        # Every successful fixture must use the same prototype for this
        # function. Detail may be absent or present by status, so compare its
        # actual values against the shared expectations in the R case driver.
        shape = {key: value for key, value in output["signature"].items() if key != "detail"}
        previous = signatures[backend_name].setdefault(q["function"], shape)
        assert previous == shape, path.name
    for row in case.get("expected", []):
        if isinstance(row, dict) and isinstance(row.get("probability"), float):
            probability = row["probability"]
            precision |= round(probability, 2) != probability
for name in coverage:
    assert coverage[name] == functions, (name, "missing shared wire fixtures", functions - coverage[name])
assert signatures["systemone"] == signatures["openai"], "backend-dependent R types"
assert precision, "shared fixtures need a probability that exposes rounding"

# Feature: NA, null and invalid IDs send no paid request on either backend.
for backend_name in coverage:
    fixture = args.scratch / f"typed-missing-{backend_name}.json"
    fixture.write_text(json.dumps({"replies": [], "requests": 0}))
    expression = '''library(thinkthen)
client <- tt_client(backend=Sys.getenv("TEST_BACKEND"),
                    url=Sys.getenv("THINKTHEN_TEST_URL"), api_key="synthetic-local-key")
for (x in list(c(NA_character_, NA_character_), list(NULL),
               list(list(id=TRUE, body="text")), list(list(id=1.5, body="text")))) {
  rows <- tt_decide(x, "Ready?", client=client, .progress=FALSE)
  stopifnot(nrow(rows) == length(x), all(rows$status == "failed"),
            all(as.character(rows$error_code) %in% c("record.missing", "record.invalid")))
}
tt_close(client)'''
    script = args.scratch / "typed-no-send.R"
    script.write_text(expression)
    with backend(fixture) as (server, url):
        run(script, env=dict(environment, TEST_BACKEND=backend_name, THINKTHEN_TEST_URL=url))
        assert count(server) == 0, backend_name

assert args.id_values.is_file(), "specialist-approved lossless R ID input fixture is required"
run(tests / "typed-ids.R", args.id_values)
for script in ("contract.R", "typed-surface.R", "typed-frames.R"):
    run(tests / script)

# Feature: incompatible major or insufficient minor fails at namespace load.
for library in (args.abi_major_library, args.abi_minor_library):
    assert (library / "thinkthen").is_dir(), ("missing incompatible R artifact", library)
    run(tests / "typed-abi.R", env=dict(environment, R_LIBS_USER=str(library)))

# Feature: R rejects embedded NUL before it can enter a library API.
nul = args.scratch / "typed-nul.R"
nul.write_text('error <- tryCatch(rawToChar(as.raw(c(97L, 0L, 98L))), error=identity)\n'
               'stopifnot(inherits(error, "error"))\n')
run(nul)

# Feature: an interrupt cancels the socket and raises the package condition.
# Readiness and closure are socket events, never elapsed-time guesses.
seen = threading.Event()
ready_or_exited = threading.Event()
closed = threading.Event()


class Held(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        self.rfile.read(int(self.headers["Content-Length"]))
        seen.set()
        ready_or_exited.set()
        if self.rfile.read(1) == b"":
            closed.set()

    def log_message(self, *_):
        pass


with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Held) as server:
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    expression = '''library(thinkthen)
client <- tt_client(url=Sys.getenv("THINKTHEN_TEST_URL"), api_key="synthetic-local-key")
caught <- tryCatch(tt_decide("held", "Ready?", client=client),
                   thinkthen_cancelled_error=identity, interrupt=function(e) stop("raw interrupt escaped"))
stopifnot(inherits(caught, "thinkthen_cancelled_error"),
          as.character(caught$code) == "local.cancelled")
tt_close(client)
client <- tt_client(backend="stub")
stopifnot(tt_decide("text", "Ready?", client=client)$status == "answered")
tt_close(client)
cat("cancelled and usable\\n")'''
    child = subprocess.Popen(["Rscript", "--vanilla", "-e", expression],
                             env=dict(environment, THINKTHEN_TEST_URL=f"http://127.0.0.1:{server.server_port}/v1"),
                             cwd=args.scratch, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    def note_exit():
        child.wait()
        ready_or_exited.set()

    watcher = threading.Thread(target=note_exit, daemon=True)
    watcher.start()
    try:
        ready_or_exited.wait()
        assert seen.is_set(), "R exited before opening the held request"
        child.send_signal(signal.SIGINT)
        output, error = child.communicate()
        assert child.returncode == 0 and "cancelled and usable" in output, (output, error)
        closed.wait()
    finally:
        if child.poll() is None:
            child.kill()
            child.communicate()
        server.shutdown()
        thread.join()
        watcher.join()
print("R typed surface and both backend wire contracts passed")
