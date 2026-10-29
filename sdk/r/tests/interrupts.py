"""Bound eager R cancellation through every installed public function."""
import http.server
import json
import signal
import subprocess
import threading
import time


def check(root, scratch, environment):
    # Feature: every eager function interrupts an active request within two seconds.
    manifest = json.loads((root / "conformance/typed/manifest.json").read_text())
    functions = {"decide", "choose", "tag", "score", "filter", "rank", "find", "recognize", "relate", "annotate"}
    held_cases = {}
    for name in manifest["cases"]:
        case = json.loads((root / "conformance/typed" / name).read_text())
        if case["requests"] and isinstance(case["expected"], list):
            held_cases.setdefault(case["request"]["question"]["function"], case)
    assert set(held_cases) == functions
    interrupt = scratch / "interrupt.R"
    interrupt.write_text('''source(commandArgs(TRUE)[[1]])
    case <- read_case(commandArgs(TRUE)[[2]])
    client <- tt_client(url=Sys.getenv("THINKTHEN_TEST_URL"), api_key="synthetic-local-test-key",
      retry=tt_retry(max_retries=0L, attempt_timeout_ms=120000L))
    interrupted <- tryCatch(invoke(case$request$records, case$request$question, client),
      thinkthen_cancelled_error=identity)
    stopifnot(inherits(interrupted, "thinkthen_cancelled_error"), interrupted$code == "local.cancelled")
    stopifnot(tt_decide("alpha", "Ready?", client=tt_client(backend="stub"))$status == "answered")
    cat("interrupted and still usable\\n")''')
    for function, case in held_cases.items():
        seen, closed = threading.Event(), threading.Event()
        requests = []
        failures = []

        class Held(http.server.BaseHTTPRequestHandler):
            def do_POST(self):
                self.rfile.read(int(self.headers["Content-Length"]))
                requests.append(self.path)
                seen.set()
                try:
                    self.connection.settimeout(120)
                    assert self.rfile.read(1) == b""
                except (OSError, AssertionError) as error:
                    failures.append(error)
                finally:
                    closed.set()

            def log_message(self, *_):
                pass

        with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Held) as server:
            server.daemon_threads = True
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            path = scratch / "held-function.json"
            path.write_text(json.dumps(case))
            child = subprocess.Popen(["Rscript", "--vanilla", interrupt,
                                      root / "sdk/r/tests/support.R", path],
                                     cwd=scratch, env=dict(environment,
                                     THINKTHEN_TEST_URL=f"http://127.0.0.1:{server.server_port}/v1"),
                                     stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            try:
                assert seen.wait(timeout=30), (function, "request never arrived")
                paid = len(requests)
                started = time.monotonic()
                child.send_signal(signal.SIGINT)
                if not closed.wait(timeout=2) or failures:
                    child.kill()
                    output, error = child.communicate(timeout=30)
                    raise AssertionError((function, "socket failed the two-second interrupt deadline",
                                          failures, "parent SIGINT", signal.getsignal(signal.SIGINT), output, error))
                elapsed = time.monotonic() - started
                output, error = child.communicate(timeout=120)
                assert child.returncode == 0 and "interrupted and still usable" in output, (function, output, error)
                assert len(requests) == paid, (function, "requests continued after cancellation", paid, len(requests))
                print(f"R {function}: socket closed in {elapsed:.3f}s; requests stayed at {paid}; typed cancellation and healthy reuse passed", flush=True)
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait(timeout=30)
                server.shutdown()
                thread.join(timeout=30)
                assert not thread.is_alive()
