"""Independently counted synthetic backend for the PHP FFI prototype. Loopback only."""
import http.server
import json
import pathlib
import threading
import time
import socket
import re
import os

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
        state = request["state"]
        state_key = state if isinstance(state, str) else ""
        packed = state_key == 'Each question quotes the text it asks about.'
        quoted = {}
        if packed:
            for name, question in request['questions'].items():
                match = re.match(r'^The text is ("(?:\\.|[^"\\])*")\. ', question['instructions'])
                if not match:
                    raise AssertionError('packed question does not quote exactly one JSON record')
                quoted[name] = json.loads(match.group(1))
        packed_rows = list(quoted.values())
        with self.server.lock:
            with (self.server.barrier / 'request-bodies.jsonl').open('a') as receipt:
                receipt.write(json.dumps(request, ensure_ascii=False) + '\n')
            self.server.arrivals.append(state)
            self.server.attempts += 1
            bulk_first = state_key in ('first', 'second', 'third') and self.server.bulk_seen[state_key] == 0
            if state_key in self.server.bulk_seen:
                self.server.bulk_seen[state_key] += 1
        if self.path != "/generic/v1/systemone" or self.headers.get("Authorization") != "Bearer tt-canary-291":
            self.send_error(403)
            return
        hold = state_key if state_key.startswith('hold-') else ('hold-bulk-1' if 'hold-bulk-1' in packed_rows else None)
        if hold:
            (self.server.barrier / ("arrived-" + hold)).touch()
            release = self.server.barrier / ("release-" + hold)
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
            row = quoted.get(name, state_key)
            if kind == "noul":
                p = {"no": 0.1, "unsure": 0.5, "first": 0.9, "second": 0.1, "third": 0.6}.get(row, 0.9)
                if packed and row == 'second' and os.environ.get('TT_PLANT_BAD_PACKED') == '1':
                    p = 0.9
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

    def log_message(self, *_):
        pass
