"""Synthetic counted System One fixture. No network interface except numeric loopback."""
import http.server
import json
import threading
import pathlib
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

class Handler(http.server.BaseHTTPRequestHandler):
    # Debt 020: ureq may reuse an HTTP/1.0 connection the server closes;
    # sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md
    def end_headers(self):
        self.send_header("Connection", "close")
        super().end_headers()

    def do_POST(self):
        body = self.rfile.read(int(self.headers['Content-Length']))
        with self.server.lock:
            self.server.arrivals.append(json.loads(body))
            self.server.user_agents.append(self.headers.get('User-Agent'))
        if self.path != '/generic/v1/systemone' or self.headers.get('Authorization') != 'Bearer tt-canary-273':
            self.send_error(403)
            return
        wire = json.loads(body)
        request = one_record(wire)
        state = request['state']
        state_key = state if isinstance(state, str) else ''
        # specification/records.md, "Order and requests" and ADR 0055 item 1:
        # distinguish packed rows by their quoted JSON in each question, not
        # by the one shared request state. Only this exact wire form is decoded.
        row_by_question = {}
        if state_key == 'Each question quotes the text it asks about.':
            for name, question in request['questions'].items():
                instructions = question['instructions']
                assert isinstance(instructions, str) and instructions.startswith('The text is '), instructions
                row, end = json.JSONDecoder().raw_decode(instructions[len('The text is '):])
                assert instructions[len('The text is ') + end:].startswith('. '), instructions
                assert isinstance(row, str), row
                row_by_question[name] = row
        packed_rows = list(row_by_question.values())
        hold_key = state_key if state_key.startswith('hold-') else next((row for row in packed_rows if row.startswith('hold-bulk-')), '')
        if hold_key and self.server.barrier_dir:
            barrier = pathlib.Path(self.server.barrier_dir)
            (barrier / ('arrived-' + hold_key)).touch()
            release = barrier / ('release-' + hold_key)
            # Held up to 60 s, past every test's 30 s wait (ticket 0356).
            end = time.monotonic() + 60
            while not release.exists() and time.monotonic() < end:
                time.sleep(0.005)
            if not release.exists():
                self.send_error(500)
                return
        if state_key in ('failure-one', 'failure-two', 'success') and self.server.concurrent_barrier:
            self.server.concurrent_barrier.wait(timeout=30)
        if state_key in ('first', 'second', 'third'):
            self.server.bulk_barrier.wait(timeout=30)
            predecessor = {'first': 'second', 'second': 'third'}.get(state_key)
            if predecessor and not self.server.bulk_done[predecessor].wait(timeout=30):
                raise RuntimeError('bulk reverse completion barrier timed out')
        # The packed bulk is a single request; the former reverse-completion
        # barrier between three requests cannot occur on this pin.
        if state_key == 'retry-status':
            self.send_response(503)
            self.send_header('retry-after-ms', '40')
            self.send_header('Content-Length', '0')
            self.end_headers()
            return
        if state_key == 'transport-close':
            self.connection.shutdown(socket.SHUT_RDWR)
            self.connection.close()
            return
        if state_key in ('status-401', 'failure-one', 'failure-two'):
            self.send_error(403 if state_key == 'failure-two' else 401)
            return
        if state_key == 'malformed-backend' or state_key == 'bulk-middle-bad' or 'bulk-middle-bad' in packed_rows:
            data = b'{broken'
        else:
            answers = {}
            for name, question in request['questions'].items():
                kind = question['type']
                if kind == 'noul':
                    p = {'no': 0.1, 'unsure': 0.5, 'first': 0.9, 'second': 0.1, 'third': 0.6}.get(row_by_question.get(name, state_key), 0.9)
                    answers[name] = {'type': kind, 'noul': p}
                else:
                    keys = list(question['criteria'])
                    if kind == 'score':
                        keys = [str(i) for i in range(len(keys))]
                    answers[name] = {'type': kind, 'probabilities': {key: (0.9 if i == 0 else 0.1 / (len(keys) - 1)) for i, key in enumerate(keys)}}
            data = json.dumps({'model': request['model'], 'answers': answers, **({} if state_key == 'hold-facts-no-usage' else {'usage': {'input_tokens': 1, 'output_tokens': 1}})}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        try:
            self.wfile.write(data)
        except BrokenPipeError:
            pass  # A deadline may close the accepted request before the held reply.
        if state_key in ('first', 'second', 'third'):
            with self.server.lock:
                self.server.bulk_completion.append(state_key)
            self.server.bulk_done[state_key].set()

    def log_message(self, *_):
        pass

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier_dir=None):
        super().__init__(('127.0.0.1', 0), Handler)
        self.barrier_dir = barrier_dir
        self.concurrent_barrier = threading.Barrier(3) if barrier_dir else None
        self.bulk_barrier = threading.Barrier(3)
        self.bulk_done = {key: threading.Event() for key in ('first', 'second', 'third')}
        self.bulk_completion = []
        self.lock = threading.Lock()
        self.arrivals = []
        self.user_agents = []
        self.thread = threading.Thread(target=self.serve_forever)
        self.thread.start()
    def close(self):
        self.shutdown()
        self.thread.join()
        self.server_close()
