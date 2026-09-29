#!/usr/bin/env python3
"""The shared five-text Max corpus through one keyed SQLite table call."""

from __future__ import annotations

import hashlib
import json
import pathlib
import sys

from helper import Backend, child, environment, expect, main


def test_portable_batch_identity() -> None:
    folder = pathlib.Path(__file__).resolve().parents[3] / "specification" / "fixtures" / "batching"
    fixture = json.loads((folder / "portable-records.json").read_text())
    bodies = [(folder / f"portable-{number}.request.json").read_text().removesuffix("\n") for number in range(1, 4)]
    expect(fixture["schema"], "thinkthen.portable-batch-records/1", "shared corpus version")
    backend = Backend()
    code = f"""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{{"batch":"max","max_retries":0}}',))
say(rows=run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)",
             ({fixture['question']!r}, {json.dumps({str(i): text for i, text in enumerate(fixture['texts'])})!r})))
"""
    held = child(code, environment(backend, "arm/full/capture"))
    expect(held["rows"], [[5]], "five accepted distinct texts")
    expect(backend.count(), 3, "three real Max requests")
    expect(sorted(backend.capture()), sorted(bodies), "three exact fixture bodies and first-seen members")
    fixed = [hashlib.sha256(f"systemone\n{fixture['url']}\n{body}".encode()).hexdigest() for body in bodies]
    expect(fixed, fixture["digests"], "independent fixed-address identities")
    expect(backend.close(), 3, "only three sends")


if __name__ == "__main__":
    sys.exit(main(globals()))
