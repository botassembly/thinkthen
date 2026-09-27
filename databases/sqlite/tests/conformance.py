#!/usr/bin/env python3
"""Every shared case in `conformance/cases.json`, run as SQL.

Each case runs in its own child process on the backend's case arm, because
the extension's engine reads its address once per process. Expected request
digests were recorded against the canonical URL, so each is recomputed for
the URL the backend served. Every case reports pass, FAIL, or not run. A
not-run reason comes only from `NOT_RUN`, the forms with no SQL spelling,
and the three counts must sum to the file's case count. An optional
argument names another cases file; the planted-failure test uses it.
"""

from __future__ import annotations

import hashlib
import json
import math
import pathlib
import re
import sys

from helper import ROOT, Backend, child, environment

CANONICAL = "https://api.typesafe.ai/v1/systemone"
NOT_RUN = {
    "rank": "no SQL form",
    "find": "no SQL find function yet",
    "relations": "no SQL form: thinkthen_recognize takes no relations, and thinkthen_relate reads rows",
    "defect": "no SQL form: no outside boundary reaches an internal invariant failure",
}
TYPED = {"decide": "thinkthen_decide", "choose": "thinkthen_choose", "tag": "thinkthen_tag", "score": "thinkthen_score"}
FAILED = re.compile(r"^thinkthen (\w+)( \(retryable\))?: ")
SENDING_ERRORS = {"21-backend-fault": 1, "23-cancelled-fault": 1}


def form(case: dict) -> str | None:
    """The NOT_RUN key for a case with no SQL spelling, or None."""
    if case["verb"] in ("rank", "find"):
        return case["verb"]
    if "relations" in case.get("question", {}).get("recognize", {}):
        return "relations"
    if case["expect"].get("error", {}).get("kind") == "defect":
        return "defect"
    return None


def digest(url: str, request: str) -> str:
    return hashlib.sha256(b"systemone\n" + url.encode() + b"\n" + request.encode()).hexdigest()


def swap(value: object, renamed: dict[str, str]) -> object:
    if isinstance(value, str):
        return renamed.get(value, value)
    if isinstance(value, list):
        return [swap(item, renamed) for item in value]
    if isinstance(value, dict):
        return {name: swap(item, renamed) for name, item in value.items()}
    return value


def same(what: str, held: object, wanted: object) -> None:
    """Compare two values, holding numbers to a rounding tolerance."""
    def close(one: object, other: object) -> bool:
        if isinstance(one, bool) or isinstance(other, bool) or one is None or other is None:
            return one == other
        if isinstance(one, (int, float)) and isinstance(other, (int, float)):
            return math.isclose(one, other, abs_tol=1e-9)
        if isinstance(one, list) and isinstance(other, list):
            return len(one) == len(other) and all(map(close, one, other))
        if isinstance(one, dict) and isinstance(other, dict):
            return one.keys() == other.keys() and all(close(one[name], other[name]) for name in one)
        return one == other
    if not close(held, wanted):
        raise AssertionError(f"{what}: got {json.dumps(held)}, expected {json.dumps(wanted)}")


def asked(steps: list, env: dict, setup: str = "") -> list:
    """Run each (sql, parameters) step on one connection; rows or error text."""
    code = f"db = connect()\n{setup}\nsay(results=[run(db, sql, tuple(parameters)) for sql, parameters in {steps!r}])\n"
    return child(code, env)["results"]


def rows_table(texts: list[str]) -> str:
    return f"db.execute('CREATE TABLE r(i INTEGER, t TEXT)')\ndb.executemany('INSERT INTO r VALUES (?, ?)', list(enumerate({texts!r})))"


def refused(case: dict, backend: Backend) -> None:
    ident, question = case["id"], json.dumps(case["question"])
    evidence = case.get("evidence", "Is this urgent?")
    arm, setup, text = "generic", "", question
    steps = [["SELECT thinkthen_decide(?, ?)", [text, evidence]]]
    if ident == "20-usage-fault":
        steps = [["SELECT thinkthen_decide(?, '   ')", [question]]]
    elif ident == "21-backend-fault":
        arm = "arm/refuse"
    elif ident == "22-local-fault":
        steps.insert(0, ["SELECT thinkthen_cache(?)", ["__SCRATCH__/not-a-folder"]])
        setup = "open(os.environ['SCRATCH'] + '/not-a-folder', 'w').write('not a folder')"
    elif ident == "23-cancelled-fault":
        arm = "arm/held"
        setup = "threading.Timer(0.3, db.interrupt).start()"
    elif ident == "24-deadline-fault":
        steps = [["SELECT thinkthen_decide(?, ?, 0)", [question, evidence]]]
    elif ident == "30-local-question-file":
        setup = f"open(os.environ['SCRATCH'] + '/q.json', 'w').write({question!r})"
        steps = [["SELECT thinkthen_decide(?, ?)", ["@__SCRATCH__/q.json", evidence]]]
    elif ident != "29-usage-json-text":
        raise AssertionError(f"no SQL boundary is written for {ident}")
    env = environment(backend, arm)
    steps = json.loads(json.dumps(steps).replace("__SCRATCH__", env["SCRATCH"]))
    results = asked(steps, env, setup)
    backend.release()
    error = next((one for one in results if isinstance(one, str)), None)
    match = FAILED.match(error or "")
    if not match:
        raise AssertionError(f"the case succeeded or failed unnamed: {results}")
    same("kind", match[1], case["expect"]["error"]["kind"])
    same("retryable", bool(match[2]), False)
    same("sent", backend.close(), SENDING_ERRORS.get(ident, 0))


