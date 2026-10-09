"""Installed named calls and native sessions send the R wrapper identity."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import subprocess
import sys
import tempfile
import threading

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import child_env

agents = []


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        self.rfile.read(int(self.headers["Content-Length"]))
        agents.append(self.headers["User-Agent"])
        body = b'{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}'
        self.send_response(200)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Connection", "close")
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *_args):
        pass


server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
worker = threading.Thread(target=server.serve_forever)
worker.start()
try:
    with tempfile.TemporaryDirectory(prefix="thinkthen-r-surface-") as home:
        code = '''library(thinkthen)
tt_engine(cache=FALSE, max_retries=0L, model="fixed")
stopifnot(tt_decide("Eager?", "one")$facts$requests_sent == 1)
batch <- tt_batch("decide", "Batch?", "two")
stopifnot(inherits(batch$next_result(), "thinkthen_DecideResult"))
stopifnot(is.null(batch$next_result()), batch$facts()$requests_sent == 1)
batch$close()
cat(as.character(packageVersion("thinkthen")))
'''
        env = child_env(("R_LIBS", "LANG", "LC_ALL"), home=home,
                        THINKTHEN_BASE_URL=f"http://127.0.0.1:{server.server_port}/v1",
                        THINKTHEN_API_KEY="fake-r-surface")
        result = subprocess.run(["Rscript", "--vanilla", "-e", code], env=env,
                                text=True, capture_output=True, timeout=15, check=True)
        assert agents == [f"thinkthen/{result.stdout} (r)"] * 2, agents
        print("r: eager and batch send R User-Agent (2 requests)")
finally:
    server.shutdown()
    worker.join()
    server.server_close()
