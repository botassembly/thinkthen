"""Run main's shared conformance cases through the extension.

Each success case runs on its own `/case/ID/v1` arm, so the backend answers
only the case's exact request bytes and any drift reads status 500. Each
case reports pass, fail, or not run with a reason from `NOT_RUN`, and the
three counts must sum to the cases in `cases.json` (R3-30). Any failure,
or a case with no runner and no reason, exits nonzero (R1-2, R5-31).

`THINKTHEN_CONFORMANCE_CASES` names another cases file; the self-test
plants a wrong answer through it.
"""

from __future__ import annotations

import json
import os
import sys
import tempfile
from pathlib import Path

from harness import HOOKS, ROOT, Backend, rows, run, said
from signal_suite import CANCELLED, held_cancel

CASES = Path(os.environ.get("THINKTHEN_CONFORMANCE_CASES", ROOT.parent.parent / "conformance" / "cases.json"))

# The one closed list of reasons a case does not run here.
NOT_RUN = {
    "relate": "relate lands in 0118",
    "find": "SQL finds with ORDER BY and LIMIT over decide, so the find wire question never goes out",
    "on": "main's public API refuses `on` in a library question set, so SQL passes each record whole",
}


def reason(case: dict) -> str | None:
    if case["verb"] in ("relate", "find"):
        return NOT_RUN[case["verb"]]
    members = case.get("question_set", {}).get("questions", {}).values()
    return NOT_RUN["on"] if any("on" in member for member in members) else None


def quoted(text: str) -> str:
    return "'" + text.replace("'", "''") + "'"


def values(texts: list[str]) -> str:
    return "(VALUES " + ", ".join(f"({index}, {quoted(text)})" for index, text in enumerate(texts)) + ") t(i, x)"


def expected(case: dict) -> list:
    return [answer["bare"] for answer in case["expect"]["success"]["answers"]]


def evidence(case: dict) -> list[str]:
    return [exchange["evidence"] for exchange in case["exchanges"]]


def single(case: dict, base: str) -> list:
    """decide, choose, tag, and score: the value `thinkthen_details` reads."""
    question = quoted(json.dumps(case["question"]))
    table = values(evidence(case))
    got = run([f"SELECT (thinkthen_details({question}, x)).value FROM {table} ORDER BY i"], base)
    answers = [json.loads(value) for (value,) in rows(got[0])]
    words = {"yes": True, "no": False, "unsure": None}
    return [words.get(answer, answer) if isinstance(answer, str) else answer for answer in answers]


def decide(case: dict, base: str) -> list:
    got = run([f"SELECT thinkthen_decide({quoted(json.dumps(case['question']))}, x) FROM {values(evidence(case))} ORDER BY i"], base)
    return [value for (value,) in rows(got[0])]


def filtered(case: dict, base: str) -> list:
    table = values(evidence(case)) if case["exchanges"] else "(SELECT 0 AS i, 'none' AS x WHERE false) t"
    got = run([f"SELECT i FROM {table} WHERE thinkthen_decide({quoted(json.dumps(case['question']))}, x) ORDER BY i"], base)
    return [index for (index,) in rows(got[0])]


def ranked(case: dict, base: str) -> list:
    got = run(
        [f"SELECT i, thinkthen_probability({quoted(json.dumps(case['question']))}, x) AS p FROM {values(evidence(case))} ORDER BY p DESC, i"],
        base,
    )
    return [{"index": index, "probability": probability} for index, probability in rows(got[0])]


def record(case: dict) -> list[str]:
    """The records an annotate case reads: one per exchange, or one JSON
    record whose `on` pointers select each group's evidence."""
    members = case["question_set"]["questions"]
    pointers = [member.get("on") for member in members.values()]
    if not any(pointers):
        return list(dict.fromkeys(evidence(case)))
    return [json.dumps({pointer.lstrip("/"): text for pointer, text in zip(pointers, evidence(case), strict=True)})]


def annotated(case: dict, base: str) -> list:
    got = run(
        [f"SELECT thinkthen_annotate({quoted(json.dumps(case['question_set']))}, x) FROM {values(record(case))} ORDER BY i"],
        base,
    )
    answers = []
    by_text = dict(zip(record(case), [json.loads(value) for (value,) in rows(got[0])], strict=True))
    for answer in case["expect"]["success"]["answers"]:
        text = case["exchanges"][answer["exchange"]]["evidence"]
        found = by_text.get(text, next(iter(by_text.values())))
        answers.append(found.get(answer["name"]))
    return answers


def relations(case: dict, base: str) -> list:
    """Recognize cases: the relations `thinkthen_relations` returns for the
    case's own question file, so the request bytes match the case."""
    got = run([f"SELECT thinkthen_relations({quoted(case['text'])}, {quoted(json.dumps(case['question']))})"], base)
    return [
        {
            "relation": found["relation"],
            "source": [found["source"], found["source_kind"]],
            "target": [found["target"], found["target_kind"]],
            "probability": found["probability"],
        }
        for found in rows(got[0])[0][0]
    ]


