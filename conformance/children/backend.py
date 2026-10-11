"""Counted loopback System One backend for installed consumer programs."""
import http.server
import json
import os
from pathlib import Path
import socket
import threading
import time

QUOTED_STATE = 'Each question quotes the text it asks about.'


def unquoted_single(request):
    """Read one quoted original without changing the captured authored wire."""
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
    return dict(request, state=json.loads(records.pop()), questions=questions) if len(records) == 1 else request


class Backend(http.server.ThreadingHTTPServer):
    def __init__(self, barrier=None, *, key='tt-canary-290', arrivals='state',
                 packed_questions=None, distinct_packed=False, canonical_packed=False,
                 hold_first=False, concurrent=False, retry_ms=False, plant=''):
        super().__init__(('127.0.0.1', 0), Handler)
        self.barrier = Path(barrier) if barrier is not None else None
        self.key, self.arrival_format = key, arrivals
        self.packed_questions = packed_questions
        self.distinct_packed, self.canonical_packed = distinct_packed, canonical_packed
        self.hold_first, self.retry_ms, self.plant = hold_first, retry_ms, plant
        self.arrivals, self.requests, self.user_agents, self.completions = [], [], [], []
        self.attempts = self.connections = 0
        self.held_bulk_body = self.relation_body = None
        self.bulk_barrier = threading.Barrier(3)
        self.concurrent_barrier = threading.Barrier(3) if concurrent and barrier else None
        self.bulk_done = {key: threading.Event() for key in ('first', 'second', 'third')}
        self.bulk_seen = dict.fromkeys(self.bulk_done, 0)
        self.bulk_completion = []
        self.lock = threading.Lock()
        self.worker = threading.Thread(target=self.serve_forever)
        self.worker.start()

    def finish_request(self, request, address):
        with self.lock:
            self.connections += 1
        return super().finish_request(request, address)

    def close(self):
        self.shutdown()
        self.worker.join()
        self.server_close()  # Joins all owned non-daemon request workers.

    def packed(self, request):
        if request['state'] != QUOTED_STATE:
            return {}
        rows = {}
        for name, question in request['questions'].items():
            text = question.get('instructions', '')
            assert text.startswith('The text is '), text
            record, end = json.JSONDecoder().raw_decode(text, len('The text is '))
            assert isinstance(record, str) and text.startswith('. ', end), text
            if self.packed_questions is not None:
                assert question['type'] == 'noul' and text[end + 2:] in self.packed_questions, text
            if self.canonical_packed:
                assert text[:end] == 'The text is ' + json.dumps(record, ensure_ascii=False, separators=(',', ':')), text
            rows[name] = record
        if self.distinct_packed:
            assert len(rows) >= 2 and len(set(rows.values())) >= 2, rows
        if self.arrival_format == 'packed':
            assert list(rows) == [f'q{i}' for i in range(1, len(rows) + 1)], rows
            assert all(set(q) == {'type', 'instructions'} for q in request['questions'].values())
        return rows

    def identity(self, state, rows):
        if self.arrival_format == 'packed' and rows:
            return 'packed:' + ','.join(rows.values())
        if self.arrival_format != 'named':
            return state
        name = state if isinstance(state, str) else json.dumps(state, sort_keys=True)
        pair = list(rows.values())
        for first, second, identity in (
            ('hold-bulk-first', 'hold-bulk-second', 'hold-bulk-first'),
            ('hold-owned-bulk-first', 'hold-owned-bulk-second', 'hold-owned-bulk-first'),
            ('batch-one', 'batch-two', 'batch-pair')):
            if len(pair) == 2 and set(pair) == {first, second}:
                return identity
        return name


