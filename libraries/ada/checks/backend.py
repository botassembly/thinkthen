"""Offline counted backend for the public package check. Loopback only."""
import http.server
import json
import os
import pathlib
import re
import threading
import time


def one_record(request):
    """ADR 0111 quotes every record. Read a request quoting one record with that record as its state."""
    if request.get('state') != 'Each question quotes the text it asks about.':
        return request
    records, questions = set(), {}
    for name, question in request['questions'].items():
        text = str(question.get('instructions'))
        record, end = json.JSONDecoder().raw_decode(text, 12) if text.startswith('The text is ') else (None, 0)
        if not end or not text.startswith('. ', end):
            return request
        records.add(json.dumps(record))
        questions[name] = dict(question, instructions=text[end + 2:])
    return dict(request, state=json.loads(records.pop()), questions=questions) if len(records) == 1 else request

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.lock = threading.Lock()
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()

    def close(self):
        self.shutdown()
        self.worker.join()
        self.server_close()

class Handler(http.server.BaseHTTPRequestHandler):
    # Debt 020: ureq may reuse an HTTP/1.0 connection the server closes;
    # sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md
    def end_headers(self):
        self.send_header("Connection", "close")
        super().end_headers()

    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        wire = json.loads(body)
        request = one_record(wire)
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        with self.server.lock:
            self.server.arrivals.append(state)
            with (self.server.barrier / "request-bodies.jsonl").open("a") as evidence:
                evidence.write(json.dumps(wire, ensure_ascii=False, sort_keys=True) + "\n")
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-293":
            self.send_error(403)
            return
        packed = state_key == "Each question quotes the text it asks about."
        rows = {}
        if packed:
            for name, question in request["questions"].items():
                text = question["instructions"]
                match = re.fullmatch(r"The text is (.+)\. Is it\?", text, re.DOTALL)
                if not match:
                    self.send_error(422, "unexpected packed instruction")
                    return
                rows[name] = json.loads(match.group(1))
                if not isinstance(rows[name], str):
                    self.send_error(422, "non-string packed record")
                    return
        held = state_key if state_key.startswith("hold-") else next((row for row in rows.values() if row.startswith("hold-")), "")
        if held:
            (self.server.barrier / ("arrived-" + held)).touch()
            release = self.server.barrier / ("release-" + held)
            # Held up to 60 s, past every test's 30 s wait (ticket 0356).
            end = time.monotonic() + 60
            while not release.exists() and time.monotonic() < end:
                time.sleep(0.005)
            if not release.exists():
                self.send_error(500)
                return
        if state_key in ("status-401", "failure-one", "failure-two", "other-engine-error"):
            self.send_error(403 if state_key == "failure-two" else 401)
            return
        answers = {}
        for name, question in request["questions"].items():
            kind = question["type"]
            if kind == "noul":
                row_key = rows.get(name, state_key)
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6, "hold-reverse-2": 0.1}.get(row_key, 0.9)
                if packed and row_key == "first" and os.environ.get("TT_ADA_PLANT") == "wrong-packed-answer":
                    p = 0.1
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                answers[name] = {"type": kind, "probabilities": {key: 0.9 if i == 0 else 0.1 / (len(keys) - 1) for i, key in enumerate(keys)}}
        data = json.dumps({"model": request["model"], "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}}).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        try:
            self.wfile.write(data)
        except BrokenPipeError:
            pass

    def log_message(self, *_):
        pass
