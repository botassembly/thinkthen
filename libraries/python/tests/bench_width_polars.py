"""The equality proof: the same gate through both containers.

1,000 records at a 300 ms stub and jobs 32, once through a plain list and
once through a Polars Series. The wall times must match within noise and
the stub must never see more than 32 in flight either way; that equality
is the proof the width and the scaling live in Rust, not in Python.

Run with the stub up on 8211:
    ENGINE_BASE_URL=http://127.0.0.1:8211/v1 ENGINE_WIDTH=32 \
        .venv/bin/python tests/bench_width_polars.py

BENCH_RECORDS shrinks the fixture (the gate runs 200 to keep its stub
window short); the assertions scale with it, so the proof is the same
shape at any size.

No key, no paid call, loopback only.
"""

import json
import os
import sys
import time
import urllib.request

import polars as pl

import thinkthen as tt

BASE = "http://127.0.0.1:8211/v1"
RECORDS = int(os.environ.get("BENCH_RECORDS", "1000"))


def stats():
    with urllib.request.urlopen(f"{BASE}/stats", timeout=5) as reply:
        return json.load(reply)


def reset():
    request = urllib.request.Request(f"{BASE}/reset", method="POST")
    with urllib.request.urlopen(request, timeout=5) as reply:
        reply.read()


def main():
    q = tt.question(decide="Does the customer ask for a refund?")
    records = [f"ticket {place}: please refund order {place}" for place in range(RECORDS)]
    series = pl.Series("body", records)

    reset()
    started = time.perf_counter()
    kept_list = tt.decide_many(q, records)
    list_wall = time.perf_counter() - started
    list_stats = stats()

    reset()
    started = time.perf_counter()
    kept_series = tt.decide_many(q, series)
    series_wall = time.perf_counter() - started
    series_stats = stats()

    print(f"list   wall {list_wall:.3f} s  stats {list_stats}")
    print(f"series wall {series_wall:.3f} s  stats {series_stats}")

    assert kept_list == kept_series
    assert len(kept_list) == RECORDS
    for label, seen in (("list", list_stats), ("series", series_stats)):
        assert seen["requests"] == RECORDS, (label, seen)
        assert seen["max_in_flight"] == 32, (label, seen)
    spread = abs(list_wall - series_wall) / max(list_wall, series_wall)
    print(f"spread {spread:.4%} of the slower run")
    assert spread < 0.05, (list_wall, series_wall)
    print("width equality: both containers hold the same gate")
    return 0


if __name__ == "__main__":
    sys.exit(main())
