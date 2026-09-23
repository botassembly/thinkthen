#!/usr/bin/env python3
"""Review 4, item 13: one Ctrl-C with two queries running stops both,
and a slow relate query is reachable by the interrupt at all.

Two connections each run a relate whose query is slow per row (a heavy
scalar subquery over few rows, so the record cap does not bound the
work). One SIGINT to the process must stop BOTH relates: the engine's
cancel token stops the asks, and the interrupt bridge stops the kept
connections' running queries.

PASS: both relate calls return (with an error) well inside the budget,
and neither keeps running past the signal. The C-API boundary — a
caller's own `con.interrupt()` cannot cross into the kept connection's
query beyond what the cap already bounds — is recorded in NOTES, not
papered over here.
"""

from __future__ import annotations

import os
import signal
import threading
import time

from review4_lib import Harness, finish, verdict

BUDGET_SECONDS = 6.0
SLOW_ROWS = 3


def main() -> int:
    harness = Harness()
    d = harness.open()
    connections = [harness.connect(d) for _ in range(2)]
    harness.load(connections[0])
    harness.load(connections[1])
    # Three rows whose per-row work is a 200M-element sum, so the
    # record cap does not bound the runtime and the interrupt has a
    # window to land mid-query.
    query = (
        "SELECT i, ''refund please'' FROM range(3) t(i), "
        "LATERAL (SELECT sum(w * w) FROM range(200000000) heavy(w)) slow(s)"
    )
    outcomes: list[tuple[float, str]] = []
    lock = threading.Lock()

    def relate(index: int) -> None:
        start = time.monotonic()
        ok, error, rows = harness.run(
            connections[index],
            f"SELECT count(*) FROM thinkthen_relate('{query}', ['caused_by'])",
        )
        elapsed = time.monotonic() - start
        with lock:
            outcomes.append(
                (elapsed, "ok" if ok else error.replace("\n", " ")[:70])
            )
        print(
            f"       relate {index + 1}: "
            + ("ok" if ok else error.replace("\n", " ")[:70])
            + f" in {elapsed:.2f}s",
            flush=True,
        )

    threads = [
        threading.Thread(target=relate, args=(index,), daemon=True)
        for index in range(2)
    ]
    # The main thread blocks SIGINT so the host's chained handler (which
    # raises KeyboardInterrupt in whatever thread it lands in) cannot
    # unwind this probe's own driver; the extension's handler runs
    # either way, because it is the process's disposition.
    signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGINT})
    for thread in threads:
        thread.start()
    time.sleep(1.0)  # both relates are inside their queries now
    os.kill(os.getpid(), signal.SIGINT)
    # Python delivers its own chained raise to THIS thread at the next
    # bytecode whatever mask is set (the handler ran on whichever thread
    # was unblocked), so the driver simply absorbs it: the probe's own
    # question is whether the RELATES stop, and they answer that.
    try:
        for thread in threads:
            thread.join(timeout=BUDGET_SECONDS)
    except KeyboardInterrupt:
        for thread in threads:
            thread.join(timeout=BUDGET_SECONDS)
    try:
        live = [i for i, thread in enumerate(threads) if thread.is_alive()]
        detail = "; ".join(
            f"relate {n + 1}: {why} in {took:.2f}s"
            for n, (took, why) in enumerate(outcomes)
        )
    except KeyboardInterrupt:
        live = [i for i, thread in enumerate(threads) if thread.is_alive()]
        detail = "the driver absorbed the host's own raise"
    return verdict(
        "cancel: one SIGINT stops both running relates",
        not live and len(outcomes) == 2,
        detail or "neither relate returned before the join timed out",
    )


if __name__ == "__main__":
    finish(main())
