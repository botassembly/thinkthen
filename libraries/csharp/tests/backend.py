"""Independently counted synthetic backend for the C# package rehearsal. Loopback only."""
import http.server
import json
import pathlib
import threading
import time
import socket
import re


QUOTED_STATE = 'Each question quotes the text it asks about.'


def unquoted_single(request):
    """ADR 0111 quotes each record in its own question, a batch of one
    included. A request whose questions all quote one record reads here as
    that record's single request, so behaviors stay keyed on the record."""
    if request.get('state') != QUOTED_STATE:
        return request
    records, questions = set(), {}
    for name, question in request['questions'].items():
        text = question.get('instructions')
        if not isinstance(text, str) or not text.startswith('The text is '):
            return request
        record, end = json.JSONDecoder().raw_decode(text, len('The text is '))
        if not text.startswith('. ', end):
            return request
        records.add(json.dumps(record))
        questions[name] = dict(question, instructions=text[end + 2:])
    if len(records) != 1:
        return request
    return dict(request, state=json.loads(records.pop()), questions=questions)

# specification/records.md "Order and requests" and ADR 0048 item 1:
# a packed decide batch quotes one JSON record in each distinct wire question.
PACKED_STATE = 'Each question quotes the text it asks about.'
def quoted_record(question):
    match = re.fullmatch(r'The text is ("(?:\\.|[^"\\])*")\. (?:Is it\?|Is it relevant\?)', question['instructions'])
    if match is None:
        raise AssertionError('unexpected packed decide question: ' + repr(question['instructions']))
    return json.loads(match.group(1))

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.user_agents = []
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
        request = unquoted_single(wire)
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        packed = state_key == PACKED_STATE
        packed_records = [quoted_record(q) for q in request['questions'].values()] if packed else []
        if packed:
            assert len(packed_records) >= 2, packed_records
            assert list(request['questions']) == [f'q{i}' for i in range(1, len(packed_records) + 1)]
            assert all(q['type'] == 'noul' and set(q) == {'type', 'instructions'} for q in request['questions'].values())
        arrival_identity = 'packed:' + ','.join(packed_records) if packed else state
        with self.server.lock:
            with (self.server.barrier / 'wire-requests.jsonl').open('ab') as capture:
                capture.write(body + b'\n')
            self.server.arrivals.append(arrival_identity)
            self.server.user_agents.append(self.headers.get("User-Agent"))
            self.server.attempts += 1
            bulk_first = state_key in ('first', 'second', 'third') and self.server.bulk_seen[state_key] == 0
            if state_key in self.server.bulk_seen:
                self.server.bulk_seen[state_key] += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-290":
            self.send_error(403)
            return
        held = 'hold-bulk-1' if packed and 'hold-bulk-1' in packed_records else state_key if state_key.startswith('hold-') else None
        if held is not None:
            (self.server.barrier / ("arrived-" + held)).touch()
            release = self.server.barrier / ("release-" + held)
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
        for index, (name, question) in enumerate(request["questions"].items()):
            kind = question["type"]
            if kind == "noul":
                effective_state = packed_records[index] if packed else state_key
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6}.get(effective_state, 0.9)
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                preferred = 'none' if '"evidence":"find-none"' in state_key and 'none' in keys else keys[0]
                answers[name] = {"type": kind, "probabilities": {key: 0.9 if key == preferred else 0.1 / (len(keys) - 1) for key in keys}}
        reply = {"model": request["model"], "answers": answers}
        if state_key != 'no-usage': reply["usage"] = {"input_tokens": 1, "output_tokens": 1}
        data = b'{broken' if state_key in ('malformed-backend', 'bulk-middle-bad') else json.dumps(reply).encode()
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
        elif packed and packed_records == ['first', 'second', 'third']:
            with self.server.lock:
                self.server.bulk_completion.append('packed:first,second,third')

    def log_message(self, *_):
        pass
