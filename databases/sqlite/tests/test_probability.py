#!/usr/bin/env python3
"""The installed scalar returns decide's yes probability without changing decide."""

import hashlib
import json
import pathlib
import sys

from helper import Backend, child, environment, expect, main


CASES = {case["id"]: case for case in json.loads(
    (pathlib.Path(__file__).resolve().parents[3] / "conformance/cases.json").read_text()
)["cases"]}


def test_probability_uses_the_existing_decide_request() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(value=run(db, "SELECT thinkthen_probability(?, ?, -1)",
              ("Does this need attention?", "A short note.")))
""", environment(backend, "case/02-decide-no"))
    expect(held["value"], [[0.12]], "yes probability from the exact case request")
    expect(backend.close(), 1, "one matched request")


def test_banded_unsure_still_has_a_yes_probability() -> None:
    question = json.dumps(CASES["03-decide-band-unsure"]["question"])
    backend = Backend()
    held = child(f"""
db = connect()
say(probability=run(db, "SELECT thinkthen_probability(?, ?)",
                    ({question!r}, "A short note.")))
""", environment(backend, "case/03-decide-band-unsure"))
    expect(held["probability"], [[0.2]], "banded yes probability")
    expect(backend.close(), 1, "one matched request")

    backend = Backend()
    held = child(f"""
db = connect()
say(decision=run(db, "SELECT thinkthen_decide(?, ?)",
                 ({question!r}, "A short note.")))
""", environment(backend, "case/03-decide-band-unsure"))
    expect(held["decision"], [[None]], "the banded decision remains unsure")
    expect(backend.close(), 1, "one matched decide request")


def test_plain_probability_has_the_independent_request_body_and_served_digest() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(value=run(db, "SELECT thinkthen_probability(?, ?)",
              ("Does this need attention?", "A short note.")),
    digest=run(db, "SELECT json_extract(thinkthen_details(?, ?), '$.meta.requests[0]')",
               ("Does this need attention?", "A short note.")))
""", environment(backend, "arm/full/capture"))
    expect(held["value"], [[0.9]], "full-arm yes probability")
    expected_body = CASES["02-decide-no"]["exchanges"][0]["request"]
    expect(backend.capture(), [expected_body], "complete request from the independent fixture")
    address = backend.base("arm/full/capture") + "/systemone"
    expected_digest = hashlib.sha256(
        b"systemone\n" + address.encode() + b"\n" + expected_body.encode()
    ).hexdigest()
    expect(held["digest"], [[expected_digest]], "served-URL digest")
    expect(backend.close(), 1, "details reuses the probability request")


def test_probability_keeps_deadline_and_final_context_slots() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(value=run(db, "SELECT thinkthen_probability(?, ?, -1, ?)",
              ("Does this need attention?", "A short note.", "Use this policy note.")))
""", environment(backend, "arm/full"))
    expect(held["value"], [[0.9]], "final context after the deadline")
    expect(backend.close(), 1, "one contextual request")


def test_wrong_kind_and_spent_deadline_do_not_send() -> None:
    backend = Backend()
    choose = json.dumps(CASES["06-choose-billing"]["question"])
    held = child(f"""
db = connect()
say(wrong=run(db, "SELECT thinkthen_probability(?, ?)",
              ({choose!r}, "Route this note.")),
    spent=run(db, "SELECT thinkthen_probability(?, ?, 0)",
              ("Does this need attention?", "A short note.")),
    null=run(db, "SELECT thinkthen_probability(NULL, 'A short note.')"))
""", environment(backend))
    expect(held["wrong"].startswith("thinkthen usage: "), True, "wrong kind")
    expect(held["spent"].startswith("thinkthen deadline: "), True, "spent budget")
    expect(held["null"], [[None]], "NULL question")
    expect(backend.close(), 0, "refused inputs")


if __name__ == "__main__":
    sys.exit(main(globals()))
