"""Per-record cost through both containers, null backend.

    ENGINE_NULL=1 .venv/bin/python tests/bench_cost_polars.py

Ten thousand records through the list and through a Polars Series, warm,
reporting microseconds a record. The engine reads ENGINE_NULL once per
process, so this arm is its own process.
"""

import time

import polars as pl

import thinkthen as tt


def main():
    q = tt.question(decide="Does the customer ask for a refund?")
    records = [f"ticket {place}: please refund order {place}" for place in range(10_000)]
    series = pl.Series("body", records)

    tt.decide_many(q, records[:100])  # warm

    started = time.perf_counter()
    a = tt.decide_many(q, records)
    list_us = (time.perf_counter() - started) / len(records) * 1e6

    started = time.perf_counter()
    b = tt.decide_many(q, series)
    series_us = (time.perf_counter() - started) / len(records) * 1e6

    assert a == b
    print(f"null cost: list {list_us:.3f} us/record, series {series_us:.3f} us/record")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
