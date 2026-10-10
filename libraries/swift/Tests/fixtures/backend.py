"""Count three owned-session calls and hold cancellation at the loopback boundary."""
import http.server
import json
from pathlib import Path
import threading
import time


class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = Path(barrier)
        self.arrivals = []
        self.attempts = self.connections = 0
        self.lock = threading.Lock()
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()

    def finish_request(self, request, client_address):
        with self.lock: self.connections += 1
        return super().finish_request(request, client_address)

    def close(self):
        self.shutdown()
        self.worker.join()
        self.server_close()


class Handler(http.server.BaseHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Connection", "close")
        super().end_headers()

    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        question = request['questions']['q1']
        instruction = question['instructions']
        prefix = 'The text is '
        assert instruction.startswith(prefix)
        state, end = json.JSONDecoder().raw_decode(instruction[len(prefix):])
        assert instruction[len(prefix) + end:] == '. Is it?'
        with self.server.lock:
            self.server.arrivals.append(state)
            self.server.attempts += 1
        assert self.path == '/generic/v1/systemone'
        assert self.headers.get('Authorization') == 'Bearer tt-canary-294'
        if state.startswith('hold-'):
            (self.server.barrier / ('arrived-' + state)).touch()
            release = self.server.barrier / ('release-' + state)
            until = time.monotonic() + 30
            while not release.exists() and time.monotonic() < until: time.sleep(0.005)
            assert release.exists(), 'Held provider was not released'
        if state == 'status-401':
            self.send_error(401)
            return
        body = json.dumps({'model': request['model'], 'answers': {'q1': {'type': 'noul', 'noul': 0.9}}, 'usage': {'input_tokens': 1, 'output_tokens': 1}}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        try: self.wfile.write(body)
        except BrokenPipeError: pass

    def log_message(self, *_): pass
