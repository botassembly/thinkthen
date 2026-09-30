"""The shared five-text Max batch through DuckDB's public vector details route."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from harness import Backend, expect, rows, run
from portable import one_portable_request, question_keys


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
        # ADR 0111 drops the content cut, so the five texts ride one request.
        expect(backend.count(), 1, "one real Max request")
        captured = backend.capture()
        one_portable_request(captured)
        address = backend.base("arm/full/capture") + "/systemone"
        keys = question_keys(address, captured[0])
        expect([answer["details"]["meta"]["requests"] for answer in answers],
               [[key] for key in keys], "each row's own question key")
        fixed = [hashlib.sha256(f"systemone\n{fixture['url']}\n{body}".encode()).hexdigest() for body in bodies]
        expect(fixed, fixture["digests"], "independent fixed-address identities")
