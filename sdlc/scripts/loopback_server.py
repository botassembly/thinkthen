"""The one loopback HTTP server class for Python test and release servers.

HTTPServer.server_bind names the server through socket.getfqdn, a reverse
lookup that stalls about 35 s on a macOS GitHub runner (ticket 0386). This
class names itself from its bound address instead.

Run as a script, it serves DIR and writes its port to PORT_FILE once it listens:
python3 loopback_server.py DIR PORT_FILE
"""

import functools
import http.server
import os
import socketserver
import sys


class LoopbackServer(http.server.ThreadingHTTPServer):
    def server_bind(self) -> None:
        socketserver.TCPServer.server_bind(self)
        self.server_name, self.server_port = self.server_address[:2]


if __name__ == "__main__":
    directory, port_file = sys.argv[1:]
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=directory)
    server = LoopbackServer(("127.0.0.1", 0), handler)
    # The port file appears whole, so a reader never sees part of the number.
    with open(port_file + ".part", "w", encoding="utf-8") as out:
        out.write(str(server.server_port))
    os.replace(port_file + ".part", port_file)
    server.serve_forever()
