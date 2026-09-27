"""One held scalar and aggregate SIGINT boundary on the stock DuckDB host."""

from __future__ import annotations

import argparse
import json
import queue
import select
import signal
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from signal_suite import Backend, Child  # noqa: E402  reuse the surface's loopback child
from harness import child_env  # noqa: E402  isolate the late-signal child too


LATE_CHILD = r"""
import duckdb, json, signal, sys, time
seen = []
signal.signal(signal.SIGINT, lambda number, frame: seen.append(number))
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute("LOAD '" + sys.argv[1] + "'")
def slow(value: bool) -> bool:
    print(json.dumps({"inside_slow": True}), flush=True)
    time.sleep(0.2)
    return value
con.create_function("slow", slow)
print(json.dumps({"loaded": True}), flush=True)
for line in sys.stdin:
    try:
        print(json.dumps({"rows": con.execute(line).fetchall(), "seen": len(seen)}), flush=True)
    except BaseException as error:
        print(json.dumps({"error": str(error), "seen": len(seen)}), flush=True)
"""


def line(child: subprocess.Popen[str]) -> dict:
    ready, _, _ = select.select([child.stdout], [], [], 3)
    assert ready, "the late-signal child did not answer in three seconds"
    return json.loads(child.stdout.readline())


def late(extension: Path) -> None:
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = subprocess.Popen(
            [sys.executable, "-c", LATE_CHILD, str(extension)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
            env=child_env(backend.base(), Path(folder)),
        )
        try:
            assert line(child) == {"loaded": True}
            child.stdin.write("SELECT slow(thinkthen_decide('Is it a refund?', 'late call'))\n")
            child.stdin.flush()
            assert line(child) == {"inside_slow": True}, "the decision had not finished"
            assert backend.count() == 1, "the marked decision did not send exactly once"
            child.send_signal(signal.SIGINT)
            assert line(child) == {"rows": [[True]], "seen": 1}, "the ending query did not own its signal"
            child.stdin.write("SELECT thinkthen_decide('Is it a refund?', 'next call')\n")
            child.stdin.flush()
            assert line(child) == {"rows": [[True]], "seen": 1}, "the next query inherited a stop"
            assert backend.count() == 2, "the next query sent another request"
        finally:
            child.terminate()
            try:
                child.wait(timeout=3)
            except subprocess.TimeoutExpired:
                child.kill()
                child.wait(timeout=3)


def answer(child: Child, backend: Backend) -> dict:
    found: queue.Queue[dict | BaseException] = queue.Queue()

    def read() -> None:
        try:
            found.put(child.read())
        except BaseException as error:
            found.put(error)

    threading.Thread(target=read, daemon=True).start()
    try:
        value = found.get(timeout=2)
    except queue.Empty as error:
        backend.release()
        raise AssertionError("a held call did not stop within two seconds") from error
    if isinstance(value, BaseException):
        raise value
    return value


def held(extension: Path, sql: str) -> None:
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder), extension=extension)
        try:
            child.ask(sql)
            assert backend.wait(1) == 1
            started = child.interrupt()
            stopped = answer(child, backend)
            elapsed = time.monotonic() - started
            assert "thinkthen cancelled: the call was cancelled" in stopped.get("error", ""), stopped
            assert elapsed < 0.1, f"held query stopped after {elapsed:.3f} s"
            backend.release()
            child.ask("SELECT thinkthen_decide('Is it a refund?', 'the next query')")
            assert child.read() == {"rows": [[True]]}, "the next query inherited a stop"
            assert backend.count() == 2, "the stopped worker started another request"
            child.interrupt()
            time.sleep(0.02)
            child.ask("SELECT thinkthen_decide('Is it a refund?', 'after a signal between queries')")
            assert child.read() == {"rows": [[True]]}, "a signal between queries stopped the next one"
        finally:
            backend.release()
            child.close()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extension", required=True, type=Path)
    extension = parser.parse_args().extension.resolve(strict=True)
    held(extension, "SELECT thinkthen_details('Is it a refund?', 'one held text')")
    held(extension, "SELECT thinkthen_choose('Which team?', 'one held text', ['billing', 'shipping'])")
    held(extension, "SELECT thinkthen_recognize('one held text', ['person'])")
    held(extension, "SELECT thinkthen_warm('Is it a refund?', 'one held text')")
    late(extension)
    print("C++ held grouped, listed, nested, warm, and late SIGINT boundaries pass")


if __name__ == "__main__":
    main()
