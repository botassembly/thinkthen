"""Shared SQL image loopback: saved replies, captured bodies, no headers."""

import base64
import json
import pathlib
import sys
import threading
from http.server import BaseHTTPRequestHandler

ROOT = pathlib.Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "specification/fixtures/images"
sys.path.insert(0, str(ROOT / "sdlc/scripts"))
from loopback_server import LoopbackServer


class ImageBackend:
    def __init__(self):
        self.bodies = []
        self.status = 200
        self.reply = None
        self.hold = False
        self.arrived = threading.Event()
        self.release = threading.Event()
        owner = self

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
                owner.bodies.append(body)
                owner.arrived.set()
                if owner.hold:
                    owner.release.wait(timeout=30)
                kind = body["questions"]["q1"]["type"]
                verb = {"noul": "decide", "choice": "choose", "score": "score"}[kind]
                reply = owner.reply or (FIXTURE / f"liquid-{verb}-reply.json").read_bytes()
                self.send_response(owner.status)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(reply)))
                self.send_header("Connection", "close")
                self.end_headers()
                try:
                    self.wfile.write(reply)
                except (BrokenPipeError, ConnectionResetError):
                    pass

            def log_message(self, *_args):
                pass

        self.server = LoopbackServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()

    @property
    def base(self):
        return f"http://127.0.0.1:{self.server.server_port}/v1"

    def count(self):
        return len(self.bodies)

    def __enter__(self):
        return self

    def __exit__(self, *_args):
        self.release.set()
        self.server.shutdown()
        self.thread.join()
        self.server.server_close()


def expected_body(verb, names=("red.png", "blue.png", "red.png"), text="Compare originals."):
    question = {
        "decide": {"type": "noul", "instructions": "Is red visible?"},
        "choose": {"type": "choice", "instructions": "Which color?", "criteria": {"red": None, "blue": None}},
        "score": {"type": "score", "instructions": "How red?", "criteria": ["none", "all"]},
    }[verb]
    images = ["data:image/png;base64," + base64.b64encode((FIXTURE / name).read_bytes()).decode() for name in names]
    return {"state": text, "model": "d1", "questions": {"q1": question}, "images": images}


if __name__ == "__main__":
    with ImageBackend() as backend:
        print(backend.server.server_port, flush=True)
        for line in sys.stdin:
            if line.strip() == "quit":
                break
        pathlib.Path(sys.argv[1]).write_text(json.dumps(backend.bodies))
        print(backend.count(), flush=True)
