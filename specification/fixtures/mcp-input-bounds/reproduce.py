"""Owned Linux reproductions against a native command; no external services."""
import json
import os
from pathlib import Path
import random
import select
import socket
import struct
import subprocess
import sys
import tempfile
import zlib


def frame(child, value):
    child.stdin.write(json.dumps(value).encode() + b"\n")
    child.stdin.flush()


def response(child, seconds=5):
    if not select.select([child.stdout], [], [], seconds)[0]:
        return None
    return json.loads(child.stdout.readline())


def call(arguments):
    return {"jsonrpc": "2.0", "id": 2, "method": "tools/call",
            "params": {"name": "decide", "arguments": arguments}}


def start(binary, root, address):
    env = {"PATH": os.defpath, "HOME": str(root), "XDG_CONFIG_HOME": str(root),
           "XDG_STATE_HOME": str(root), "XDG_CACHE_HOME": str(root),
           "THINKTHEN_API_KEY": "owned-fixture-key", "LANG": "C.UTF-8"}
    child = subprocess.Popen([binary, "mcp", "--backend", "liquid", "--model", "d1",
                              "--url", address, "--no-cache"], env=env, cwd=root,
                             stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                             stderr=subprocess.PIPE, bufsize=0)
    frame(child, {"jsonrpc": "2.0", "id": 1, "method": "initialize",
                  "params": {"protocolVersion": "2025-11-25", "capabilities": {},
                             "clientInfo": {"name": "owned-fixture", "version": "1"}}})
    assert response(child)["id"] == 1
    frame(child, {"jsonrpc": "2.0", "method": "notifications/initialized"})
    return child


def stop(child):
    child.kill()
    child.communicate(timeout=5)


def png():
    def chunk(name, body):
        return (struct.pack(">I", len(body)) + name + body
                + struct.pack(">I", zlib.crc32(name + body)))
    noise = random.Random(481).randbytes(256 * 256 * 3)
    rows = b"".join(b"\0" + noise[n:n + 768] for n in range(0, len(noise), 768))
    return (b"\x89PNG\r\n\x1a\n"
            + chunk(b"IHDR", struct.pack(">IIBBBBB", 256, 256, 8, 2, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(rows)) + chunk(b"IEND", b""))


def main():
    binary = str(Path(sys.argv[1]).resolve())
    with tempfile.TemporaryDirectory(prefix="thinkthen-input-bounds-") as folder:
        root = Path(folder)
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            listener.listen()
            address = "http://127.0.0.1:%d" % listener.getsockname()[1]
            image = root / "image.png"
            image.write_bytes(png())
            rows = [{"images": [str(image)]} for _ in range(100)]
            rows.append({"images": [str(root / "absent.png")]})
            child = start(binary, root, address)
            try:
                before = Path(f"/proc/{child.pid}/status").read_text()
                request = call({"question": "q", "inputs": rows})
                frame(child, request)
                result = response(child, 20)
                assert result is not None and result["result"]["isError"]
                after = Path(f"/proc/{child.pid}/status").read_text()
                assert "could not" in json.dumps(result), result
                print("aggregate: frame bytes=%d; repeated original bytes=%d; later missing file reached" %
                      (len(json.dumps(request).encode()) + 1, image.stat().st_size * 100))
                for label, status in [("before", before), ("after", after)]:
                    print(label, next(line for line in status.splitlines() if line.startswith("VmHWM:")))
            finally:
                stop(child)
            fifo = root / "question.fifo"
            os.mkfifo(fifo)
            for selector in [str(fifo), "/dev/stdin"]:
                child = start(binary, root, address)
                try:
                    frame(child, call({"question_file": selector, "evidence": "x"}))
                    assert response(child, 0.5) is None
                    frame(child, {"jsonrpc": "2.0", "id": 3, "method": "ping"})
                    ping = response(child)
                    if ping is not None and ping.get("id") == 3:
                        frame(child, call({"question": "q", "evidence": "x"}))
                        busy = response(child)
                        assert busy is not None and "error" in busy, busy
                        print("selector:", Path(selector).name, "worker blocked; ping survives; next tool is busy")
                    else:
                        assert selector == "/dev/stdin", ping
                        print("selector: stdin worker blocked; next ping lost or malformed by competing read")
                    frame(child, {"jsonrpc": "2.0", "method": "notifications/cancelled",
                                  "params": {"requestId": 2}})
                    child.stdin.close()
                    child.stdin = None
                    try:
                        code = child.wait(timeout=0.5)
                        print("selector:", Path(selector).name, "EOF joined; exit=", code)
                    except subprocess.TimeoutExpired:
                        print("selector:", Path(selector).name, "cancel/EOF could not join; kill required")
                finally:
                    stop(child)
            assert not select.select([listener], [], [], 0)[0], "unexpected backend send"
            print("backend connections=0; every child killed and reaped")


if __name__ == "__main__":
    main()