def check(case: dict, backend: Backend) -> None:
    if "error" in case["expect"]:
        return refused(case, backend)
    arm = f"case/{case['id']}"
    served = backend.base(arm) + "/systemone"
    exchanges = case.get("exchanges", [])
    renamed = {digest(CANONICAL, one["request"]): digest(served, one["request"]) for one in exchanges}
    success = swap(case["expect"]["success"], renamed)
    texts = [one["evidence"] for one in exchanges]
    env = environment(backend, arm)
    question = json.dumps(case.get("question"))
    kind, answers = success["kind"], success.get("answers", [])
    if kind == "single":
        calls = success.get("counters", {}).get("calls", 1)
        verb = next(name for name in TYPED if name in case["question"])
        steps = [["SELECT thinkthen_usage()", []]] + [["SELECT thinkthen_details(?, ?)", [question, texts[0]]]] * calls
        steps += [["SELECT thinkthen_usage()", []], [f"SELECT {TYPED[verb]}(?, ?)", [question, texts[0]]]]
        results = asked(steps, env)
        details, typed = json.loads(results[1][0][0]), results[-1][0][0]
        expected = answers[0]
        same("value", details["value"], expected["bare"])
        same("answer", details["answer"], expected["details"]["answer"])
        for name in ("model", "question_sha256", "requests", "usage", "requests_sent", "cached"):
            same(name, details["meta"].get(name, "absent"), expected["details"].get(name, "absent"))
        same("url", details["meta"]["url"], served)
        typed = {1: True, 0: False}.get(typed, typed) if verb == "decide" else typed
        same("typed", json.loads(typed) if verb == "tag" else typed, expected["bare"])
        if "counters" in success:
            before, after = (json.loads(results[at][0][0]) for at in (0, calls + 1))
            moved = {"calls": calls, "requests": after["requests_sent"] - before["requests_sent"],
                     "cache_answers": after["cache_answers"] - before["cache_answers"]}
            same("counters", moved, success["counters"])
    elif kind == "filter":
        results = asked([["SELECT i FROM r WHERE thinkthen_decide(?, t) ORDER BY i", [question]]], env, rows_table(texts))
        same("indexes", [row[0] for row in results[0]], success["operation"]["indexes"])
    elif kind == "decide_many":
        steps = [["SELECT thinkthen_warm(?, t) FROM r", [question]], ["SELECT thinkthen_decide(?, t) FROM r ORDER BY i", [question]]]
        results = asked(steps, env, rows_table(texts))
        same("bare", [{1: True, 0: False}.get(row[0]) for row in results[1]], [one["bare"] for one in answers])
    elif kind == "annotate":
        questions = json.dumps(case["question_set"])
        sent = [json.dumps(case["record"])] if "record" in case else texts
        results = asked([["SELECT thinkthen_annotate(?, ?)", [questions, text]] for text in sent], env)
        records = [json.loads(result[0][0]) for result in results]
        for one in answers:
            same(one["name"], records[0 if "record" in case else one["exchange"]][one["name"]], one["bare"])
        failed = sum(isinstance(value, dict) and "failed" in value for record in records for value in record.values())
        same("failed", failed, success.get("failed_questions", 0))
    elif kind == "recognize":
        sql = 'SELECT text, start, "end", length, kind, strength FROM thinkthen_recognize(?, ?)'
        rows = asked([[sql, [case["text"], question]]], env)[0]
        names = ("text", "start", "end", "length", "kind", "strength")
        same("result", {"entities": [dict(zip(names, row)) for row in rows]}, answers[0]["bare"])
    elif kind == "relate":
        entities = case["entities"]
        setup = f"db.execute('CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)')\ndb.executemany('INSERT INTO e VALUES (?, ?, ?)', [(at, one['name'], one['kind']) for at, one in enumerate({entities!r})])"
        sql = "SELECT relation, source, target, probability FROM thinkthen_relate('e', 'id', 'name', 'kind', ?)"
        rows = asked([[sql, [question]]], env, setup)[0]
        edges = [{"relation": row[0], "source": entities[row[1]], "target": entities[row[2]], "probability": row[3]} for row in rows]
        same("result", edges, answers[0]["bare"])
    else:
        raise AssertionError(f"no SQL form is written for the {kind} kind")
    return None


def main() -> int:
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT.parents[1] / "conformance" / "cases.json"
    document = json.loads(path.read_text())
    counts = {"pass": 0, "FAIL": 0, "not run": 0}
    for case in document["cases"]:
        if (reason := form(case)) is not None:
            counts["not run"] += 1
            print(f"not run  {case['id']}: {NOT_RUN[reason]}")
            continue
        backend = Backend()
        try:
            check(case, backend)
            counts["pass"] += 1
            print(f"pass     {case['id']}")
        except Exception as failure:  # noqa: BLE001 (a case's failure of any kind is reported by name)
            counts["FAIL"] += 1
            print(f"FAIL     {case['id']}: {failure}")
        finally:
            backend.process.kill()
    total = sum(counts.values())
    print(f"{counts['pass']} pass, {counts['FAIL']} FAIL, {counts['not run']} not run, {total} of {document['case_count']}")
    if total != document["case_count"] or len(document["cases"]) != document["case_count"]:
        print(f"FAIL     the counts sum to {total}, and the file holds {document['case_count']}")
        return 1
    return 1 if counts["FAIL"] else 0


if __name__ == "__main__":
    sys.exit(main())