def relations_wanted(case: dict) -> list:
    return [
        {
            "relation": found["relation"],
            "source": [found["source"]["name"], found["source"]["kind"]],
            "target": [found["target"]["name"], found["target"]["kind"]],
            "probability": found["probability"],
        }
        for found in expected(case)[0].get("relations") or []
    ]


def counters(case: dict, base: str) -> dict:
    question, text = quoted(json.dumps(case["question"])), quoted(evidence(case)[0])
    with tempfile.TemporaryDirectory() as cache:
        usage = "SELECT metric, value FROM thinkthen_usage() WHERE metric IN ('requests_sent', 'cache_answers')"
        got = run([usage, f"SELECT thinkthen_decide({question}, {text})", f"SELECT thinkthen_decide({question}, {text})", usage], base, extra={"THINKTHEN_CACHE": cache})
    before, after = dict(rows(got[0])), dict(rows(got[3]))
    return {"calls": 2, "requests": after["requests_sent"] - before["requests_sent"], "cache_answers": after["cache_answers"] - before["cache_answers"]}


def fault(case: dict, backend: Backend) -> str:
    """The error word a fault case reads."""
    injection = case.get("operation", {}).get("injection")
    form, question = case.get("question_form"), case["question"]
    ask = "SELECT thinkthen_decide({}, 'evidence text')"
    with tempfile.TemporaryDirectory() as folder:
        if injection == "cancel_token":
            held_cancel("SELECT thinkthen_decide('Does this need attention?', 'held text')", 1)
            return "cancelled"
        if form == "file":
            path = Path(folder) / "q.json"
            path.write_text(json.dumps(question))
            statement = ask.format(quoted("@" + str(path)))
        elif form == "text" and case["verb"] == "rank":
            statement = f"SELECT i FROM {values(['a', 'b'])} ORDER BY thinkthen_probability({quoted(json.dumps(question))}, x) DESC"
        elif form == "text":
            statement = ask.format(quoted(json.dumps(question)))
        elif injection == "invalid_arguments":
            statement = "SELECT thinkthen_decide('', 'evidence text')"
        elif injection == "backend_refusal" or case["id"].startswith("21-"):
            got = run([ask.format(quoted(question["decide"]))], backend.base("arm/refuse"))
            return said(got[0]).split(":")[0].removeprefix("thinkthen ")
        elif injection == "unreadable_question_file" or case["id"].startswith("22-"):
            statement = ask.format(quoted(f"@{folder}/missing.json"))
        elif injection == "expired_deadline":
            statement = "SELECT thinkthen_decide('Does this need attention?', 'evidence text', 0)"
        elif injection == "internal_invariant_failure":
            got = run([ask.format(quoted(question["decide"]))], backend.base(), extension=HOOKS, extra={"thinkthen_test_hook_panic": "scalar"})
            return said(got[0]).split(":")[0].removeprefix("thinkthen ")
        else:
            raise LookupError(f"no runner for the injection {injection!r}")
        got = run([statement], backend.base())
    return said(got[0]).split(":")[0].removeprefix("thinkthen ")


def check(case: dict) -> str | None:
    """None when the case passes, else what differed."""
    with Backend() as backend:
        if "error" in case["expect"]:
            wanted, got = case["expect"]["error"]["kind"], fault(case, backend)
            return None if got == wanted else f"wanted {wanted}, got {got}"
        base = backend.base(f"case/{case['id']}")
        success = case["expect"]["success"]
        kind = success["kind"]
        if "counters" in success:
            got, wanted = counters(case, base), success["counters"]
        elif kind == "recognize":
            got, wanted = relations(case, base), relations_wanted(case)
        elif kind == "filter":
            got, wanted = filtered(case, base), success["operation"]["indexes"]
        elif kind == "rank":
            got, wanted = ranked(case, base), success["operation"]["ranking"]
        elif kind == "annotate":
            got, wanted = annotated(case, base), expected(case)
        elif kind == "decide_many":
            got, wanted = decide(case, base), expected(case)
        elif kind == "single":
            got, wanted = single(case, base), expected(case)
        else:
            raise LookupError(f"no runner for the kind {kind!r}")
        return None if got == wanted else f"wanted {wanted!r}, got {got!r}"


def main() -> int:
    cases = json.loads(CASES.read_text())["cases"]
    passed, failed, skipped = 0, 0, 0
    for case in cases:
        why = reason(case)
        if why:
            skipped += 1
            print(f"not run {case['id']}: {why}")
            continue
        try:
            wrong = check(case)
        except (LookupError, AssertionError, TypeError, KeyError, ValueError) as error:
            wrong = f"refused: {error}"
        if wrong:
            failed += 1
            print(f"FAIL {case['id']}: {wrong}")
        else:
            passed += 1
            print(f"pass {case['id']}")
    print(f"conformance: {passed} pass, {failed} fail, {skipped} not run, {len(cases)} cases")
    if passed + failed + skipped != len(cases):
        print("conformance: the counts do not sum to the cases")
        return 1
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
