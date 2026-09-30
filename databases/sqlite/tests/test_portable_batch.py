#!/usr/bin/env python3
"""The shared five-text Max corpus through one keyed SQLite table call."""

from __future__ import annotations

import hashlib
import json
import pathlib
import sys

from helper import Backend, child, environment, expect, main


def test_portable_questions_ride_one_request() -> None:
    """The content cut is gone, by ADR 0111, so the five texts ride one request
    that keeps the fixture's state, model and questions in order."""
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
    expect(backend.count(), 1, "one real Max request")
    sent = [json.loads(body) for body in backend.capture()]
    first = json.loads(bodies[0])
    questions = [question for body in bodies
                 for _, question in sorted(json.loads(body)["questions"].items(), key=lambda item: int(item[0][1:]))]
    expect(len(sent), 1, "one captured body")
    expect((sent[0]["state"], sent[0]["model"]), (first["state"], first["model"]), "the fixture's state and model")
    expect([sent[0]["questions"][f"q{place}"] for place in range(1, len(sent[0]["questions"]) + 1)], questions,
           "the fixture's questions in first-seen order")
    fixed = [hashlib.sha256(f"systemone\n{fixture['url']}\n{body}".encode()).hexdigest() for body in bodies]
    expect(fixed, fixture["digests"], "independent fixed-address identities")
    expect(backend.close(), 1, "only one send")


if __name__ == "__main__":
    sys.exit(main(globals()))
