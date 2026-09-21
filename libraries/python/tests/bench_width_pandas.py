"""Check 3: the width bench through four containers, one stub.

1,000 records at a 300 ms stub and jobs 32, once through a plain list,
once through an object-dtype pandas column, once through a default (Arrow-
backed) pandas column, and once through a Polars Series. Every container
must reach the same 1,000 requests and 32 in flight; the wall times are the
record the manual states the slow one and the fast one from.

Run with the stub up on 8211:
    ENGINE_BASE_URL=http://127.0.0.1:8211/v1 ENGINE_WIDTH=32 \
        .venv/bin/python tests/bench_width_pandas.py

No key, no paid call, loopback only.
"""

import json
import sys
import time
import urllib.request

import pandas as pd
import polars as pl

import thinkthen as tt

BASE = "http://127.0.0.1:8211/v1"


def stats():
    with urllib.request.urlopen(f"{BASE}/stats", timeout=5) as reply:
        return json.load(reply)


def reset():
    request = urllib.request.Request(f"{BASE}/reset", method="POST")
    with urllib.request.urlopen(request, timeout=5) as reply:
        reply.read()


def main():
    q = tt.question(decide="Does the customer ask for a refund?")
    records = [f"ticket {place}: please refund order {place}" for place in range(1000)]
    containers = [
        ("list", records),
        ("pd object", pd.Series(records, dtype=object)),
        ("pd str (arrow)", pd.Series(records)),
        ("polars", pl.Series("body", records)),
    ]

    answers = {}
    rows = []
    for label, container in containers:
        reset()
        started = time.perf_counter()
        answers[label] = tt.decide_many(q, container)
        wall = time.perf_counter() - started
        seen = stats()
        rows.append((label, wall, seen))
        print(f"{label:16} wall {wall:.3f} s  stats {seen}")

    reference = answers["list"]
    for label, wall, seen in rows:
        assert answers[label] == reference, label
        assert seen["requests"] == 1000, (label, seen)
        assert seen["max_in_flight"] == 32, (label, seen)

    slow = max(rows, key=lambda row: row[1])
    fast = min(rows, key=lambda row: row[1])
    spread = (slow[1] - fast[1]) / slow[1]
    print(f"slowest {slow[0]} {slow[1]:.3f} s | fastest {fast[0]} {fast[1]:.3f} s"
          f" | spread {spread:.2%}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
