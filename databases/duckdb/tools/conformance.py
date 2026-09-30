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

import hashlib
import json
import os
import sys
import tempfile
from pathlib import Path

import harness
from harness import ROOT, Backend, expect, rows, run, said
from portable import question_keys
from signal_suite import CANCELLED, held_cancel

CASES = Path(os.environ.get("THINKTHEN_CONFORMANCE_CASES", ROOT.parent.parent / "conformance" / "cases.json"))
CANONICAL_CASES = ROOT.parent.parent / "conformance" / "cases.json"
# Every case runs at batch 1, as the C door and PostgreSQL runners do; `packed_default` pins the packed body.
BATCH_ONE = {"THINKTHEN_BATCH": "1"}

# The one closed list of reasons a case does not run here.
NOT_RUN = {
    "internal_invariant_failure": "the private shared panic-boundary proof replaces the retired C API test hook",
    "groups": "one record's groups were recorded as separate requests, and ADR 0111 section 5 packs them into one",
}


def reason(case: dict) -> str | None:
    if case.get("operation", {}).get("injection") == "internal_invariant_failure":
        return NOT_RUN["internal_invariant_failure"]
    if case["verb"] == "annotate" and "record" in case and len(case.get("exchanges", [])) > 1:
        return NOT_RUN["groups"]
    return NOT_RUN.get(case["verb"])


def selected_ids(cases: list[dict]) -> set[str]:
    """Validate one optional absolute ID list against the canonical corpus."""
    available = [case["id"] for case in cases]
    if len(available) != len(set(available)):
        raise ValueError("duplicate shared case ID")
    path = os.environ.get("THINKTHEN_CONFORMANCE_IDS")
    if path is None:
        return set(available)
    source = Path(path)
    if not source.is_absolute():
        raise ValueError("THINKTHEN_CONFORMANCE_IDS takes an absolute path")
    chosen = [line.strip() for line in source.read_text().splitlines()]
    chosen = [one for one in chosen if one and not one.startswith("#")]
    if not chosen:
        raise ValueError("the selected case list is empty")
    if len(chosen) != len(set(chosen)):
        raise ValueError("duplicate selected case ID")
    absent = set(chosen) - set(available)
    if absent:
        raise ValueError(f"selected case is absent from the shared corpus: {sorted(absent)[0]}")
    return set(chosen)


def quoted(text: str) -> str:
    return "'" + text.replace("'", "''") + "'"


def values(texts: list[str]) -> str:
    return "(VALUES " + ", ".join(f"({index}, {quoted(text)})" for index, text in enumerate(texts)) + ") t(i, x)"


def expected(case: dict) -> list:
    return [answer["bare"] for answer in case["expect"]["success"]["answers"]]


def evidence(case: dict) -> list[str]:
    return [exchange["evidence"] for exchange in case["exchanges"]]


FIELDS = ("model", "question_sha256", "requests", "usage", "requests_sent", "cached")
CANONICAL = "https://api.typesafe.ai/v1/systemone"


def digest(url: str, request: str) -> str:
    return hashlib.sha256(f"systemone\n{url}\n{request}".encode()).hexdigest()


def single(case: dict, base: str) -> list:
    """decide, choose, tag, and score: the whole line `thinkthen_details` returns."""
    question = quoted(json.dumps(case["question"]))
    got = run([f"SELECT thinkthen_details({question}, x) FROM {values(evidence(case))} ORDER BY i"], base, extra=BATCH_ONE)
    lines = [json.loads(line) for (line,) in rows(got[0])]
    return [{"bare": line["value"], "answer": line["answer"], "url": line["meta"]["url"]} | {name: line["meta"].get(name, "absent") for name in FIELDS} for line in lines]


def single_wanted(case: dict, base: str) -> list:
    served = base + "/systemone"
    # A recorded request digest stands for its questions' keys (ADR 0111 section 7); a row reads them flattened.
    renamed = {digest(CANONICAL, one["request"]): question_keys(served, one["request"]) for one in case["exchanges"]}
    wanted = []
    for answer in case["expect"]["success"]["answers"]:
        details = answer["details"] | {"requests": [key for held in answer["details"]["requests"] for key in renamed[held]]}
        wanted.append({"bare": answer["bare"], "answer": details["answer"], "url": served} | {name: details.get(name, "absent") for name in FIELDS})
    return wanted


def decide(case: dict, base: str) -> list:
    got = run([f"SELECT thinkthen_decide({quoted(json.dumps(case['question']))}, x) FROM {values(evidence(case))} ORDER BY i"], base, extra=BATCH_ONE)
    return [value for (value,) in rows(got[0])]


