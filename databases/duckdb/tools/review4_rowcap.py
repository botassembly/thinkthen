#!/usr/bin/env python3
"""Review 4, item 8: the row cap runs as a count first, then the real
query executes again.

The reviewer's bypass: range(3 + (random()<0.5)::INT*8000000) counted 3
and then materialized eight million rows (2.35 GB, 4.5 s) before the
cap refused — the count and the run are two executions, so any query
whose rows change between them slips past.

The coin must flip for the bypass to show, so the probe runs twelve
trials and FAILs if ANY trial is slow: on the reviewed code roughly a
quarter of trials count 3 and then materialize the big side; on the fix
the single execution is LIMIT-bounded and no trial can be slow.
"""

from __future__ import annotations

import sys
import time

from review4_lib import Harness, finish, verdict

TRIALS = 12
SLOW_SECONDS = 1.5


def main() -> int:
    # Stage markers: this ctypes harness is layout-sensitive at its
    # buffer boundary; the flushed markers keep the crash (when it
    # appears) diagnosable to a stage.
    print("probe start", flush=True)
    harness = Harness()
    print("harness up", flush=True)
    d = harness.open()
    c = harness.connect(d)
    harness.load(c)
    query = "SELECT i, ''refund please'' FROM range(3 + (random()<0.5)::INT*8000000) t(i)"
    worst = 0.0
    print("loop start", flush=True)
    outcomes = []
    for trial in range(TRIALS):
        start = time.monotonic()
        ok, error, rows = harness.run(
            c, f"SELECT count(*) FROM thinkthen_relate('{query}', ['caused_by'])"
        )
        elapsed = time.monotonic() - start
        worst = max(worst, elapsed)
        outcomes.append((ok, elapsed, error[:90]))
    slow = [(n, o) for n, o in enumerate(outcomes) if o[1] > SLOW_SECONDS]
    # The trials report once, through the verdict: per-trial prints in
    # this ctypes harness crash at the interpreter's buffer boundary
    # (observed as a segfault inside print), and the one summary is the
    # evidence anyway.
    detail = "; ".join(
        f"trial {n + 1}: {'ok' if ok else 'refused: ' + why[:60]} in {took:.2f}s"
        for n, (ok, took, why) in enumerate(outcomes)
    )
    return verdict(
        f"rowcap: no trial materializes past the cap (worst {worst:.2f}s of {TRIALS})",
        not slow,
        detail + f" — {len(slow)} slow trial(s)",
    )


if __name__ == "__main__":
    finish(main())
