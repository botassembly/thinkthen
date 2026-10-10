"""SIGINT, the interrupt predicate, the host's own action, and the panic
guard (ticket 0110 decisions 7, 8, 13, and 14).

A child loads the extension and runs queries on the held arm. The parent
reads the backend's count, sends SIGINT, and reads the child's answer while
the reply is still held. The stress profile also times the answer against
the 100 ms promise (ticket 0352). The `test-hooks` build serves only R7-7
and R1-10.
"""

from __future__ import annotations

import json
import os
import select
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from harness import EXTENSION, STRESS, Backend, case, child_env, expect, main, said, timed, within
from harness import select as select_cases

CANCELLED = "thinkthen cancelled: the call was cancelled (retryable: no)"

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
con.execute("SET enable_progress_bar = false")
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
    before the release (within 100 ms under stress), release, and see the
    count stay put."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            child.ask("SET thinkthen_throttle = 8")
            child.read()
            child.ask(query)
            expect(backend.wait(in_flight), in_flight, "requests in flight")
            started = child.interrupt()
            answer = child.read()
            within(started, 0.1, "the cancel")
            expect(said(answer), CANCELLED, "the held query")
            backend.release()
            time.sleep(0.2)
            expect(backend.count(), in_flight, "the count after release")
        finally:
            # Release a held arm even when an assertion fails, so cleanup
            # cannot hide the actual count or timing failure.
            backend.release()
            child.close()


@case
def complete_file_cancel_returns_while_provider_remains_held():
    with tempfile.TemporaryDirectory(prefix='thinkthen-held-file-') as tmp:
        path=Path(tmp)/'source';path.write_text('Refund held file.')
        inputs=json.dumps({'files':{'paths':[str(path)]},'incremental':True})
        literal=lambda value:"'"+value.replace("'","''")+"'"
        held_cancel("SELECT thinkthen_decide_complete('Refund?',"+literal(inputs)+",'{\"batch\":1}')",1)


@case
def rank_set_cancel_stops_the_shared_worker_without_later_sends():
    held_cancel("SELECT key FROM thinkthen_rank_set('{\"version\":1,\"questions\":{\"first\":{\"decide\":\"First?\"},\"second\":{\"decide\":\"Second?\"}}}', '{\"a\":\"a\",\"b\":\"b\"}', '{\"batch\":\"max\"}')", 1)


@timed
def r5_23_a_held_batch_stops_within_100_ms():
    # The lazy default packs these 64 rows into one held request. Cancellation
    # must stop that request without admitting a second one after release.
    held_cancel(f"SELECT thinkthen_choose('Which team?', x, ['billing', 'shipping']) FROM (VALUES {texts(64, 'batch')}) t(x)", 1)


@timed
def a_held_details_call_stops_within_100_ms():
    held_cancel("SELECT thinkthen_details('Is it a refund?', 'one held text')", 1)


PAIR = "SELECT * FROM thinkthen_relate('SELECT * FROM (VALUES (1, ''Ada'', ''person''), (2, ''Acme'', ''organization'')) v(id, name, kind)', ['works_for=person:organization'])"


@timed
def a_held_relate_stops_within_100_ms():
    held_cancel(PAIR, 1)


