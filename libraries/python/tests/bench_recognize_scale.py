"""`recognize` and `relate` at the scale their doors support, offline.

The bulk verbs prove equality at the 300 ms width bench over the wire;
these two functions read the recordings, so no wire is involved. What is
proven here instead: the frame door over a thousand rows gives exactly
the same answers as a thousand single calls, with the wall times beside
each other, so the long-frame path adds no Python-side work per row; and
the relate frame door gives exactly the list door's edges on the one
recorded set both doors can serve.

The stand-in answers only recorded texts, so the thousand rows all hold
the reference sentence the recordings carry.
"""

import sys
import time

import polars as pl

import thinkthen as tt

TEXT = "Maria Chen joined Northwind Freight in Chicago last spring."
ALERTS = [
    "Checkout returns 500 at the payment step.",
    "Card charges are failing for every customer.",
    "The nightly export ran two hours late.",
    "The payments database ran out of disk space.",
]


def main():
    rows = 1000
    frame = pl.DataFrame({"body": [TEXT] * rows})

    started = time.perf_counter()
    long = tt.recognize(frame, on="body")
    frame_wall = time.perf_counter() - started

    started = time.perf_counter()
    per_row = []
    for row in range(rows):
        found = tt.recognize(TEXT)
        per_row.extend(
            (row + 1, entity.text, entity.kind, entity.start, entity.end, entity.strength)
            for entity in found.entities
        )
    per_row_wall = time.perf_counter() - started

    frame_rows = [
        (row["row"], row["text"], row["kind"], row["start"], row["end"], row["strength"])
        for row in long.iter_rows(named=True)
    ]
    print(f"frame:   {rows} rows, one call,  wall {frame_wall:.3f} s, {len(frame_rows)} name rows")
    print(f"per-row: {rows} calls, one each, wall {per_row_wall:.3f} s, {len(per_row)} name rows")
    assert frame_rows == per_row
    print("identical answers: True")

    list_edges = tt.relate(ALERTS, relations=["caused_by"], either=["same_as"], threshold=0.9)
    frame_edges = tt.relate(pl.DataFrame({"body": ALERTS}), on="body",
                            relations=["caused_by"], either=["same_as"], threshold=0.9)
    listed = [(edge.name, edge.source, edge.target, edge.probability) for edge in list_edges]
    framed = [(edge["name"], edge["source"], edge["target"], edge["probability"])
              for edge in frame_edges.iter_rows(named=True)]
    print(f"relate list door:  {listed}")
    print(f"relate frame door: {framed}")
    assert listed == framed
    print("relate doors agree: True")
    return 0


if __name__ == "__main__":
    sys.exit(main())
