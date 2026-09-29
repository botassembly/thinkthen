"""The shared five-text Max batch through DuckDB's public vector details route."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from harness import Backend, expect, rows, run


def portable_batch_identity() -> None:
    folder = Path(__file__).resolve().parents[3] / "specification" / "fixtures" / "batching"
    fixture = json.loads((folder / "portable-records.json").read_text())
    bodies = [(folder / f"portable-{number}.request.json").read_text().removesuffix("\n") for number in range(1, 4)]
    assert fixture["schema"] == "thinkthen.portable-batch-records/1"
    quote = lambda value: "'" + value.replace("'", "''") + "'"
    values = ",".join(f"({index},{quote(value)})" for index, value in enumerate(fixture["texts"]))
    query = (f"SELECT thinkthen_try_details({quote(fixture['question'])}, x) "
             f"FROM (VALUES {values}) t(i,x) ORDER BY i")
    with Backend() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = 'max'", query],
                  backend.base("arm/full/capture"))
        answers = [json.loads(value) for value, in rows(got[2])]
        expect([answer["status"] for answer in answers], ["answered"] * 5, "five ordered SQL answers")
        expect([answer["details"]["value"] for answer in answers], [True] * 5, "five decide values")
        expect(backend.count(), 3, "three real Max requests")
        expect(sorted(backend.capture()), sorted(bodies), "three exact fixture bodies")
        address = backend.base("arm/full/capture") + "/systemone"
        digests = [hashlib.sha256(f"systemone\n{address}\n{body}".encode()).hexdigest() for body in bodies]
        expect([answer["details"]["meta"]["requests"] for answer in answers],
               [[digests[part]] for part in (0, 0, 1, 1, 2)], "returned packed request identities")
        fixed = [hashlib.sha256(f"systemone\n{fixture['url']}\n{body}".encode()).hexdigest() for body in bodies]
        expect(fixed, fixture["digests"], "independent fixed-address identities")
