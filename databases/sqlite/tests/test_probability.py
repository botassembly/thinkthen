#!/usr/bin/env python3
"""Probability travels with keyed decide and choose rows, including unsure."""

import json
import pathlib
import sys

from helper import Backend, child, environment, expect, main

CASES = {case["id"]: case for case in json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance/cases.json").read_text()
)["cases"]}


def test_banded_unsure_keeps_yes_probability_on_its_row() -> None:
    case = CASES["03-decide-band-unsure"]
    backend = Backend()
    held = child(f"""
db = connect()
say(rows=run(db, "SELECT key,value,probability FROM thinkthen_decide_many(?,?)",
             ({json.dumps(case['question'])!r}, {json.dumps({'7': case['exchanges'][0]['evidence']})!r})))
""", environment(backend, "case/03-decide-band-unsure"))
    expect(held["rows"], [["7", None, 0.2]], "unsure and yes probability from one reply")
    expect(backend.close(), 1, "one send")


def test_choose_probability_is_the_winners_probability() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(rows=run(db, "SELECT key,value,probability FROM thinkthen_choose_many(?,?,?)",
             ("Which team?", '{"7":"A short note."}', '{"options":["billing","shipping"]}')))
""", environment(backend, "generic"))
    expect(held["rows"], [["7", "billing", 0.9]], "selected label and probability")
    expect(backend.close(), 1, "one packed send")


def test_old_scalar_probability_refuses_without_a_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(old=run(db, "SELECT thinkthen_probability('q','e')"), settings=run(db, "SELECT thinkthen_probability('q','e','{}')"))
""", environment(backend))
    sentence = "thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank"
    expect((held["old"], held["settings"]), (sentence, sentence), "old scalar with two and three arguments")
    expect(backend.close(), 0, "no old scalar send")


if __name__ == "__main__":
    raise SystemExit(main(globals()))
