"""Independently counted synthetic backend for the Swift C-door prototype. Loopback only."""
import http.server
import json
import pathlib
import threading
import time
import socket


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

PACKED_STATE = 'Each question quotes the text it asks about.'

def packed_rows(request):
    if request['state'] != PACKED_STATE:
        return {}
    rows = {}
    for name, question in request['questions'].items():
        instruction = question['instructions']
        prefix = 'The text is '
        if not instruction.startswith(prefix):
            raise ValueError('packed question has no quoted record')
        row, end = json.JSONDecoder().raw_decode(instruction[len(prefix):])
        if not isinstance(row, str) or instruction[len(prefix) + end:] != '. Is it?':
            raise ValueError('packed question is not a quoted Is it? record')
        rows[name] = row
    return rows

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.completions = []
        self.attempts = 0
        self.connections = 0
        self.bulk_barrier = threading.Barrier(3)
        self.bulk_done = {key: threading.Event() for key in ('first', 'second', 'third')}
        self.bulk_seen = {key: 0 for key in ('first', 'second', 'third')}
        self.bulk_completion = []
        self.lock = threading.Lock()
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()

    def finish_request(self, request, client_address):
        with self.lock:
            self.connections += 1
        return super().finish_request(request, client_address)

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
        (self.server.barrier / 'requests.jsonl').open('a').write(json.dumps(wire,ensure_ascii=False) + '\n')
        try:
            packed = packed_rows(request)
        except ValueError:
            self.send_error(400)
            return
        with self.server.lock:
            self.server.arrivals.append(state)
            self.server.attempts += 1
            bulk_first = state_key in ('first', 'second', 'third') and self.server.bulk_seen[state_key] == 0
            if state_key in self.server.bulk_seen:
                self.server.bulk_seen[state_key] += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-294":
            self.send_error(403)
            return
        held_rows = [state] if isinstance(state, str) and state.startswith('hold-') else [row for row in packed.values() if row.startswith('hold-')]
        for held_row in held_rows:
            (self.server.barrier / ('arrived-' + held_row)).touch()
        for held_row in held_rows:
            release = self.server.barrier / ('release-' + held_row)
            # Held up to 60 s, past every test's 30 s wait (ticket 0356).
            end = time.monotonic() + 60
            while not release.exists() and time.monotonic() < end:
                time.sleep(0.005)
            if not release.exists():
                self.send_error(500)
                return
        if bulk_first:
            self.server.bulk_barrier.wait(timeout=30)
            predecessor = {'first': 'second', 'second': 'third'}.get(state_key)
            if predecessor and not self.server.bulk_done[predecessor].wait(timeout=30):
                raise RuntimeError('bulk reverse completion timed out')
        if state_key == 'transport-close':
            self.connection.shutdown(socket.SHUT_RDWR)
            self.connection.close()
            return
        if state_key == 'retry-status':
            self.send_response(503)
            self.send_header('Retry-After', '1')
            self.send_header('Content-Length', '0')
            self.end_headers()
            return
        if state_key in ("status-401", "failure-one", "failure-two"):
            self.send_error(403 if state_key == "failure-two" else 401)
            return
        answers = {}
        for name, question in request["questions"].items():
            kind = question["type"]
            if kind == "noul":
                row = packed.get(name,state_key)
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6}.get(row, 0.9)
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                preferred = 'none' if '"evidence":"find-none"' in state_key and 'none' in keys else keys[0]
                answers[name] = {"type": kind, "probabilities": {key: 0.9 if key == preferred else 0.1 / (len(keys) - 1) for key in keys}}
        data = b'{broken' if state_key in ('malformed-backend', 'bulk-middle-bad') else json.dumps({"model": request["model"], "answers": answers, **({} if state_key == "hold-facts-no-usage" else {"usage": {"input_tokens": 1, "output_tokens": 1}})}).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        try:
            self.wfile.write(data)
            with self.server.lock:
                self.server.completions.append(state)
        except BrokenPipeError:
            pass
        if bulk_first:
            with self.server.lock:
                self.server.bulk_completion.append(state_key)
            self.server.bulk_done[state_key].set()
        if {'first','second','third'}.issubset(set(packed.values())):
            with self.server.lock:
                self.server.bulk_completion.append('packed:first,second,third')

    def log_message(self, *_):
        pass
