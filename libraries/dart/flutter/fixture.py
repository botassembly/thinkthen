"""Counted loopback backend; request shapes follow specification/backends.md and recognize.md."""
import http.server
import json
import pathlib
import threading
import time

class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier, plant=''):
        super().__init__(("127.0.0.1", 0), Handler)
        self.barrier = pathlib.Path(barrier)
        self.arrivals = []
        self.held_bulk_body = None
        self.relation_body = None
        self.completions = []
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
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        state = body['state']
        name = state if isinstance(state, str) else json.dumps(state, sort_keys=True)
        # ADR 0055 batches distinct decide rows into one quoted-evidence request.
        if state == 'Each question quotes the text it asks about.':
            rows = [q.get('instructions', '') for q in body['questions'].values()]
            if len(rows) == 2 and any('The text is "hold-bulk-first".' in q for q in rows) and any('The text is "hold-bulk-second".' in q for q in rows):
                name = 'hold-bulk-first'
            elif len(rows) == 2 and any('The text is "batch-one".' in q for q in rows) and any('The text is "batch-two".' in q for q in rows):
                name = 'batch-pair'
        if self.path != '/generic/v1/systemone' or self.headers.get('Authorization') != 'Bearer tt-canary-300':
            self.send_error(403)
            return
        with self.server.lock:
            self.server.arrivals.append(name)
            (self.server.barrier / 'requests.jsonl').open('a').write(json.dumps({'name': name, 'body': body}, ensure_ascii=False) + '\n')
            if name == 'hold-bulk-first': self.server.held_bulk_body = body
            if name.startswith('{') and 'entities' in name: self.server.relation_body = body
        if name.startswith('hold-'):
            (self.server.barrier / ('arrived-' + name)).touch()
            release = self.server.barrier / ('release-' + name)
            end = time.monotonic() + 15
            while not release.exists() and time.monotonic() < end:
                time.sleep(.005)
            if not release.exists():
                self.send_error(504)
                return
        if name == 'backend-failure':
            self.send_error(401)
            return
        answers = {}
        for key, question in body['questions'].items():
            kind = question['type']
            if kind == 'noul':
                row = question.get('instructions', '')
                p = {'no': .1, 'unsure': .5}.get(name, .9)
                if name == 'batch-pair': p = .1 if '"batch-two"' in row else .9
                if self.server.plant == 'wrong-probability' and name == 'yes': p = .1
                if self.server.plant == 'wrong-json' and name == 'json-decide': p = .1
                if self.server.plant == 'wrong-annotate' and name == 'annotate-one': p = .1
                if self.server.plant == 'wrong-relate' and name.startswith('{'): p = .1
                if self.server.plant == 'swapped-bulk' and name == 'batch-pair': p = .9 if '"batch-two"' in row else .1
                answers[key] = {'type': kind, 'noul': p}
            else:
                keys = list(question['criteria'])
                if kind == 'score': keys = [str(i) for i in range(len(keys))]
                if self.server.plant == 'wrong-recognize' and name == 'John Smith':
                    answers[key] = {'type': kind, 'probabilities': {k: 1.0 if k in ('OUT', 'none of these') else 0.0 for k in keys}}
                else:
                    answers[key] = {'type': kind, 'probabilities': {k: .9 if i == 0 else .1 / (len(keys)-1) for i,k in enumerate(keys)}}
        data = json.dumps({'model': body['model'], 'answers': answers, 'usage': {'input_tokens': 1, 'output_tokens': 1}}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        try:
            self.wfile.write(data)
            with self.server.lock: self.server.completions.append(name)
        except BrokenPipeError:
            pass
    def log_message(self, *_): pass