def packed_default() -> None:
    """The installed default sends one exact three-record request."""
    question = quoted('{"decide":"Does the writer ask for a refund?","threshold":0.5}')
    query = (f"SELECT thinkthen_decide({question}, x) FROM "
             "(VALUES (0, 'refund now'), (1, 'good morning'), (2, 'maybe so')) t(i,x) ORDER BY i")
    expected = (
        '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0",'
        '"questions":{"q1":{"type":"noul","instructions":"The text is \\"refund now\\". Does the writer ask for a refund?"},'
        '"q2":{"type":"noul","instructions":"The text is \\"good morning\\". Does the writer ask for a refund?"},'
        '"q3":{"type":"noul","instructions":"The text is \\"maybe so\\". Does the writer ask for a refund?"}}}'
    )
    with Backend() as backend:
        expect(rows(run([query], backend.base())[0]), [[True], [True], [True]], "default packed answers")
        expect(backend.count(), 1, "default packed send count")
    with Backend() as backend:
        refused = run(["SET thinkthen_max_retries = 0", query],
                      backend.base("case/42-recognize-C01-relations/capture"))
        expect("status 500" in said(refused[1]), True, "the capture arm intentionally refuses")
        expect(backend.count(), 1, "one captured packed request")
        backend._say("capture")
        expect(json.loads(backend._line()), {"bodies": [expected]}, "exact default packed request")


def filtered(case: dict, base: str) -> list:
    table = values(evidence(case)) if case["exchanges"] else "(SELECT 0 AS i, 'none' AS x WHERE false) t"
    got = run([f"SELECT i FROM {table} WHERE thinkthen_decide({quoted(json.dumps(case['question']))}, x) ORDER BY i"], base, extra=BATCH_ONE)
    return [index for (index,) in rows(got[0])]


def ranked(case: dict, base: str) -> list:
    got = run(
        [f"SELECT i, thinkthen_probability({quoted(json.dumps(case['question']))}, x) AS p FROM {values(evidence(case))} ORDER BY p DESC, i"],
        base,
        extra=BATCH_ONE,
    )
    return [{"index": index, "probability": probability} for index, probability in rows(got[0])]


def record(case: dict) -> list[str]:
    """The records an annotate case reads: its one JSON record, whose parts
    the set's `on` pointers select, or one per distinct exchange."""
    if "record" in case:
        return [json.dumps(case["record"])]
    return list(dict.fromkeys(evidence(case)))


