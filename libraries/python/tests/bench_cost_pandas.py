"""Per-record cost through four containers, null backend.

    ENGINE_NULL=1 .venv/bin/python tests/bench_cost_pandas.py

Ten thousand records through the list, an object-dtype pandas column, a
default (Arrow-backed) pandas column, and a Polars Series, warm, reporting
microseconds a record. The engine reads ENGINE_NULL once per process, so
this arm is its own process. This is where the containers differ; on the
300 ms wire bench they are within noise.
"""

import time

import pandas as pd
import polars as pl

import thinkthen as tt


def main():
    q = tt.question(decide="Does the customer ask for a refund?")
    records = [f"ticket {place}: please refund order {place}" for place in range(10_000)]
    containers = [
        ("list", records),
        ("pd object", pd.Series(records, dtype=object)),
        ("pd str (arrow)", pd.Series(records)),
        ("polars", pl.Series("body", records)),
    ]

    results = {}
    for label, container in containers:
        tt.decide_many(q, container[:100] if not isinstance(container, list)
                       else container[:100])  # warm
        started = time.perf_counter()
        results[label] = tt.decide_many(q, container)
        us = (time.perf_counter() - started) / len(records) * 1e6
        print(f"null cost: {label:16} {us:.3f} us/record")

    reference = results["list"]
    for label, answer in results.items():
        assert answer == reference, label
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
