#!/usr/bin/env python3
"""The shared Max corpus through PostgreSQL's keyed JSONB result."""

from __future__ import annotations

import hashlib
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from portable import one_portable_request  # noqa: E402

FOLDER = pathlib.Path(__file__).resolve().parents[3] / "specification" / "fixtures" / "batching"
FIXTURE = json.loads((FOLDER / "portable-records.json").read_text())


def quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def query() -> str:
    keyed = {str(index): value for index, value in enumerate(FIXTURE["texts"])}
    return ("SELECT key || ':' || coalesce(value::text, 'null') "
            f"FROM thinkthen_decide_many({quote(FIXTURE['question'])}, "
            f"{quote(json.dumps(keyed, ensure_ascii=False))}::jsonb) ORDER BY key")


def verify(rows: str, count: str, capture: str) -> None:
    assert FIXTURE["schema"] == "thinkthen.portable-batch-records/1"
    assert rows.splitlines() == [f"{index}:true" for index in range(5)], rows
    # The content cut is gone (ADR 0111), so the five texts ride one request.
    assert count == "1", count
    bodies = [(FOLDER / f"portable-{number}.request.json").read_text().removesuffix("\n") for number in range(1, 4)]
    observed = json.loads(pathlib.Path(capture).read_text())
    one_portable_request(observed["bodies"])
    fixed = [hashlib.sha256(f"systemone\n{FIXTURE['url']}\n{body}".encode()).hexdigest() for body in bodies]
    assert fixed == FIXTURE["digests"], (fixed, FIXTURE["digests"])


if __name__ == "__main__":
    if sys.argv[1] == "sql":
        print(query())
    elif sys.argv[1] == "verify":
        verify(*sys.argv[2:])
    else:
        raise SystemExit("portable_batch.py takes sql or verify")