def annotated(case: dict, base: str) -> list:
    got = run(
        [f"SELECT thinkthen_annotate({quoted(json.dumps(case['question_set']))}, x) FROM {values(record(case))} ORDER BY i"],
        base,
        extra=BATCH_ONE,
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
    got = run([f"SELECT thinkthen_relations({quoted(case['text'])}, {quoted(json.dumps(case['question']))})"], base, extra=BATCH_ONE)
    return [
        {
            "relation": found["relation"],
            "source": [found["source"], found["source_kind"]],
            "target": [found["target"], found["target_kind"]],
            "probability": found["probability"],
            "either": found["either"],
        }
        for found in rows(got[0])[0][0]
    ]


def relations_wanted(case: dict) -> list:
    return [
        {
            "relation": found["relation"],
            "source": [found["source"]["text"], found["source"]["kind"]],
            "target": [found["target"]["text"], found["target"]["kind"]],
            "probability": found["probability"],
            "either": found.get("either", False),
        }
        for found in expected(case)[0].get("relations") or []
    ]


def related(case: dict, base: str) -> list:
    """Relate cases: `thinkthen_relate` over the case's entities, each
    entity's name as its id, with the case's own rules file text."""
    table = ", ".join(f"({quoted(entity['name'])}, {quoted(entity['name'])}, {quoted(entity['kind'])})" for entity in case["entities"])
    query = f"SELECT * FROM (VALUES {table}) v(id, name, kind)"
    got = run([f"SELECT * FROM thinkthen_relate({quoted(query)}, {quoted(json.dumps(case['question']))})"], base, extra=BATCH_ONE)
    kinds = {entity["name"]: entity["kind"] for entity in case["entities"]}
    return [
        {"relation": relation, "source": {"name": source, "kind": kinds[source]}, "target": {"name": target, "kind": kinds[target]}, "probability": probability, **({"either": True} if either else {})}
        for relation, source, target, probability, either in rows(got[0])
    ]


FIND_RESULTS = {
    "18-find-second": {
        "index": 1, "value": "Second passage.", "probability": 0.8,
        "candidates": [{"index": 0, "probability": 0.1}, {"index": 1, "probability": 0.8},
                       {"index": 2, "probability": 0.05}, {"index": None, "probability": 0.05}],
    },
    "19-find-none": {
        "index": None, "value": None, "probability": 0.7,
        "candidates": [{"index": 0, "probability": 0.1}, {"index": 1, "probability": 0.2},
                       {"index": None, "probability": 0.7}],
    },
}


def find(case: dict, base: str, backend: Backend) -> dict:
    question = case["question"]
    sql = (f"SELECT thinkthen_find({quoted(question['find'])}, list(x ORDER BY i), "
           f"{quoted(json.dumps({'none': question.get('none', False)}))}) FROM {values(question['units'])}")
    result = rows(run([sql], base, extra=BATCH_ONE)[0])[0][0]
    observed = backend.capture()
    expect(len(observed), 1, "one captured find body")
    pinned = case["exchanges"][0]["request"]
    expect(observed[0], pinned, "complete sent find body")
    served = base + "/systemone"
    expect(digest(served, observed[0]), digest(served, pinned), "served find request digest")
    expect(backend.count(), 1, "one find attempt")
    return result


def counters(case: dict, base: str) -> dict:
    question, text = quoted(json.dumps(case["question"])), quoted(evidence(case)[0])
    with tempfile.TemporaryDirectory() as cache:
        usage = "SELECT metric, value FROM thinkthen_usage() WHERE metric IN ('requests_sent', 'cache_answers')"
        got = run([usage, f"SELECT thinkthen_decide({question}, {text})", f"SELECT thinkthen_decide({question}, {text})", usage], base, extra=BATCH_ONE | {"THINKTHEN_CACHE": cache})
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
            got = run([ask.format(quoted(question["decide"]))], backend.base("arm/refuse"), extra=BATCH_ONE)
            return said(got[0]).split(":")[0].removeprefix("thinkthen ")
        elif injection == "unreadable_question_file" or case["id"].startswith("22-"):
            statement = ask.format(quoted(f"@{folder}/missing.json"))
        elif injection == "expired_deadline":
            statement = "SELECT thinkthen_decide('Does this need attention?', 'evidence text', '{\"deadline_ms\":0}')"
        else:
            raise LookupError(f"no runner for the injection {injection!r}")
        got = run([statement], backend.base(), extra=BATCH_ONE)
    return said(got[0]).split(":")[0].removeprefix("thinkthen ")


def check(case: dict) -> str | None:
    """None when the case passes, else what differed."""
    with Backend() as backend:
        if "error" in case["expect"]:
            wanted, got = case["expect"]["error"]["kind"], fault(case, backend)
            return None if got == wanted else f"wanted {wanted}, got {got}"
        arm = f"case/{case['id']}" + ("/capture" if case["verb"] == "find" else "")
        base = backend.base(arm)
        success = case["expect"]["success"]
        harness.STORED.clear()
        kind = success["kind"]
        if kind == "find":
            got, wanted = find(case, base, backend), FIND_RESULTS[case["id"]]
        elif kind == "relate":
            got, wanted = related(case, base), expected(case)[0]
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
            if got == wanted and case["id"] == "27-decide-many":
                packed_default()
        elif kind == "single":
            pairs = zip(single(case, base), single_wanted(case, base), strict=True)
            why = next((f"{name}: wanted {one[name]!r}, got {other[name]!r}" for other, one in pairs for name in one if one[name] != other[name]), None)
            if why:
                return why
            got, wanted = None, None
            if "counters" in success:
                got, wanted = counters(case, base), success["counters"]  # last, on a cache folder of their own
        else:
            raise LookupError(f"no runner for the kind {kind!r}")
        if got != wanted:
            return f"wanted {wanted!r}, got {got!r}"
        return kept(case, base, success)


def kept(case: dict, base: str, success: dict) -> str | None:
    """ADR 0111: the case's first child ran on the question store, one row per good answer."""
    keys = {key for one in case.get("exchanges", []) for key in question_keys(base + "/systemone", one["request"])}
    wanted = len(keys) - success.get("failed_questions", 0)
    got = harness.STORED[0] if harness.STORED else 0
    return None if got == wanted else f"stored answers: wanted {wanted}, got {got}"


def main() -> int:
    canonical = json.loads(CANONICAL_CASES.read_text())
    cases = canonical["cases"]
    if len(cases) != canonical["case_count"]:
        raise ValueError("the canonical case count differs from its cases")
    selected = selected_ids(cases)
    alternate = json.loads(CASES.read_text())["cases"]
    by_id = {case["id"]: case for case in alternate}
    if len(by_id) != len(alternate) or set(by_id) - {case["id"] for case in cases}:
        raise ValueError("the executable case file has duplicate or unknown IDs")
    passed, failed, skipped = 0, 0, 0
    for case in cases:
        if case["id"] not in selected:
            continue
        if case["id"] not in by_id:
            raise ValueError(f"selected case is absent from the executable file: {case['id']}")
        case = by_id[case["id"]]
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
    print(f"conformance: total={len(cases)} selected={len(selected)} pass={passed} fail={failed} not_run={skipped} unselected={len(cases) - len(selected)}")
    if passed + failed + skipped != len(selected):
        print("conformance: the counts do not sum to the selected cases")
        return 1
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
