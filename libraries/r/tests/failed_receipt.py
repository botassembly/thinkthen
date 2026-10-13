"""An installed R failed call retains its prefix and settles the same failure."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import json
import subprocess
import sys
import tempfile
import threading

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env

arrivals = []


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        arrivals.append(self.rfile.read(int(self.headers['Content-Length'])))
        body = json.dumps({'model': 'fixed', 'answers': {'q1': {'type': 'noul', 'noul': 0.9}}}).encode()
        self.send_response(200 if len(arrivals) == 1 else 503)
        self.send_header('Content-Length', str(len(body)))
        self.send_header("Connection", "close")
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *_args):
        pass


server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
worker = threading.Thread(target=server.serve_forever)
worker.start()
try:
    with tempfile.TemporaryDirectory(prefix='thinkthen-r-failed-receipt-') as home:
        code = '''library(thinkthen)
tt_engine(cache = FALSE, max_retries = 0L, throttle = 1L, batch = 1L, model = "fixed")
receipt <- tt_completion()
input <- tempfile()
writeLines(c("first", "second"), input)
failure <- tryCatch(tt_decide("Prefix?", tt_files(input), completion = receipt), thinkthen_error = identity)
settled <- tt_completion_read(receipt)
stopifnot(inherits(failure, "thinkthen_backend"), length(failure$completed) == 1L, failure$completed[[1]]$input == "first")
stopifnot(settled$state == "terminal", settled$kind == failure$kind, settled$facts$requests_sent == failure$facts$requests_sent, settled$facts$requests_sent == 2)
cat("r: failed prefix and completion agree\\n")
'''
        env = child_env(('R_LIBS', 'LANG', 'LC_ALL'), home=home,
                        THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/v1',
                        THINKTHEN_API_KEY='fake-r-failed-receipt')
        result = subprocess.run(['Rscript', '--vanilla', '-e', code], env=env,
                                text=True, capture_output=True, timeout=15)
        assert result.returncode == 0, result.stdout + result.stderr
        assert len(arrivals) == 2, len(arrivals)
        print(result.stdout.strip())
finally:
    server.shutdown()
    worker.join()
    server.server_close()
