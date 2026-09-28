#!/usr/bin/env python3
"""Interrupts reach every call that can send (ticket 0109 decision 6).

Each held test sends its interrupt only once the backend's count reads the
stated number, and arms a release 2 s after the signal, so a planted run
fails on its assertion instead of waiting out the request timeout.

These tests also guard the hand-extended API table in `src/ffi.rs`. The
rusqlite loadable bindings end at SQLite 3.34, so `ApiRoutines` adds the
tail pointers up to `is_interrupted`, which 3.41 added. Ian ruled on
2026-09-25 to keep that table and ask no upstream fix. Ticket 0127 planted a
tail of 12 and of 14 in place of 13: each held test here then read
`interrupted` in place of the cancelled sentence, and each plant failed.
"""

from __future__ import annotations

import os
import signal
import subprocess
import sys
import threading
import time

from helper import CLI, LIB, Backend, Child, environment, expect, main

CANCELLED = "thinkthen cancelled: the call was cancelled"

# The child runs `sql` on `db` and interrupts `target` when the parent says
# go. It reports the error and the time from the interrupt to the return,
# then stays alive until its input ends, so a detached worker could still send.
HELD = """
db = connect()
target = db
{setup}
stopped = []
def stop():
    sys.stdin.readline()
    stopped.append(time.monotonic())
    target.interrupt()
threading.Thread(target=stop, daemon=True).start()
error = run(db, {sql!r})
say(error=error, after=time.monotonic() - stopped[0] if stopped else None)
sys.stdin.readline()
"""


def interrupted(backend: Backend, sql: str, sends: int, setup: str = "") -> dict:
    """Run a held call, interrupt it once `sends` are counted, and read its result."""
    held = Child(HELD.format(sql=sql, setup=setup), environment(backend, "arm/held"))
    expect(backend.wait(sends), sends, "sends before the interrupt")
    held.send()
    release = threading.Timer(2, backend.release)
    release.start()
    result = held.read()
    release.cancel()
    return result | {"child": held}


def stopped_fast(result: dict) -> None:
    expect(result["error"], CANCELLED, "the error")
    expect(result["after"] is not None and result["after"] < 0.1, True, f"returned {result['after']} s after the interrupt")


def settled(backend: Backend, sends: int) -> None:
    """After a release, no detached worker starts another send."""
    time.sleep(0.3)
    backend.release()
    time.sleep(0.3)
    expect(backend.count(), sends, "sends after the release")


def test_a_single_call_is_cancelled_while_its_send_is_held() -> None:
    """R2-24: the interrupt returns within 100 ms, and the worker sends no more."""
    backend = Backend()
    result = interrupted(backend, "SELECT thinkthen_decide('Is it red?', 'a red door')", 1)
    stopped_fast(result)
    settled(backend, 1)


def test_the_second_connection_hears_its_own_interrupt() -> None:
    """R1-14: the interrupt is read from the calling connection, the first one closed."""
    backend = Backend()
    setup = "first = db\ndb = connect()\ntarget = db\nfirst.close()"
    result = interrupted(backend, "SELECT thinkthen_decide('Is it red?', 'a red door')", 1, setup)
    stopped_fast(result)


def test_recognize_is_cancelled_while_its_send_is_held() -> None:
    """R4-17 recognize half."""
    backend = Backend()
    sql = "SELECT * FROM thinkthen_recognize('Maria Chen joined Northwind Freight.', 'person,organization')"
    stopped_fast(interrupted(backend, sql, 1))


def test_relate_is_cancelled_and_sends_no_more_after_release() -> None:
    """R4-17 relate half: 21 persons make 420 pairs in two requests.

    Throttle 1 keeps the second request waiting behind the first.
    """
    backend = Backend()
    setup = """db.execute("SELECT thinkthen_throttle(1)")
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", [(n, f'Person {n}', 'person') for n in range(21)])"""
    sql = "SELECT * FROM thinkthen_relate('e', 'id', 'name', 'kind', 'knows=person:person')"
    stopped_fast(interrupted(backend, sql, 1, setup))
    settled(backend, 1)


def test_a_warm_is_cancelled_mid_batch() -> None:
    """Case 18: at throttle 8, a held 20-row warm stops at 8 sends and stays there."""
    backend = Backend()
    setup = """db.execute("SELECT thinkthen_throttle(8)")
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(20)])"""
    result = interrupted(backend, "SELECT thinkthen_warm('Is it red?', body) FROM t", 8, setup)
    stopped_fast(result)
    time.sleep(0.3)
    expect(backend.count(), 8, "sends 300 ms after the interrupt")
    settled(backend, 8)


