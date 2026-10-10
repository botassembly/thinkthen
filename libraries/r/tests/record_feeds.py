"""Installed R producers preserve native bounds, locations and failed prefixes."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import subprocess
import sys
import tempfile
import threading

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import child_env

sends = []
held = threading.Event()
released = threading.Event()
class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        sends.append(self.rfile.read(int(self.headers["Content-Length"])))
        if b"Held?" in sends[-1]:
            held.set()
            assert released.wait(15)
        body = b'{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}'
        self.send_response(200)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
    def log_message(self, *_args):
        pass

server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
worker = threading.Thread(target=server.serve_forever)
worker.start()
try:
    with tempfile.TemporaryDirectory(prefix="thinkthen-r-feeds-") as home:
        code = '''library(thinkthen)
tt_engine(cache=FALSE, max_retries=0L, model="fixed", batch=1L)
at <- 0L
feed <- tt_feed(function() { at <<- at + 1L; if (at <= 2L) tt_record(paste0("row", at), list(file="records",first_line=at*2L,last_line=at*2L)) else NULL })
result <- tt_decide("Feed?", feed)
stopifnot(length(result$results) == 2L, result$facts$requests_sent == 2,
          inherits(result, "thinkthen_Call"), at == 3L,
          result$results[[1]]$source$first_line==2, result$results[[2]]$source$first_line==4)
at <- 0L
bad <- tt_feed(function() { at <<- at + 1L; "never" })
fault <- tryCatch(tt_choose(list(choose="Invalid?",options=list()), bad), thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_usage"), at == 0L)
fault <- tryCatch(tt_decide("Receipt?",bad,completion=tt_completion()),thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_usage"),at==0L)
fault <- tryCatch(tt_decide("Expired?",bad,deadline_ms=0),thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_deadline"),fault$facts$requests_sent==0)
at <- 0L
partial <- tt_feed(function() { at <<- at + 1L; if(at==1L) tt_record("prefix") else tt_reader_failure("io") })
fault <- tryCatch(tt_decide("Partial?", partial), thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_local"),length(fault$completed)==1L,
          fault$facts$requests_sent==1, inherits(fault$facts,"thinkthen_Facts"))
batch <- tt_batch("decide","Push?",tt_feed())
stopifnot(batch$push(tt_record("manual")) == "accepted")
batch$finish()
stopifnot(batch$push(tt_record("closed")) == "closed", inherits(batch$next_result(),"thinkthen_DecideResult"),
          is.null(batch$next_result()), batch$facts()$requests_sent==1)
batch$close()
batch <- tt_batch("decide","Cancel?",tt_feed(function() stop("must not read")))
batch$cancel()
fault <- tryCatch(batch$next_result(), thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_cancelled"),fault$facts$requests_sent==0)
closed <- FALSE
batch <- tt_batch("decide","Drop?",tt_feed(close=function() closed <<- TRUE))
rm(batch); gc(); stopifnot(closed)
cat("r: bounded record feeds passed\\n")
'''
        env = child_env(("R_LIBS", "LANG", "LC_ALL"), home=home,
                        THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/v1",
                        THINKTHEN_API_KEY="fake-r-feeds")
        result = subprocess.run(["Rscript", "--vanilla", "-e", code], env=env,
                                text=True, capture_output=True, timeout=20)
        print(result.stdout, end="")
        print(result.stderr, end="", file=sys.stderr)
        assert result.returncode == 0, result.returncode
        assert len(sends) == 4, len(sends)
        held_code = '''library(thinkthen)
tt_engine(cache=FALSE,max_retries=0L,model="fixed",batch=1L)
at <- 0L
batch <- tt_batch("decide","Held?",tt_feed(function() {at <<- at + 1L; tt_record(paste0("row",at))}))
stopifnot(is.null(batch$poll()),at==1L)
cat("started\\n");flush(stdout());readLines(file("stdin"),n=1L)
stopifnot(is.null(batch$poll()),is.null(batch$poll()),is.null(batch$poll()),
          at==3L,batch$push(tt_record("fourth"))=="full")
batch$cancel()
stopifnot(batch$push(tt_record("fourth"))=="closed",at==3L)
cat("cancelled-before-release\\n");flush(stdout())
fault <- tryCatch({repeat { if(is.null(batch$next_result())) break }},thinkthen_error=identity)
stopifnot(inherits(fault,"thinkthen_cancelled"),fault$facts$requests_sent==1)
batch$close()
'''
        child = subprocess.Popen(["Rscript", "--vanilla", "-e", held_code], env=env,
                                 text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE)
        try:
            assert child.stdout.readline() == "started\n"
            assert held.wait(10), "held provider was never called"
            child.stdin.write("continue\n")
            child.stdin.flush()
            # R readLines prints its input; skip that ordinary R output.
            line = child.stdout.readline()
            if line.startswith("[1]"):
                line = child.stdout.readline()
            assert line == "cancelled-before-release\n", line
            assert not released.is_set()
            released.set()
            child.stdin.close()
            child.stdin = None
            output, error = child.communicate(timeout=10)
            assert child.returncode == 0, (output, error)
            assert len(sends) == 5, len(sends)
            print("r: Full/Closed and cancellation before provider release passed")
        finally:
            released.set()
            if child.poll() is None:
                child.kill()
                child.wait()
finally:
    server.shutdown()
    worker.join()
    server.server_close()
