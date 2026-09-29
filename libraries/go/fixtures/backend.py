"""Independently counted synthetic backend for the Go cgo prototype. Loopback only."""
import http.server
import json
import pathlib
import threading
import time
import socket

PACKED_EVIDENCE = 'Each question quotes the text it asks about.'


def decode_packed_decide(request):
    """ADR 0055 item 1 and specification/records.md: decode exact quoted per-row questions."""
    if request['state'] != PACKED_EVIDENCE:
        return None
    result = {}
    for key, question in request['questions'].items():
        if question.get('type') != 'noul':
            raise ValueError('packed decision needs noul question')
        text = question.get('instructions', '')
        if not text.startswith('The text is '):
            raise ValueError('packed decision lacks record quote')
        record, end = json.JSONDecoder().raw_decode(text[len('The text is '):])
        if not isinstance(record, str) or text[len('The text is ')+end:] != '. Is it?':
            raise ValueError('packed decision has wrong record/question form')
        result[key] = record
    if len(result) < 2 or len(set(result.values())) < 2:
        raise ValueError('packed decision requires distinct records')
    return result

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.requests = []
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
    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        request = json.loads(body)
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        try:
            packed = decode_packed_decide(request)
        except (ValueError, json.JSONDecodeError):
            self.send_error(400, 'bad packed request')
            return
        packed_rows = list(packed.values()) if packed is not None else []
        with self.server.lock:
            self.server.arrivals.append(state)
            self.server.requests.append(request)
            self.server.attempts += 1
            bulk_first = state_key in ('first', 'second', 'third') and self.server.bulk_seen[state_key] == 0
            if state_key in self.server.bulk_seen:
                self.server.bulk_seen[state_key] += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-274":
            self.send_error(403)
            return
        held = [s for s in packed_rows if s.startswith('hold-')]
        if isinstance(state, str) and state.startswith('hold-'):
            held.append(state)
        for text in held:
            (self.server.barrier / ('arrived-' + text)).touch()
        if held:
            end = time.monotonic() + 10
            while any(not (self.server.barrier / ('release-' + text)).exists() for text in held) and time.monotonic() < end:
                time.sleep(0.005)
            if any(not (self.server.barrier / ('release-' + text)).exists() for text in held):
                self.send_error(500)
                return
        if bulk_first:
            self.server.bulk_barrier.wait(timeout=5)
            predecessor = {'first': 'second', 'second': 'third'}.get(state_key)
            if predecessor and not self.server.bulk_done[predecessor].wait(timeout=5):
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
                row = packed[name] if packed is not None else state_key
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6}.get(row, 0.9)
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                preferred = 'none' if '"evidence":"find-none"' in state_key and 'none' in keys else keys[0]
                answers[name] = {"type": kind, "probabilities": {key: 0.9 if key == preferred else 0.1 / (len(keys) - 1) for key in keys}}
        data = b'{broken' if state_key == 'malformed-backend' or 'bulk-middle-bad' in packed_rows else json.dumps({"model": request["model"], "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}}).encode()
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
        if packed_rows == ['first', 'second', 'third']:
            with self.server.lock:
                self.server.bulk_completion.append('packed:first,second,third')

    def log_message(self, *_):
        pass
