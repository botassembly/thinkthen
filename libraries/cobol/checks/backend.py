"""Counted loopback-only synthetic C-door backend, with file barriers for held work."""
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
        self.completions = []
        self.lock = threading.Lock()
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()

    def close(self):
        self.shutdown()
        self.worker.join()
        self.server_close()

class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        wire = json.loads(body)
        request = one_record(wire)
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        with self.server.lock:
            self.server.arrivals.append(state)
            index = len(self.server.arrivals)
            (self.server.barrier / f"request-{index:03d}.json").write_text(json.dumps(wire, ensure_ascii=False, indent=2) + "\n")
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-fixture-key":
            self.send_error(403)
            return
        packed = state == "Each question quotes the text it asks about."
        rows = {}
        if packed:
            for name, question in request["questions"].items():
                match = re.match(r'^The text is ("(?:\\.|[^"\\])*")\. ', question["instructions"])
                if not match:
                    self.send_error(400)
                    return
                rows[name] = json.loads(match.group(1))
        held = next((v for v in rows.values() if isinstance(v, str) and v.startswith("hold-")), None)
        if isinstance(state, str) and state.startswith("hold-") or held:
            hold_name = held or state
            (self.server.barrier / ("arrived-" + hold_name)).touch()
            release = self.server.barrier / ("release-" + hold_name)
            end = time.monotonic() + 8
            while not release.exists() and time.monotonic() < end:
                time.sleep(.005)
            if not release.exists():
                self.send_error(500)
                return
        if state in ("first", "second", "third"):
            time.sleep({"first": .09, "second": .05, "third": .005}[state])
        if state in ("status-401", "failure-one", "failure-two"):
            self.send_error(403 if state == "failure-two" else 401)
            return
        answers = {}
        for name, question in request["questions"].items():
            kind = question["type"]
            if kind == "noul":
                row_key = rows.get(name, state_key)
                if os.environ.get("TT_PLANT_PACKED_SWAP") == "1" and packed:
                    row_key = {"first": "second", "second": "first"}.get(row_key, row_key)
                p = {"no": .1, "unsure": .5, "first": .9, "second": .1, "third": .6}.get(row_key, .9)
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                answers[name] = {"type": kind, "probabilities": {key: .9 if i == 0 else .1 / (len(keys) - 1) for i, key in enumerate(keys)}}
        data = json.dumps({"model": request["model"], "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}}).encode()
        with self.server.lock:
            self.server.completions.append(state)
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
