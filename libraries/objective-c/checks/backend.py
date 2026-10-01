"""Offline counted backend for the public package check. Loopback only."""
import http.server
import os
import json
import pathlib
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

PACKED_STATE = "Each question quotes the text it asks about."

def packed_rows(request):
    if request["state"] != PACKED_STATE:
        raise ValueError("not a packed request")
    rows = {}
    for name, question in request["questions"].items():
        instruction = question["instructions"]
        prefix, suffix = "The text is ", ". Is it?"
        if not instruction.startswith(prefix) or not instruction.endswith(suffix):
            raise ValueError((name, instruction))
        encoded = instruction[len(prefix):-len(suffix)]
        row = json.loads(encoded)
        if not isinstance(row, str) or instruction != prefix + json.dumps(row, ensure_ascii=False, separators=(",", ":")) + suffix:
            raise ValueError((name, instruction))
        rows[name] = row
    return rows

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.requests = []
        self.attempts = 0
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
        packed = state_key == PACKED_STATE
        rows = packed_rows(request) if packed else {}
        with self.server.lock:
            self.server.arrivals.append(state)
            self.server.requests.append(wire)
            self.server.attempts += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-295":
            self.send_error(403)
            return
        held = [x for x in ([state_key] if state_key.startswith("hold-") else rows.values()) if isinstance(x,str) and x.startswith("hold-")]
        # A packed cancelled call is one request. Releasing its first row releases the whole request;
        # the second row has no separately arriving wire request. Reverse ordering waits for both.
        if "hold-bulk-1" in held: held = ["hold-bulk-1"]
        for row in held: (self.server.barrier / ("arrived-" + row)).touch()
        for row in held:
            release = self.server.barrier / ("release-" + row)
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
                row = rows.get(name,state_key)
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6, "hold-reverse-2": 0.1}.get(row, 0.9)
                if row == 'json-decide' and os.environ.get('TT_PLANT_JSON_WRONG') == '1': p = 0.1
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