@timed
def the_bridge_stops_a_running_relate_query_within_100_ms():
    """Decision 7: the SIGINT reaches the kept connection's running query
    through the bridge, not only the engine call after it. The routine run
    repeats the signal until the answer comes, since a SIGINT before the scan
    starts stops nothing; the stress run times one signal after 0.5 s."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base(), Path(folder))
        try:
            child.ask("SELECT * FROM thinkthen_relate('SELECT i AS id, ''n'' AS name, ''k'' AS kind FROM range(100000000000) t(i) WHERE i < 0', ['near'])")
            if STRESS:
                time.sleep(0.5)
                started = child.interrupt()
            else:
                started = child.interrupt()
                while not select.select([child.process.stdout], [], [], 0.05)[0]:
                    started = child.interrupt()
            answer = child.read()
            within(started, 0.1, "the cancel")
            expect(said(answer), CANCELLED, "the running relate query")
            expect(backend.count(), 0, "counted sends")
        finally:
            child.close()


THREADS = r"""
import json, signal, sys, threading
import duckdb
signal.signal(signal.SIGINT, lambda number, frame: None)
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute("SET enable_progress_bar = false")
con.execute(f"LOAD '{sys.argv[1]}'")
con.execute("SET GLOBAL thinkthen_throttle = 8")
queries = json.loads(sys.argv[2])
def ask(query):
    try:
        print(json.dumps({"rows": con.cursor().execute(query).fetchall()}, default=str), flush=True)
    except BaseException as error:
        print(json.dumps({"error": str(error)}), flush=True)
threads = [threading.Thread(target=ask, args=(query,)) for query in queries]
for thread in threads:
    thread.start()
for thread in threads:
    thread.join()
"""


def held_queries_stop_together(rounds: int):
    """Exercise both thread widths and keep the repeated run opt-in."""
    queries = [PAIR, "SELECT thinkthen_decide('Is it a refund?', 'held a')", "SELECT thinkthen_details('Is it a refund?', 'held b')", "SELECT count(*) FROM thinkthen_rank('Is it a refund?', '{\"c\":\"held c\"}')"]
    for run_number in range(rounds):
        for width in (2, 4):
            with Backend() as backend, tempfile.TemporaryDirectory() as folder:
                child = subprocess.Popen(
                    [sys.executable, "-c", THREADS, str(EXTENSION), json.dumps(queries[:width])],
                    stdout=subprocess.PIPE, text=True, env=child_env(backend.base("arm/held"), Path(folder)),
                )
                try:
                    expect(backend.wait(width), width, "requests in flight")
                    started = time.monotonic()
                    child.send_signal(signal.SIGINT)
                    answers = [json.loads(child.stdout.readline()) for _ in range(width)]
                    within(started, 0.1, f"run {run_number} with {width} queries")
                    expect([said(answer) for answer in answers], [CANCELLED] * width, f"run {run_number} with {width} queries")
                    backend.release()
                    time.sleep(0.2)
                    expect(backend.count(), width, "the count after release")
                finally:
                    child.wait(timeout=20)


@case
def one_signal_stops_two_and_four_held_queries():
    held_queries_stop_together(1)


@case
def r4_22_one_signal_stops_every_held_query_and_relate_asks_once():
    """Twenty rounds at each width, retained as an opt-in campaign."""
    held_queries_stop_together(20)


def stop_then_answer(rounds: int):
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            sent = 0
            for index in range(rounds):
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
def the_next_query_answers_after_one_stop():
    stop_then_answer(1)


@case
def r1_21_the_next_query_answers_after_a_stop():
    """Fifty stop/recovery rounds, retained as an opt-in campaign."""
    stop_then_answer(50)


def signal_between_queries(rounds: int):
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base(), Path(folder))
        try:
            for index in range(rounds):
                child.interrupt()
                # The host handler counts the signal, so the query goes only after it landed.
                child.ask("seen")
                expect(child.read(), {"seen": index + 1}, f"round {index} signal")
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'after {index}')")
                expect(child.read(), {"rows": [[True]]}, f"round {index}")
        finally:
            child.close()


@case
def a_signal_between_queries_stops_nothing_once():
    signal_between_queries(1)


@case
def r2_14_a_signal_between_queries_stops_nothing():
    """Fifty between-query interrupts, retained as an opt-in campaign."""
    signal_between_queries(50)


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


def chained_host_handler(rounds: int):
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = Child(backend.base("arm/held"), Path(folder))
        try:
            sent = 0
            for index in range(rounds):
                child.ask(f"SELECT thinkthen_decide('Is it a refund?', 'chained {index}')")
                sent = backend.wait(sent + 1)
                child.interrupt()
                expect(said(child.read()), CANCELLED, f"signal {index}")
                backend.round()
            child.ask("seen")
            expect(child.read(), {"seen": rounds}, "the host handler's count")
        finally:
            child.close()


@case
def a_chained_host_handler_sees_two_signals():
    chained_host_handler(2)


@case
def r6_6_a_chained_host_handler_sees_every_signal():
    """Twenty held signals, retained as an opt-in campaign."""
    chained_host_handler(20)


SIGINFO = r"""
import ctypes, json, sys
import duckdb
libc = ctypes.CDLL(None, use_errno=True)
if sys.platform == "darwin":
    class Info(ctypes.Structure):
        _fields_ = [("signo", ctypes.c_int), ("errno", ctypes.c_int), ("code", ctypes.c_int), ("pid", ctypes.c_int)]
    class Action(ctypes.Structure):
        _fields_ = [("handler", ctypes.c_void_p), ("mask", ctypes.c_uint32), ("flags", ctypes.c_int)]
    SA_SIGINFO = 0x40
