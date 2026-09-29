"""Independently counted synthetic backend for the Go cgo prototype. Loopback only."""
import http.server
import json
import pathlib
import threading
import time
import socket
import re

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
    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        request = json.loads(body)
        if request.get('state') == 'Each question quotes the text it asks about.' or isinstance(request.get('state'), dict) or b'bulk-middle-bad' in body or str(request.get('state', '')).startswith('release-'):
            with self.server.lock:
                index = self.server.attempts
                (self.server.barrier / f'wire-body-{index}.json').write_bytes(body)
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        packed_hold = state_key == 'Each question quotes the text it asks about.' and any('"hold-bulk-1"' in str(q.get('instructions','')) for q in request['questions'].values())
        packed_ordered = state_key == 'Each question quotes the text it asks about.' and all(any(f'"{name}"' in str(q.get('instructions','')) for q in request['questions'].values()) for name in ('first','second','third'))
        with self.server.lock:
            self.server.arrivals.append(state)
            self.server.attempts += 1
            bulk_first = state_key in ('first', 'second', 'third') and self.server.bulk_seen[state_key] == 0
            if state_key in self.server.bulk_seen:
                self.server.bulk_seen[state_key] += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-275":
            self.send_error(403)
            return
        if isinstance(state, str) and (state.startswith("hold-") or packed_hold):
            barrier_name = 'hold-bulk-1' if packed_hold else state
            (self.server.barrier / ("arrived-" + barrier_name)).touch()
            release = self.server.barrier / ("release-" + barrier_name)
            end = time.monotonic() + 10
            while not release.exists() and time.monotonic() < end:
                time.sleep(0.005)
            if not release.exists():
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
        if state_key in ("status-401", "failure-one", "failure-two", "parallel-failure-one", "parallel-failure-two"):
            self.send_error(403 if state_key.endswith("two") else 401)
            return
        answers = {}
        for name, question in request["questions"].items():
            kind = question["type"]
            if kind == "noul":
                match = re.fullmatch(r'The text is "([^"\\]*)"\. .*', str(question.get('instructions', '')))
                record = match.group(1) if match else state_key
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6}.get(record, 0.9)
                answers[name] = {"type": kind, "noul": p}
            else:
                keys = list(question["criteria"])
                if kind == "score":
                    keys = [str(i) for i in range(len(keys))]
                preferred = 'none' if '"evidence":"find-none"' in state_key and 'none' in keys else keys[0]
                answers[name] = {"type": kind, "probabilities": {key: 0.9 if key == preferred else 0.1 / (len(keys) - 1) for key in keys}}
        packed_middle_bad = any('"bulk-middle-bad"' in str(q.get('instructions', '')) for q in request['questions'].values())
        data = b'{broken' if state_key in ('malformed-backend', 'bulk-middle-bad') or packed_middle_bad else json.dumps({"model": request["model"], "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}}).encode()
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
        if packed_ordered:
            with self.server.lock:
                self.server.bulk_completion.append('first+second+third (one packed request)')

    def log_message(self, *_):
        pass
