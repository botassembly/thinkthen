"""One deadline for a whole column operation.

The review of 2026-09-22 found `score` over a Polars column rebuilding
the caller's options row by row, so every row got a fresh budget and a
column of any length could outlive the deadline (group 4, "the Polars
score column restarts the deadline for every row").

This test serves the score shape from its own loopback server, one
request a row, each request sleeping `DELAY`. With one deadline for the
whole column, a budget under two delays must stop the column before
every row runs; with a per-row restart the column would answer all four
rows and never raise. No key, no paid call: the server is this file's
own, on an ephemeral loopback port.
"""

import json
import os
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

# Under two delays: the first two rows fit, the third is refused.
DELAY = 0.15
BUDGET = 0.25
ROWS = 4


class _Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        self.rfile.read(length)
        time.sleep(DELAY)
        body = json.dumps(
            {
                "model": "stand-in",
                "answers": {
                    "q1": {
                        "type": "score",
                        "probabilities": {"0": 0.1, "1": 0.2, "2": 0.7},
                    }
                },
            }
        ).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *args):  # keep the test's output clean
        pass


def test_a_score_column_honors_one_deadline():
    server = ThreadingHTTPServer(("127.0.0.1", 0), _Handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        # The engine is built on first use in this process, so the address
        # set here is the one it reads.
        os.environ["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{server.server_port}/v1"
        import polars as pl

        import thinkthen as tt

        question = tt.question(score="urgency", levels=["low", "medium", "high"])
        column = pl.Series("body", [f"ticket {place}" for place in range(ROWS)])
        started = time.monotonic()
        with pytest.raises(tt.DeadlineError):
            tt.score(question, column, deadline=BUDGET)
        elapsed = time.monotonic() - started
        assert elapsed < DELAY * ROWS, (
            f"the column ran for {elapsed:.2f} s: every row got its own budget"
        )
    finally:
        server.shutdown()
        server.server_close()
