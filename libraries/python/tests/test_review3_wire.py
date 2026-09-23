"""The third review's wire finding, counted through a loopback stub.

From ``sdlc/issues/2026-09-22-surfaces-branch-third-review-the-unheld-fixes.md``:

- item 14, a pre-cancelled token still spent a full batch: the token is
  now read before any request is built, so a call that starts cancelled
  must leave the server's count at zero and raise ``Cancelled``.

The server answers the wire shape a decide question asks for. No key, no
paid call: the server is this file's own, on an ephemeral loopback port.
Run in its own stage, without ``ENGINE_NULL``:
``.venv/bin/python -m pytest tests/test_review3_wire.py -q``
"""

import json
import os
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

os.environ.pop("ENGINE_NULL", None)
os.environ["ENGINE_WIDTH"] = "1"

DELAY = 0.05
SERVED = {"requests": 0}


def _reply_for(body: bytes) -> bytes:
    plan = json.loads(body.decode())
    question = next(iter(plan.get("questions", {}).values()), {})
    answer = (
        {"type": "noul", "noul": 0.9}
        if question.get("type") == "noul"
        else {"type": "other"}
    )
    return json.dumps({"model": "stand-in", "answers": {"q1": answer}}).encode()


class _Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        SERVED["requests"] += 1
        time.sleep(DELAY)
        payload = _reply_for(body)
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)

    def log_message(self, *args):  # keep the test's output clean
        pass


_server = ThreadingHTTPServer(("127.0.0.1", 0), _Handler)
threading.Thread(target=_server.serve_forever, daemon=True).start()
# The engine is built on first use in this process, so the address set
# here is the one it reads.
os.environ["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{_server.server_port}/v1"

import thinkthen as tt  # noqa: E402  (after the environment is set)

TEXTS = [f"record {at}" for at in range(4)]


def test_a_precancelled_bulk_token_sends_nothing():
    token = tt.CancelToken()
    token.cancel()
    before = SERVED["requests"]
    with pytest.raises(tt.Cancelled):
        tt.decide_many("Is this a complaint?", TEXTS, token=token)
    assert SERVED["requests"] == before, (
        f"a cancelled token must not spend: {SERVED['requests'] - before} requests"
    )


def test_a_precancelled_single_token_sends_nothing():
    token = tt.CancelToken()
    token.cancel()
    before = SERVED["requests"]
    with pytest.raises(tt.Cancelled):
        tt.decide("Is this a complaint?", TEXTS[0], token=token)
    assert SERVED["requests"] == before


def test_a_precancelled_column_token_sends_nothing():
    pytest.importorskip("polars")
    pl = pytest.importorskip("polars")
    token = tt.CancelToken()
    token.cancel()
    before = SERVED["requests"]
    with pytest.raises(tt.Cancelled):
        tt.decide("Is this a complaint?", pl.Series("body", TEXTS), token=token)
    assert SERVED["requests"] == before


def test_a_live_token_still_runs_and_stops_between_rows():
    """The guard must not overfire: a fresh token runs, and a controller
    thread's cancel lands between rows (ENGINE_WIDTH=1), not never."""
    import threading

    token = tt.CancelToken()
    worker = {"raised": None}

    def run():
        try:
            tt.decide_many("Is this a complaint?", TEXTS * 2, token=token)
            worker["raised"] = "ran to the end"
        except tt.Cancelled:
            worker["raised"] = "cancelled"

    thread = threading.Thread(target=run)
    thread.start()
    time.sleep(DELAY * 1.5)
    token.cancel()
    thread.join(timeout=10)
    assert worker["raised"] == "cancelled"
    spent = SERVED["requests"]
    assert spent >= 1, "a live call must still send"
    assert spent < len(TEXTS) * 2, "the cancel must land before the end"
