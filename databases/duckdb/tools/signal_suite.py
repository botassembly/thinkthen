"""SIGINT, the interrupt predicate, the host's own action, and the panic
guard (ticket 0110 decisions 7, 8, 13, and 14).

A child loads the extension and runs queries on the held arm. The parent
reads the backend's count, sends SIGINT, and times the child's answer.
The `test-hooks` build serves only R7-7 and R1-10.
"""

from __future__ import annotations

import json
import os
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from harness import HOOKS, Backend, case, child_env, expect, main, run, said, EXTENSION

CANCELLED = "thinkthen cancelled: the call was cancelled"

# The child: take the host action named in argv[2], LOAD, then run one
# statement per stdin line and print each result as one JSON line.
CHILD = r"""
import json, signal, sys
import duckdb
extension, host = sys.argv[1], sys.argv[2]
seen = []
if host == "python":
    signal.signal(signal.SIGINT, lambda number, frame: seen.append(number))
elif host == "ignore":
    signal.signal(signal.SIGINT, signal.SIG_IGN)
elif host == "default":
    signal.signal(signal.SIGINT, signal.SIG_DFL)
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute(f"LOAD '{extension}'")
print(json.dumps({"loaded": len(seen)}), flush=True)
for line in sys.stdin:
    if line.strip() == "seen":
        print(json.dumps({"seen": len(seen)}), flush=True)
        continue
    try:
        print(json.dumps({"rows": con.execute(line).fetchall()}, default=str), flush=True)
    except BaseException as error:
        print(json.dumps({"error": str(error)}), flush=True)
"""


class Child:
    def __init__(self, base: str, folder: Path, host: str = "python", extension: Path = EXTENSION, extra: dict | None = None):
        self.process = subprocess.Popen(
            [sys.executable, "-c", CHILD, str(extension), host],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            bufsize=1,
            env=child_env(base, folder, extra),
        )
        self.loaded = self.read()

    def ask(self, sql: str) -> None:
        self.process.stdin.write(sql + "\n")
        self.process.stdin.flush()

    def read(self) -> dict:
        line = self.process.stdout.readline()
        if not line:
            raise AssertionError(f"the child ended with code {self.process.wait(10)}")
        return json.loads(line)

    def interrupt(self) -> float:
        started = time.monotonic()
        self.process.send_signal(signal.SIGINT)
        return started

    def close(self) -> None:
        if self.process.poll() is None:
            self.process.stdin.close()
            self.process.wait(timeout=20)


def texts(count: int, tag: str) -> str:
    return ", ".join(f"('{tag} text {index}')" for index in range(count))


def held_cancel(query: str, in_flight: int) -> None:
    """Send SIGINT once `in_flight` requests are held, read `cancelled`
    within 100 ms, release, and see the count stay put."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            child.ask("SET thinkthen_throttle = 8")
            child.read()
            child.ask(query)
            expect(backend.wait(in_flight), in_flight, "requests in flight")
            started = child.interrupt()
            answer = child.read()
            took = time.monotonic() - started
            expect(said(answer), CANCELLED, "the held query")
            if took > 0.1:
                raise AssertionError(f"cancelled after {took * 1000:.0f} ms, over 100 ms")
            backend.release()
            time.sleep(0.2)
            expect(backend.count(), in_flight, "the count after release")
        finally:
            child.close()


@case
def r5_23_a_held_batch_stops_within_100_ms():
    held_cancel(f"SELECT thinkthen_choose('Which team?', x, ['billing', 'shipping']) FROM (VALUES {texts(64, 'batch')}) t(x)", 8)


@case
def a_held_details_call_stops_within_100_ms():
    held_cancel("SELECT thinkthen_details('Is it a refund?', 'one held text')", 1)


@case
def r1_21_the_next_query_answers_after_a_stop():
    """50 rounds on the re-holding arm: a stopped query, then one that answers."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            sent = 0
            for index in range(50):
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'stopped {index}')")
                sent = backend.wait(sent + 1)
                child.interrupt()
                expect(said(child.read()), CANCELLED, f"round {index} stop")
                backend.round()
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'answered {index}')")
                sent = backend.wait(sent + 1)
                backend.round()
                expect(child.read(), {"rows": [[True]]}, f"round {index} next query")
        finally:
            child.close()


