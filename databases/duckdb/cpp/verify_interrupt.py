"""One held scalar and aggregate SIGINT boundary on the stock DuckDB host."""

from __future__ import annotations

import argparse
import queue
import sys
import tempfile
import threading
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from signal_suite import Backend, Child  # noqa: E402  reuse the surface's loopback child


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
    print("C++ held grouped, listed, nested, and warm SIGINT boundaries pass")


if __name__ == "__main__":
    main()
