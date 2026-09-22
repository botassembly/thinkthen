#!/usr/bin/env python3
"""One interrupt stops one query, the next query still answers, and an
interrupt that reaches no call poisons nothing.

Before the first fix the surface carried one process-wide cancel token,
and a one-shot token is spent forever: after one Ctrl-C every later call
in that process returned `cancelled` without asking anything. The second
review found the surviving half: an interrupt landing near an engine call
— just after one returned — still cancelled the next query, four runs in
five. This suite pins all three behaviors: a long query stops on SIGINT,
the query after it answers, a signal that lands while nothing runs leaves
the next query alone (ten rounds), and DuckDB's own `interrupt()` ends a
query without poisoning the surface. The venv carries duckdb 1.5.5, the
CLI's version.

DuckDB's own cancel reaches the query, not the engine call inside it:
the C API exposes no hook that lets a table or scalar function observe
the caller's interrupt (the extension API's function-info accessors are
`extra_info`, `bind_data`, `init_data`, and `local_init_data`; the
client-context accessors are catalog, config, file system, and
connection id), so the engine call itself stops on the SIGINT token and
on deadlines, and DuckDB's interrupt ends the query at its next boundary.
The boundary is pinned in NOTES.md.
"""

from __future__ import annotations

import os
import signal
import sys
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
SLOW = (
    "SELECT count(thinkthen_decide('Is this a complaint?', 'i want a refund now'))"
    " FROM range(3000000)"
)
FRESH = "SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today')"


def interrupt_after(seconds: float) -> threading.Timer:
    timer = threading.Timer(seconds, os.kill, args=(os.getpid(), signal.SIGINT))
    timer.daemon = True
    timer.start()
    return timer


def main() -> int:
    os.environ["ENGINE_NULL"] = "1"
    import duckdb  # noqa: PLC0415 - imported after the environment is set

    con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    con.execute(f"LOAD '{EXTENSION}'")

    interrupted = False
    timer = interrupt_after(1.0)
    try:
        con.execute(SLOW).fetchall()
        print("FAILED   the long query ran to completion despite the interrupt")
        timer.cancel()
        return 1
    except BaseException as error:  # noqa: BLE001 - the interrupt's own kind
        interrupted = True
        try:
            print(f"the interrupted call ended with {type(error).__name__}: {str(error)[:90]}")
        except BaseException:  # noqa: BLE001 - a pending interrupt may land in the print
            pass
    finally:
        timer.cancel()
    # A KeyboardInterrupt can still be pending after the C call returns.
    try:
        time.sleep(0.2)
    except KeyboardInterrupt:
        pass
    if not interrupted:
        print("FAILED   the long query was not interrupted")
        return 1

    try:
        fresh = con.execute(FRESH).fetchone()
    except BaseException as error:  # noqa: BLE001 - the poisoned token's own error
        print(
            "FAILED   the query after the interrupt errored, so the interrupt poisoned "
            f"the process: {type(error).__name__}: {str(error)[:90]}"
        )
        return 1
    if fresh != (True,):
        print(f"FAILED   the query after the interrupt must answer, got {fresh!r}")
        return 1
    print("ok       the interrupt stopped one query and the next query answered")

    # A signal that lands after a lone query's last call is the host's own
    # gesture: it must not cancel the next query. The signal is sent right
    # after a query returns, ten rounds, and every next query must answer.
    # The rounds are spaced past the surface's burst window (10 ms): two
    # call starts closer than that are a query still producing calls, and
    # a signal there stops that query — the chain arm of host_signal.py
    # proves that half. What no call-pattern rule can tell apart is a
    # signal in the gap between a chunked query's last call and its
    # return; the C API carries no per-query hook for scalar functions,
    # and NOTES.md pins that boundary.
    poisoned = []
    for round_number in range(10):
        time.sleep(0.03)
        con.execute(FRESH).fetchone()
        # The chained handler raises Python's own interrupt; consume it so
        # the next statement runs and its own answer is what is read.
        try:
            os.kill(os.getpid(), signal.SIGINT)
            time.sleep(0.02)
        except KeyboardInterrupt:
            pass
        try:
            answer = con.execute(FRESH).fetchone()
        except BaseException as error:  # noqa: BLE001 - the poisoned token's own error
            poisoned.append(f"round {round_number}: {type(error).__name__}: {str(error)[:60]}")
            break
        if answer != (True,):
            poisoned.append(f"round {round_number}: got {answer!r}")
            break
    if poisoned:
        print(f"FAILED   an interrupt near a call poisoned the next query: {poisoned[0]}")
        return 1
    print("ok       ten interrupts near calls left the next query answering")

    # DuckDB's own cancel, from another thread: the query ends, and the
    # surface stays clean for the next one.
    duckdb_interrupted = threading.Timer(0.3, con.interrupt)
    duckdb_interrupted.daemon = True
    duckdb_interrupted.start()
    try:
        con.execute(SLOW).fetchall()
        print("FAILED   DuckDB's own interrupt did not end the query")
        duckdb_interrupted.cancel()
        return 1
    except BaseException as error:  # noqa: BLE001 - the interruption's own kind
        ended = type(error).__name__
    finally:
        duckdb_interrupted.cancel()
    try:
        answer = con.execute(FRESH).fetchone()
    except BaseException as error:  # noqa: BLE001 - the poisoned token's own error
        print(f"FAILED   DuckDB's interrupt poisoned the next query: {type(error).__name__}")
        return 1
    if answer != (True,):
        print(f"FAILED   the query after DuckDB's interrupt must answer, got {answer!r}")
        return 1
    print(f"ok       DuckDB's own interrupt ended the query ({ended}) and the next answered")
    return 0


if __name__ == "__main__":
    sys.exit(main())