@case
def r2_14_a_signal_between_queries_stops_nothing():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base(), Path(folder))
        try:
            for index in range(50):
                child.interrupt()
                time.sleep(0.02)
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'after {index}')")
                expect(child.read(), {"rows": [[True]]}, f"round {index}")
        finally:
            child.close()


@case
def r3_13_an_ignored_sigint_stays_ignored():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder), host="ignore")
        try:
            child.ask("SELECT thinkthen_decide('Is it a refund?', 'ignored')")
            backend.wait(1)
            child.interrupt()
            time.sleep(0.3)
            backend.release()
            expect(child.read(), {"rows": [[True]]}, "the query under an ignored SIGINT")
        finally:
            child.close()


@case
def r3_13_a_default_action_still_ends_the_process():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder), host="default")
        child.ask("SELECT thinkthen_decide('Is it a refund?', 'default')")
        backend.wait(1)
        child.interrupt()
        expect(child.process.wait(timeout=10), -signal.SIGINT, "the child's exit")
        backend.release()


@case
def r6_6_a_chained_host_handler_sees_every_signal():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            sent = 0
            for index in range(20):
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'chained {index}')")
                sent = backend.wait(sent + 1)
                child.interrupt()
                expect(said(child.read()), CANCELLED, f"signal {index}")
                backend.round()
            child.ask("seen")
            expect(child.read(), {"seen": 20}, "the host handler's count")
        finally:
            child.close()


@case
def r7_7_a_signal_inside_the_install_window_reaches_the_host():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base(), Path(folder), extension=HOOKS, extra={"thinkthen_test_hook_raise_in_install": "1"})
        try:
            child.ask("seen")
            expect(child.read(), {"seen": 1}, "the host handler's count after LOAD")
        finally:
            child.close()


@case
def r1_10_a_panic_in_each_boundary_reads_defect():
    boundaries = {
        "scalar": "SELECT thinkthen_decide('Is it a refund?', 'a')",
        "scalar init": "SELECT thinkthen_decide('Is it a refund?', 'a')",
        "usage bind": "SELECT * FROM thinkthen_usage()",
        "usage init": "SELECT * FROM thinkthen_usage()",
        "usage scan": "SELECT * FROM thinkthen_usage()",
        "warm update": "SELECT thinkthen_warm('Is it a refund?', 'a')",
        "warm finalize": "SELECT thinkthen_warm('Is it a refund?', 'a')",
    }
    with Backend() as backend:
        for boundary, query in boundaries.items():
            got = run([query, "SELECT 1"], backend.base(), extension=HOOKS, extra={"thinkthen_test_hook_panic": boundary})
            expect(said(got[0]), f"thinkthen defect: the {boundary} callback panicked: thinkthen_test_hook_panic fired in {boundary}", boundary)
            expect(got[1], {"rows": [[1]]}, f"the next query after a {boundary} panic")


STRESS = r"""
import signal, sys, threading
import duckdb
signal.signal(signal.SIGINT, lambda number, frame: None)
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute(f"LOAD '{sys.argv[1]}'")
stop = threading.Event()
def churn():
    cursor = con.cursor()
    while not stop.is_set():
        try:
            cursor.execute("SELECT string_agg(i::VARCHAR, ',') FROM range(20000) t(i)").fetchall()
        except Exception:
            pass
threads = [threading.Thread(target=churn) for _ in range(4)]
for thread in threads:
    thread.start()
print("ready", flush=True)
sys.stdin.readline()
stop.set()
for thread in threads:
    thread.join()
print("done", flush=True)
"""


@case
def r5_21_ten_thousand_signals_while_four_threads_allocate():
    """The handler takes no lock and allocates nothing, so 10,000 SIGINTs
    while four threads allocate never deadlock the process. The run ends
    under 60 s with exit 0."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = subprocess.Popen(
            [sys.executable, "-c", STRESS, str(EXTENSION)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=child_env(backend.base(), Path(folder)),
        )
        try:
            expect(child.stdout.readline().strip(), "ready", "the child's start")
            started = time.monotonic()
            for _ in range(10_000):
                child.send_signal(signal.SIGINT)
            child.stdin.write("\n")
            child.stdin.flush()
            expect(child.stdout.readline().strip(), "done", "the child's end")
            expect(child.wait(timeout=60), 0, "the child's exit")
            if time.monotonic() - started > 60:
                raise AssertionError("the stress run took over 60 s")
        finally:
            if child.poll() is None:
                child.kill()


if __name__ == "__main__":
    sys.exit(main())
