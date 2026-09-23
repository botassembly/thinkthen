"""The fourth review's signal findings, counted through a loopback stub.

From ``sdlc/issues/2026-09-23-surfaces-branch-fourth-review.md`` item 14:
a single ``decide`` retried the interrupted request and raised late, and
a Polars score column never heard a signal inside itself. The door now
polls the interpreter's signals beside every call and cancels the call's
own token the moment one arrives, so the raise comes back promptly and no
new request starts.

The server sleeps half a second a request, so a prompt raise lands well
before the in-flight send finishes — the old door could not raise until
the call returned. No key, no paid call: the server is this file's own,
on an ephemeral loopback port. Run in its own stage, without
``ENGINE_NULL``:
``.venv/bin/python -m pytest tests/test_review4_signals.py -q``
"""

import json
import os
import pathlib
import signal
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

import pytest

os.environ.pop("ENGINE_NULL", None)
os.environ["ENGINE_WIDTH"] = "1"

DELAY = 0.5

SERVED = {"requests": 0}


def _reply_for(body: bytes) -> bytes:
    """The wire answer the request's own question asks for."""
    plan = json.loads(body.decode())
    question = next(iter(plan.get("questions", {}).values()), {})
    kind = question.get("type")
    if kind == "noul":
        answer = {"type": "noul", "noul": 0.9}
    elif kind == "score":
        levels = question.get("criteria") or ["0"]
        count = len(levels)
        share = round(1.0 / count, 6)
        probabilities = {str(place): share for place in range(count)}
        probabilities[str(count - 1)] = round(1.0 - share * (count - 1), 6)
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

import polars as pl  # noqa: E402  (after the environment is set)
import thinkthen as tt  # noqa: E402

QUESTION = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
LEVELS = tt.question(score="how formal is the note?", levels=["low", "high"])


def _raise_on_usr1():
    def on_signal(signum, frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGUSR1, on_signal)


def _signal_at(delay: float) -> threading.Thread:
    import os as _os

    def send():
        time.sleep(delay)
        _os.kill(_os.getpid(), signal.SIGUSR1)

    thread = threading.Thread(target=send, daemon=True)
    thread.start()
    return thread


def _settle() -> int:
    """Let any in-flight send finish and read the served count."""
    time.sleep(DELAY * 1.5)
    return SERVED["requests"]


@pytest.fixture(autouse=True)
def _arm():
    _raise_on_usr1()


def test_the_counting_stub_counts_a_served_call():
    before = SERVED["requests"]
    assert tt.decide(QUESTION, "a fresh text the counter has not seen") is True
    assert SERVED["requests"] == before + 1


def test_a_signal_during_a_single_decide_raises_promptly_and_once():
    before = SERVED["requests"]
    _signal_at(DELAY * 0.2)
    start = time.monotonic()
    with pytest.raises(KeyboardInterrupt):
        tt.decide(QUESTION, "another text the cache has not seen")
    elapsed = time.monotonic() - start
    # The old door held the raise until the send returned; half the send
    # is the line between prompt and late.
    assert elapsed < DELAY * 0.6, f"the raise took {elapsed:.2f}s of a {DELAY}s send"
    served = _settle()
    assert served == before + 1, (
        f"exactly one request: {served - before} served, and a signal "
        "must not start a second"
    )


def test_a_signal_inside_a_score_column_stops_before_the_next_row():
    rows = ["first note", "second note", "third note"]
    column = pl.Series("body", rows)
    before = SERVED["requests"]
    _signal_at(DELAY * 0.3)
    with pytest.raises(KeyboardInterrupt):
        tt.score(LEVELS, column)
    served = _settle()
    # The first row's send is in flight when the signal lands; the raise
    # is prompt, the in-flight send finishes, and no later row starts.
    assert served <= before + 2, (
        f"{served - before} requests served; the signal must stop the "
        "column before the last row"
    )