def test_the_cli_prints_the_cancelled_sentence_on_sigint() -> None:
    """R2-24: SIGINT in the CLI built from the amalgamation interrupts a held call."""
    backend = Backend()
    env = environment(backend, "arm/held")
    cli = subprocess.Popen(
        [CLI, ":memory:", f".load {LIB}", "SELECT thinkthen_decide('Is it red?', 'a red door');"],
        env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    expect(backend.wait(1), 1, "sends before the signal")
    release = threading.Timer(2, backend.release)
    release.start()
    signalled = time.monotonic()
    cli.send_signal(signal.SIGINT)
    assert cli.stderr is not None
    line = cli.stderr.readline()
    after = time.monotonic() - signalled
    release.cancel()
    cli.kill()
    expect(CANCELLED in line, True, f"the CLI printed {line!r}")
    expect(after < 0.1, True, f"the CLI printed {after} s after the signal")


def test_a_fast_warm_stops_soon_after_the_interrupt() -> None:
    """On a fast backend SQLite's own step loop may stop first, so only the time is held."""
    backend = Backend()
    code = """
db = connect()
db.execute("SELECT thinkthen_throttle(8)")
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(100000)])
def stop():
    time.sleep(0.5)
    stopped.append(time.monotonic())
    db.interrupt()
stopped = []
threading.Thread(target=stop, daemon=True).start()
error = run(db, "SELECT thinkthen_warm('Is it red?', body) FROM t")
say(error=isinstance(error, str), after=time.monotonic() - stopped[0])
"""
    result = Child(code, environment(backend)).result()
    expect(result["error"], True, "the warm ended in an error")
    expect(result["after"] < 1, True, f"the warm stopped {result['after']} s after the interrupt")


def cached_calls(count: int) -> dict:
    """Count cache hits and remaining threads after one child finishes."""
    backend = Backend()
    code = """
db = connect()
db.execute("SELECT thinkthen_decide('Is it red?', 'a red door')")
threads = lambda: int([line for line in open('/proc/self/status') if line.startswith('Threads:')][0].split()[1])
before, started = threads(), time.monotonic()
for _ in range(int(os.environ["COUNT"])):
    db.execute("SELECT thinkthen_decide('Is it red?', 'a red door')").fetchall()
took = time.monotonic() - started
settled = time.monotonic() + 1
while threads() > before and time.monotonic() < settled:
    time.sleep(0.01)
say(took=took, before=before, after=threads())
"""
    env = environment(backend)
    env["COUNT"] = str(count)
    result = Child(code, env).result()
    expect(backend.close(), 1, "sends")
    return result


def test_one_cached_call_leaves_no_thread() -> None:
    result = cached_calls(3)
    expect(result["after"], result["before"], "threads after the cached calls")


def test_cached_calls_return_at_once_and_leave_no_thread() -> None:
    """R3-22: 200 cached answers take under 2 s, and within 1 s no thread outlives them."""
    result = cached_calls(200)
    expect(result["took"] < 2, True, f"200 cached calls took {result['took']} s")
    expect(result["after"], result["before"], "threads after the calls")


FORKED = """
import os
db = connect()
db.execute("SELECT thinkthen_decide('Is it red?', 'a red door')")
reading, writing = os.pipe()
depth = {depth}
def held():
    signal_read, signal_write = os.pipe()
    pid = os.fork()
    if pid:
        return pid, signal_write
    child = connect()
    stopped = []
    def stop():
        os.read(signal_read, 1)
        stopped.append(time.monotonic())
        child.interrupt()
    threading.Thread(target=stop, daemon=True).start()
    error = run(child, "SELECT thinkthen_decide('Is it red?', 'a blue door')")
    os.write(writing, json.dumps([error, time.monotonic() - stopped[0] if stopped else None]).encode())
    os._exit(0)
if depth == 1:
    pid, go = held()
else:
    pid = os.fork()
    if pid == 0:
        grandchild, go = held()
        sys.stdin.readline()
        os.write(go, b"x")
        os.waitpid(grandchild, 0)
        os._exit(0)
if depth == 1:
    sys.stdin.readline()
    os.write(go, b"x")
signal.alarm(20)
os.waitpid(pid, 0)
error, after = json.loads(os.read(reading, 65536))
say(error=error, after=after)
"""


def forked(depth: int) -> None:
    backend = Backend()
    held = Child("import signal\n" + FORKED.format(depth=depth), environment(backend, "arm/held"))
    expect(backend.wait(1), 1, "the parent's call is held")
    backend.round()
    expect(backend.wait(2), 2, "the forked call is held")
    held.send()
    release = threading.Timer(2, backend.release)
    release.start()
    result = held.result()
    release.cancel()
    stopped_fast(result)


def test_a_forked_child_hears_its_own_interrupt() -> None:
    """R4-21: a child forked after a call interrupts its own held call."""
    forked(1)


def test_a_grandchild_hears_its_own_interrupt() -> None:
    """R6-9: the same, one generation down."""
    forked(2)


if __name__ == "__main__":
    stress = {"test_a_fast_warm_stops_soon_after_the_interrupt",
              "test_cached_calls_return_at_once_and_leave_no_thread"}
    only_stress = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"
    sys.exit(main({name: value for name, value in globals().items()
                   if name.startswith("test_") and (name in stress) == only_stress}))
