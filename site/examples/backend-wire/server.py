# A stand-in model server: prints what ThinkThen sends, then answers 400.
import json, sys
from http.server import BaseHTTPRequestHandler, HTTPServer

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["content-length"])))
        scheme = self.headers["authorization"].split(" ")[0]
        print(f"server: {self.command} {self.path}", file=sys.stderr)
        print(f"server: content-type {self.headers['content-type']},"
              f" authorization {scheme} <key>", file=sys.stderr)
        print(f"server: body fields {sorted(body)}", file=sys.stderr)
        self.send_response(400)
        self.end_headers()
    def log_message(self, *args):
        pass

server = HTTPServer(("127.0.0.1", 0), Handler)
print(server.server_address[1], flush=True)
server.serve_forever()