class Handler(http.server.BaseHTTPRequestHandler):
    def end_headers(self):
        # ureq can reuse an HTTP/1.0 connection unless the reply closes it.
        self.send_header("Connection", "close")
        super().end_headers()

    def do_POST(self):
        server = self.server
        body = self.rfile.read(int(self.headers['Content-Length']))
        wire = json.loads(body)
        request = unquoted_single(wire)
        state = request['state']
        state_key = state if isinstance(state, str) else ''
        rows = server.packed(request)
        identity = server.identity(state, rows)
        with server.lock:
            server.arrivals.append(wire if server.arrival_format == 'wire' else identity)
            server.requests.append(wire)
            server.user_agents.append(self.headers.get('User-Agent'))
            server.attempts += 1
            bulk_first = state_key in server.bulk_seen and server.bulk_seen[state_key] == 0
            if state_key in server.bulk_seen:
                server.bulk_seen[state_key] += 1
            if server.barrier:
                with (server.barrier / 'wire-requests.jsonl').open('ab') as capture:
                    capture.write(body + b'\n')
                for name, captured in (('request-bodies.jsonl', wire), ('user-agents.jsonl', self.headers.get('User-Agent')),
                                       ('requests.jsonl', {'name': identity, 'questions': wire['questions']})):
                    with (server.barrier / name).open('a') as capture:
                        capture.write(json.dumps(captured, ensure_ascii=False) + '\n')
            if identity == 'hold-bulk-first': server.held_bulk_body = wire
            if isinstance(identity, str) and identity.startswith('{') and 'entities' in identity: server.relation_body = wire
        if self.path != '/generic/v1/systemone' or self.headers.get('Authorization') != 'Bearer ' + server.key:
            self.send_error(403)
            return
        held = [row for row in rows.values() if row.startswith('hold-')]
        if state_key.startswith('hold-'): held = [state_key]
        if server.arrival_format == 'named' and isinstance(identity, str) and identity.startswith('hold-'): held = [identity]
        if server.hold_first and 'hold-bulk-1' in held: held = ['hold-bulk-1']
        if held and server.barrier:
            for row in held: (server.barrier / ('arrived-' + row)).touch()
            end = time.monotonic() + 60
            while any(not (server.barrier / ('release-' + row)).exists() for row in held) and time.monotonic() < end:
                time.sleep(.005)
            if any(not (server.barrier / ('release-' + row)).exists() for row in held):
                self.send_error(500)
                return
        if server.concurrent_barrier and state_key in ('failure-one', 'failure-two', 'success'):
            server.concurrent_barrier.wait(timeout=30)
        if bulk_first:
            server.bulk_barrier.wait(timeout=30)
            predecessor = {'first': 'second', 'second': 'third'}.get(state_key)
            if predecessor and not server.bulk_done[predecessor].wait(timeout=30):
                raise RuntimeError('bulk reverse completion timed out')
        if state_key == 'transport-close':
            self.connection.shutdown(socket.SHUT_RDWR); self.connection.close(); return
        if state_key == 'retry-status':
            self.send_response(503)
            self.send_header('retry-after-ms' if server.retry_ms else 'Retry-After', '40' if server.retry_ms else '1')
            self.send_header('Content-Length', '0'); self.end_headers(); return
        if state_key in ('status-401', 'failure-one', 'failure-two', 'parallel-failure-one', 'parallel-failure-two', 'other-engine-error', 'backend-failure'):
            self.send_error(403 if state_key.endswith('two') else 401); return
        answers = {}
        for name, question in request['questions'].items():
            kind = question['type']
            row = rows.get(name, state_key)
            if kind == 'noul':
                p = {'no': .1, 'unsure': .5, 'first': .9, 'second': .1, 'third': .6, 'hold-reverse-2': .1, 'batch-two': .1}.get(row, .9)
                if (server.plant, identity) in (('wrong-probability', 'yes'), ('wrong-json', 'json-decide'), ('wrong-annotate', 'annotate-one')): p = .1
                if server.plant == 'wrong-relate' and isinstance(identity, str) and identity.startswith('{'): p = .1
                if server.plant == 'swapped-bulk' and identity == 'batch-pair': p = 1 - p
                if row == 'json-decide' and os.environ.get('TT_PLANT_JSON_WRONG') == '1': p = .1
                if rows and row == 'second' and os.environ.get('TT_PLANT_BAD_PACKED') == '1': p = .9
                answers[name] = {'type': kind, 'noul': p}
            else:
                keys = list(question['criteria']) if kind != 'score' else [str(i) for i in range(len(question['criteria']))]
                preferred = 'none' if '"evidence":"find-none"' in state_key and 'none' in keys else keys[0]
                probabilities = {key: .9 if key == preferred else .1 / (len(keys) - 1) for key in keys}
                if server.plant == 'wrong-recognize' and identity == 'John Smith':
                    probabilities = {key: 1.0 if key in ('OUT', 'none of these') else 0.0 for key in keys}
                answers[name] = {'type': kind, 'probabilities': probabilities}
        reply = {'model': request['model'], 'answers': answers}
        if state_key not in ('no-usage', 'without-usage', 'hold-facts-no-usage') and 'no-usage' not in rows.values():
            reply['usage'] = {'input_tokens': 1, 'output_tokens': 1}
        data = b'{broken' if state_key in ('malformed-backend', 'bulk-middle-bad') or 'bulk-middle-bad' in rows.values() else json.dumps(reply).encode()
        self.send_response(200); self.send_header('Content-Type', 'application/json'); self.send_header('Content-Length', str(len(data))); self.end_headers()
        try:
            self.wfile.write(data)
            with server.lock: server.completions.append(identity)
        except BrokenPipeError:
            pass
        if bulk_first:
            with server.lock: server.bulk_completion.append(state_key)
            server.bulk_done[state_key].set()
        if list(rows.values()) == ['first', 'second', 'third']:
            with server.lock: server.bulk_completion.append('packed:first,second,third')

    def log_message(self, *_):
        pass
