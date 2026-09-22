"""The second review's wire findings, counted through a loopback stub.

From ``sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md``:

- item 11, a refused pandas frame or pyarrow table ran the whole paid
  batch before the refusal: this file's own loopback server counts the
  requests, and a refused input must leave the count untouched;
- the cancel leftovers, a token for Python: a controller thread's
  ``CancelToken`` stops a bulk call and is read between the rows of a
  score column.

The server answers the two wire shapes the calls here ask for — a `noul`
for a decide question and a `score` distribution for a score question —
and sleeps ``DELAY`` a request so a cancel has a window. No key, no paid
call: the server is this file's own, on an ephemeral loopback port. Run
in its own stage, without ``ENGINE_NULL``:
``.venv/bin/python -m pytest tests/test_review2_wire.py -q``
"""

import json
import os
import pathlib
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

os.environ.pop("ENGINE_NULL", None)
os.environ["ENGINE_WIDTH"] = "1"

DELAY = 0.2

SET = str(pathlib.Path(__file__).with_name("fixture") / "form.json")
SERVED = {"requests": 0}


def _reply_for(body: bytes) -> bytes:
    """The wire answer the request's own question asks for."""
    plan = json.loads(body.decode())
    question = next(iter(plan.get("questions", {}).values()), {})
    kind = question.get("type")
    if kind == "noul":
        answer = {"type": "noul", "noul": 0.9}
    elif kind == "choice":
        options = list((question.get("criteria") or {}).keys()) or ["one"]
        share = round(1.0 / len(options), 6)
        probabilities = {name: share for name in options}
        probabilities[options[-1]] = round(1.0 - share * (len(options) - 1), 6)
        answer = {"type": "choice", "probabilities": probabilities}
    elif kind == "score":
        levels = question.get("criteria") or ["0"]
        count = len(levels)
        share = round(1.0 / count, 6)
        probabilities = {str(place): share for place in range(count)}
        probabilities[str(count - 1)] = round(
            1.0 - share * (count - 1), 6
        )
        answer = {"type": "score", "probabilities": probabilities}
    else:
        answer = {"type": "other"}
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

import pandas as pd  # noqa: E402  (after the environment is set)
import polars as pl  # noqa: E402
import pyarrow as pa  # noqa: E402
import thinkthen as tt  # noqa: E402

QUESTION = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)


def test_a_refused_pandas_frame_makes_no_request():
    frame = pd.DataFrame({"body": ["a", "b", "c"]})
    before = SERVED["requests"]
    with pytest.raises(tt.UsageError, match="Pass the column instead"):
        tt.annotate(SET, frame, on="body")
    assert SERVED["requests"] == before


def test_a_refused_pyarrow_table_makes_no_request():
    table = pa.table({"body": ["a", "b", "c"]})
    before = SERVED["requests"]
    with pytest.raises(tt.UsageError, match="Pass the column instead"):
        tt.annotate(SET, table, on="body")
    assert SERVED["requests"] == before


def test_the_counting_stub_counts_a_served_call():
    # The positive control: the counter moves when a request is really
    # served, so the zero-request assertions above can fail.
    before = SERVED["requests"]
    assert tt.decide(QUESTION, "a fresh text the cache has not seen") is True
    assert SERVED["requests"] == before + 1


def test_a_cancel_token_stops_a_bulk_call():
    token = tt.CancelToken()
    records = [f"a token cancel record {place}" for place in range(30)]
    timer = threading.Timer(0.4, token.cancel)
    timer.start()
    started = time.monotonic()
    try:
        with pytest.raises(tt.Cancelled) as caught:
            tt.decide_many(QUESTION, records, token=token)
    finally:
        timer.cancel()
    elapsed = time.monotonic() - started
    # The whole batch would take 30 x DELAY; a token heard at the engine's
    # ticks stops it well inside that.
    assert elapsed < 2.0, f"the call ran {elapsed:.2f} s"
    assert caught.value.retryable is False
    # The cancel is this package's own error too, not only a
    # KeyboardInterrupt.
    assert isinstance(caught.value, tt.ThinkThenError)


def test_a_cancel_token_stops_a_score_column_between_rows():
    token = tt.CancelToken()
    ask = tt.question(score="urgency", levels=["low", "medium", "high"])
    column = pl.Series("body", [f"a token score row {place}" for place in range(12)])
    before = SERVED["requests"]
    timer = threading.Timer(0.4, token.cancel)
    timer.start()
    started = time.monotonic()
    try:
        with pytest.raises(tt.Cancelled):
            tt.score(ask, column, token=token)
    finally:
        timer.cancel()
    elapsed = time.monotonic() - started
    served = SERVED["requests"] - before
    assert served < 12, f"the column served all {served} rows"
    assert elapsed < 2.0, f"the column ran {elapsed:.2f} s"


def test_a_polars_frame_still_runs_through_the_wire():
    frame = pl.DataFrame({"body": ["a served frame row", "another served row"]})
    before = SERVED["requests"]
    out = tt.annotate(SET, frame, on="body")
    assert type(out) is pl.DataFrame
    assert SERVED["requests"] > before


def teardown_module(module):
    _server.shutdown()
    _server.server_close()
