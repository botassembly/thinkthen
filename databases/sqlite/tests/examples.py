#!/usr/bin/env python3
"""Every function's example in `examples.json`, run on the generic arm.

Each example runs in its own child with its own backend and cache, so its
answer and its usage totals are fixed. Rows print as cells joined by `|`,
one row a line, and must equal `expected` whole.
"""

from __future__ import annotations

import json
import sys

from helper import ROOT, Backend, child, environment, expect, main

EXAMPLES = json.loads((ROOT / "examples.json").read_text())["examples"]


def test_every_example_answers_as_written() -> None:
    failed = []
    for name, example in EXAMPLES.items():
        held = child(f"""
db = connect()
db.execute("SELECT thinkthen_configure(?)", (json.dumps(dict(throttle=4)),))
for statement in {example.get("setup", [])!r}:
    db.execute(statement).fetchall()
rows = run(db, {example["sql"]!r})
say(said=rows if isinstance(rows, str) else "\\n".join("|".join("" if cell is None else str(cell) for cell in row) for row in rows))
""", environment(Backend()))
        if held["said"] != example["expected"]:
            failed.append(f"{name}: said {held['said']!r}, expected {example['expected']!r}")
    expect(failed, [], "the examples")


if __name__ == "__main__":
    sys.exit(main(globals()))