else:
    class Info(ctypes.Structure):
        _fields_ = [("signo", ctypes.c_int), ("errno", ctypes.c_int), ("code", ctypes.c_int), ("pad", ctypes.c_int), ("pid", ctypes.c_int)]
    class Action(ctypes.Structure):
        _fields_ = [("handler", ctypes.c_void_p), ("mask", ctypes.c_ulong * 16), ("flags", ctypes.c_int), ("restorer", ctypes.c_void_p)]
    SA_SIGINFO = 4
seen = []
Handler = ctypes.CFUNCTYPE(None, ctypes.c_int, ctypes.POINTER(Info), ctypes.c_void_p)
def record(number, info, context):
    seen.append([number, info.contents.signo, info.contents.pid] if info else [number, None, None])
handler = Handler(record)
action = Action()
action.handler = ctypes.cast(handler, ctypes.c_void_p).value
action.flags = SA_SIGINFO
if libc.sigaction(2, ctypes.byref(action), None) != 0:
    raise SystemExit("sigaction failed")
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute("SET enable_progress_bar = false")
con.execute(f"LOAD '{sys.argv[1]}'")
print("loaded", flush=True)
sys.stdin.readline()
print(json.dumps(seen), flush=True)
"""


@case
def r3_13_an_siginfo_host_handler_gets_the_number_and_sender():
    """A host that installs its handler with SA_SIGINFO before LOAD gets
    the real signal number and the sender's process id through the chain."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        child = subprocess.Popen(
            [sys.executable, "-c", SIGINFO, str(EXTENSION)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=child_env(backend.base(), Path(folder)),
        )
        try:
            expect(child.stdout.readline().strip(), "loaded", "the child's LOAD")
            child.send_signal(signal.SIGINT)
            time.sleep(0.3)
            child.stdin.write("\n")
            child.stdin.flush()
            expect(json.loads(child.stdout.readline()), [[2, 2, os.getpid()]], "the host handler's number, siginfo number, and sender")
            expect(child.wait(timeout=20), 0, "the child's exit")
        finally:
            if child.poll() is None:
                child.kill()


CHURN = r"""
import signal, sys, threading
import duckdb
signal.signal(signal.SIGINT, lambda number, frame: None)
con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
con.execute("SET enable_progress_bar = false")
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
            [sys.executable, "-c", CHURN, str(EXTENSION)],
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
    stress = {"r4_22_one_signal_stops_every_held_query_and_relate_asks_once",
              "r1_21_the_next_query_answers_after_a_stop",
              "r2_14_a_signal_between_queries_stops_nothing",
              "r6_6_a_chained_host_handler_sees_every_signal",
              "r5_21_ten_thousand_signals_while_four_threads_allocate"}
    select_cases(stress)
    sys.exit(main())
