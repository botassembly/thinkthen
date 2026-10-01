"""Focused packed try-details budget cases on the existing DuckDB child harness."""

from __future__ import annotations

import json
import sys
import threading
from http.server import BaseHTTPRequestHandler
from pathlib import Path

from harness import case, expect, rows, run

# Ticket 0386: the shared server class skips the reverse lookup that stalls a macOS runner.
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "sdlc" / "scripts"))
from loopback_server import LoopbackServer  # noqa: E402

PACKED_PAIR_BODIES = {
    b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"}}}',
    b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"gamma\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"delta\\". Is it a refund?"}}}',
}
SPLIT_ORIGINAL = b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"},"q3":{"type":"noul","instructions":"The text is \\"gamma\\". Is it a refund?"},"q4":{"type":"noul","instructions":"The text is \\"delta\\". Is it a refund?"}}}'
SAFE_SPENT = {"kind": "usage", "message":
              "check the row's question and arguments, or raise the process request total when it is spent",
              "retryable": False}


def answer(questions: dict) -> bytes:
    return json.dumps({"model": "jev-latest", "answers":
                       {key: {"type": "noul", "noul": 0.7} for key in questions}}).encode()


class PackedReplies:
    """One captured loopback listener shared by existing and new packed cases."""

    def __init__(self, missing_second=True, fail_first_pair=False, responder=None):
        self.bodies = []
        owner = self

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                body = self.rfile.read(int(self.headers["Content-Length"]))
                owner.bodies.append(body)
                questions = json.loads(body)["questions"]
                if responder is not None:
                    status, headers, reply = responder(body, questions)
                elif fail_first_pair and b'alpha' in body:
                    status, headers, reply = 503, {}, b""
                elif len(questions) == 3 and missing_second:
                    status, headers = 200, {"Content-Type": "application/json"}
                    reply = json.dumps({"model": "jev-latest", "answers":
                                        {"q1": {"type": "noul", "noul": 0.9},
                                         "q3": {"type": "noul", "noul": 0.8}}}).encode()
                else:
                    status, headers, reply = 200, {"Content-Type": "application/json"}, answer(questions)
                self.send_response(status)
                # An HTTP/1.0 server closes after each reply. ureq-proto 0.6.4 pools the
                # connection unless the reply says so, and a reused one fails under load.
                # Issue: sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md
                self.send_header("Connection", "close")
                for name, value in headers.items():
                    self.send_header(name, value)
                self.send_header("Content-Length", str(len(reply)))
                self.end_headers()
                self.wfile.write(reply)

            def log_message(self, _format, *_args):
                pass

        self.server = LoopbackServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()

    @property
    def base(self):
        return f"http://127.0.0.1:{self.server.server_port}/v1"

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.server.shutdown()
        self.thread.join(timeout=10)
        self.server.server_close()


def values(result: dict) -> list:
    return [json.loads(row[0]) if row[0] is not None else None for row in rows(result)]


@case
def b13c_try_details_split_denials():
    """A refused original spends one attempt; denied left/right never resend."""
    query = ("SELECT thinkthen_try_details('Is it a refund?', x) FROM "
             "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta'),(5,NULL)) t(i,x) ORDER BY i")

    def split(_body, questions):
        if len(questions) == 4:
            return 413, {}, b"too large"
        return 200, {"Content-Type": "application/json"}, answer(questions)

    for total, expected_bodies, statuses in [
        (1, [SPLIT_ORIGINAL], ["failed"] * 4),
        (2, [SPLIT_ORIGINAL, next(body for body in PACKED_PAIR_BODIES if b'alpha' in body)],
         ["answered", "answered", "failed", "failed"]),
    ]:
        with PackedReplies(responder=split) as backend:
            got = run(["SET threads = 1", "SET thinkthen_batch = 'max'",
                       "SET thinkthen_max_retries = 0",
                       f"SET thinkthen_max_requests_total = {total}", query], backend.base)
            result = values(got[4])
            expect([value["status"] for value in result[:4]], statuses, f"total {total} SQL slots")
            expect(result[4], None, f"total {total} SQL NULL")
            for place, value in enumerate(result[:4]):
                if statuses[place] == "failed":
                    expect(value, {"status": "failed", "error": SAFE_SPENT}, f"total {total} safe slot {place}")
            expect(backend.bodies, expected_bodies, f"total {total} actual ordered request bytes")


@case
def b13c_try_details_total_zero_sends_nothing():
    with PackedReplies() as backend:
        got = run(["SET thinkthen_max_requests_total = 0",
                   "SELECT thinkthen_try_details('Is it a refund?', x) FROM "
                   "(VALUES (1,'alpha'),(2,NULL)) t(i,x) ORDER BY i"], backend.base)
        expect(values(got[1]), [{"status": "failed", "error": SAFE_SPENT}, None],
               "before-first-send safe value and SQL NULL")
        expect(backend.bodies, [], "zero actual sends")


@case
def b13c_try_details_denied_retry_keeps_later_answer():
    """A later packed answer spends the last send before a held 503 retries."""
    later_answered = threading.Event()

    def respond(body, questions):
        if b'alpha' in body:
            if not later_answered.wait(60):
                return 500, {}, b"later packed request did not complete"
            return 503, {"retry-after-ms": "0"}, b""
        reply = answer(questions)
        later_answered.set()
        return 200, {"Content-Type": "application/json"}, reply

    query = ("SELECT thinkthen_try_details('Is it a refund?', x) FROM "
             "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta')) t(i,x) ORDER BY i")
    with PackedReplies(responder=respond) as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '2'",
                   "SET thinkthen_max_retries = 1", "SET thinkthen_max_requests_total = 2", query],
                  backend.base)
        result = values(got[4])
        expect([value["status"] for value in result],
               ["failed", "failed", "answered", "answered"], "denied retry and later answer positions")
        for place in [0, 1]:
            expect(result[place], {"status": "failed", "error": SAFE_SPENT}, f"denied retry slot {place}")
        expect(set(backend.bodies), PACKED_PAIR_BODIES, "both exact packed bodies, no retry send")
        expect(len(backend.bodies), 2, "two actual sends")
