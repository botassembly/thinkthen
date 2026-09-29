#!/usr/bin/env python3
"""The shared five-text Max corpus through SQLite's ordered warm aggregate."""

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
db.execute("SELECT thinkthen_batch('max')")
db.execute("SELECT thinkthen_max_retries(0)")
db.execute("CREATE TABLE t(i INTEGER PRIMARY KEY, body TEXT)")
db.executemany("INSERT INTO t VALUES (?, ?)", {list(enumerate(fixture['texts']))!r})
say(warm=run(db, "SELECT thinkthen_warm(?, body) FROM (SELECT body FROM t ORDER BY i)", ({fixture['question']!r},)))
"""
    held = child(code, environment(backend, "arm/full/capture"))
    expect(held["warm"], [[5]], "five accepted distinct texts")
    expect(backend.count(), 3, "three real Max requests")
    expect(sorted(backend.capture()), sorted(bodies), "three exact fixture bodies and first-seen members")
    fixed = [hashlib.sha256(f"systemone\n{fixture['url']}\n{body}".encode()).hexdigest() for body in bodies]
    expect(fixed, fixture["digests"], "independent fixed-address identities")
    expect(backend.close(), 3, "only three sends")


if __name__ == "__main__":
    sys.exit(main(globals()))
