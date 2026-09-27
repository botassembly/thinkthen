#!/usr/bin/env python3
"""Shared SQL test proxy, counted in SQLite's Python test ratchet.

SQLite, DuckDB, and PostgreSQL use this loopback-only proxy to prove that a backend failure remains a
value for one row while later rows in the same statement still reach the
generic conformance backend. No user key is forwarded.
"""

from __future__ import annotations

import sys
import threading
import urllib.error
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlsplit


class ConditionalBackend:
    def __init__(self, generic_base: str, sentinel: str = "private evidence") -> None:
        parsed = urlsplit(generic_base)
        if parsed.scheme != "http" or parsed.hostname != "127.0.0.1" or not parsed.path.endswith("/v1"):
            raise ValueError("the generic backend must be a loopback /v1 address")
        self.generic_base = generic_base.rstrip("/")
        self.sentinel = sentinel.encode()
        self._count = 0
        self._lock = threading.Lock()
        owner = self

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self) -> None:
                if not self.path.startswith("/v1/"):
                    self.send_error(404)
                    return
                length = int(self.headers.get("Content-Length", "0"))
                body = self.rfile.read(length)
                with owner._lock:
                    owner._count += 1
                if owner.sentinel in body:
                    status, reply = 422, b'{"error":{"message":"test refusal"}}'
                else:
                    target = owner.generic_base + self.path[len("/v1"):]
                    request = urllib.request.Request(
                        target, data=body, headers={"Content-Type": "application/json"}, method="POST"
                    )
                    try:
                        with urllib.request.urlopen(request, timeout=10) as response:
                            status, reply = response.status, response.read()
                    except urllib.error.HTTPError as error:
                        status, reply = error.code, error.read()
                self.send_response(status)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(reply)))
                self.end_headers()
                self.wfile.write(reply)

            def log_message(self, _format: str, *_args: object) -> None:
                pass

        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()

    @property
    def base(self) -> str:
        return f"http://127.0.0.1:{self.server.server_port}/v1"

    def count(self) -> int:
        with self._lock:
            return self._count

    def close(self) -> None:
        self.server.shutdown()
        self.thread.join(timeout=10)
        if self.thread.is_alive():
            raise RuntimeError("the conditional backend did not stop")
        self.server.server_close()

    def __enter__(self) -> "ConditionalBackend":
        return self

    def __exit__(self, *_: object) -> None:
        self.close()


if __name__ == "__main__":
    with ConditionalBackend(sys.argv[1], sys.argv[2]) as backend:
        print(backend.server.server_port, flush=True)
        sys.stdin.readline()
        print(backend.count(), flush=True)
