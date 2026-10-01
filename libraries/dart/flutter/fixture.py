"""Counted loopback backend for the Flutter host test and embedder; request shapes follow specification/backends.md."""
import http.server
import json
import pathlib
import threading


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
    def __init__(self, barrier, plant=''):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.last_body = None
        self.lock = threading.Lock()
        self.plant = plant
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()
    def close(self):
        self.shutdown()
        self.worker.join()
        self.server_close()
        # ThreadingHTTPServer owns a private set of non-daemon request threads.
        self._threads.join()

class Handler(http.server.BaseHTTPRequestHandler):
    # Debt 020: ureq may reuse an HTTP/1.0 connection the server closes;
    # sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md
    def end_headers(self):
        self.send_header("Connection", "close")
        super().end_headers()

    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        state = one_record(body)['state']
        name = state if isinstance(state, str) else json.dumps(state, sort_keys=True)
        if self.path != '/generic/v1/systemone' or self.headers.get('Authorization') != 'Bearer tt-canary-300':
            self.send_error(403)
            return
        with self.server.lock:
            self.server.arrivals.append(name)
            self.server.last_body = body
            (self.server.barrier / 'requests.jsonl').open('a').write(json.dumps({'name': name, 'body': body}, ensure_ascii=False) + '\n')
        p = .1 if self.server.plant == 'wrong-probability' else .9
        answers = {key: {'type': 'noul', 'noul': p} for key in body['questions']}
        data = json.dumps({'model': body['model'], 'answers': answers, 'usage': {'input_tokens': 1, 'output_tokens': 1}}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        self.wfile.write(data)
    def log_message(self, *_): pass
