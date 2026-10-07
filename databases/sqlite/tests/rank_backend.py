"""Owned SQL rank listener serving independent saved question exchanges."""
import json
import pathlib
import sys
import threading
from http.server import BaseHTTPRequestHandler

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'sdlc/scripts'))
from loopback_server import LoopbackServer

SAVED = json.loads((pathlib.Path(__file__).parent / 'fixtures/rank-set-exchanges.json').read_text())
SET = '{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}'
ONE = '{"version":1,"questions":{"first":{"decide":"First?"}}}'
RECORDS = '{"a":"a","b":"b","c":"c","d":"a"}'
# Declared independently: first=[a,d,b,c], second=[a,d,c,b].
# Duplicate visits consume their turns; cross-member probabilities do not sort.
EXPECTED = [['a', 1, 0.7, 'first'], ['d', 2, 0.7, 'first'],
            ['b', 3, 0.6, 'first'], ['c', 4, 0.99, 'second']]


class RankBackend:
    def __init__(self):
        self.bodies = []
        owner = self

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                owner.bodies.append(body)
                answers = {}
                for key, question in body['questions'].items():
                    # Criteria are tested on the captured original question;
                    # only its wording selects an independently saved answer.
                    wording = question['instructions']
                    entry = next(row for row in SAVED['exchanges']
                                 if row['question']['instructions'] == wording)
                    answers[key] = entry['answer']
                reply = json.dumps({'model': SAVED['model'], 'usage': SAVED['usage'],
                                    'answers': answers}).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(reply)))
                self.send_header("Connection", "close")
                self.end_headers()
                try:
                    self.wfile.write(reply)
                except (BrokenPipeError, ConnectionResetError):
                    pass

            def log_message(self, *_args):
                pass

        self.server = LoopbackServer(('127.0.0.1', 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()

    @property
    def base(self):
        return f'http://127.0.0.1:{self.server.server_port}/v1'

    def count(self):
        return len(self.bodies)

    def __enter__(self):
        return self

    def __exit__(self, *_args):
        self.server.shutdown()
        self.thread.join()
        self.server.server_close()


if __name__ == '__main__':
    with RankBackend() as backend:
        print(backend.server.server_port, flush=True)
        sys.stdin.readline()
        pathlib.Path(sys.argv[1]).write_text(json.dumps(backend.bodies))
        print(backend.count(), flush=True)
