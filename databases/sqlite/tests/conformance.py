#!/usr/bin/env python3
"""Every shared case in `conformance/cases.json`, run as SQL.

Each case runs in its own child process on the backend's case arm, because
the extension's engine reads its address once per process. Expected request
digests were recorded against the canonical URL, so each is recomputed for
the URL the backend served. A record function's row lists the question keys
of ADR 0111 section 2 in place of request digests, and the store holds one
row per good answer. Every case reports pass, FAIL, or not run. A
not-run reason comes only from `NOT_RUN`, the forms with no SQL spelling,
and the three counts must sum to the file's case count. An optional
argument names another cases file; the planted-failure test uses it.
"""

from __future__ import annotations

import hashlib
import json
import math
import os
import pathlib
import re
import sqlite3
import sys

from helper import ROOT, Backend, child, environment
from portable import question_keys

CANONICAL = "https://api.typesafe.ai/v1/systemone"
NOT_RUN = {
    "defect": "no SQL form: no outside boundary reaches an internal invariant failure",
    "groups": "one record's groups were recorded as separate requests, and ADR 0111 section 5 packs them into one",
}
TYPED = {"decide": "thinkthen_decide", "choose": "thinkthen_choose", "tag": "thinkthen_tag", "score": "thinkthen_score"}
FAILED = re.compile(r"thinkthen (usage|local|backend|cancelled|deadline|defect): (.+) \(retryable: (yes|no)\)", re.DOTALL)
SENDING_ERRORS = {"21-backend-fault": 1, "23-cancelled-fault": 1}


def failure(text: str | None) -> tuple[str, bool]:
    """Read the complete ordinary SQL error; shared cases have no removal stubs."""
    match = FAILED.fullmatch(text or "")
    if match is None:
        raise AssertionError(f"the case succeeded or lacked the SQL error form: {text!r}")
    return match[1], match[3] == "yes"


def form(case: dict) -> str | None:
    """The NOT_RUN key for a case with no SQL spelling, or None."""
    if case["expect"].get("error", {}).get("kind") == "defect":
        return "defect"
    if case["verb"] == "annotate" and "record" in case and len(case.get("exchanges", [])) > 1:
        return "groups"
    return None


def selected_ids(cases: list[dict]) -> set[str]:
    available = [case["id"] for case in cases]
    if len(available) != len(set(available)):
        raise ValueError("duplicate shared case ID")
    path = os.environ.get("THINKTHEN_CONFORMANCE_IDS")
    if path is None:
        return set(available)
    source = pathlib.Path(path)
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
        steps.insert(0, ["SELECT thinkthen_configure(json_object('cache',?))", ["__SCRATCH__/not-a-folder"]])
        setup = "open(os.environ['SCRATCH'] + '/not-a-folder', 'w').write('not a folder')"
    elif ident == "23-cancelled-fault":
        arm = "arm/held"
        setup = "threading.Timer(0.3, db.interrupt).start()"
    elif ident == "24-deadline-fault":
        steps = [["SELECT thinkthen_decide(?, ?, ?)", [question, evidence, '{"deadline_ms":0}']]]
    elif ident == "31-usage-rank-blank-question":
        steps = [["SELECT thinkthen_details(?, ?)", [question, evidence]]]
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
    kind, retryable = failure(error)
    same("kind", kind, case["expect"]["error"]["kind"])
    same("retryable", retryable, case["expect"]["error"].get("retryable", False))
    same("sent", backend.close(), SENDING_ERRORS.get(ident, 0))


def check(case: dict, backend: Backend) -> None:
    if "error" in case["expect"]:
        return refused(case, backend)
    relations = "relations" in case.get("question", {}).get("recognize", {})
    arm = f"case/{case['id']}" + ("/capture" if relations or case["verb"] == "find" else "")
    served = backend.base(arm) + "/systemone"
    exchanges = case.get("exchanges", [])
    # Every row lists question keys, by ADR 0111.
    renamed = {digest(CANONICAL, one["request"]): question_keys(served, one["request"]) for one in exchanges}
    success = swap(case["expect"]["success"], renamed)
    texts = [one["evidence"] for one in exchanges]
    env = environment(backend, arm)
    question = json.dumps(case.get("question"))
    kind, answers = success["kind"], success.get("answers", [])
    if kind == "find":
        units = case["question"]["units"]
        rows = asked([["SELECT thinkthen_find(?, ?, ?)",
                       [case["question"]["find"], json.dumps(units), json.dumps({"none": case["question"].get("none", False)})]]], env)[0]
        if isinstance(rows, str):
            raise AssertionError(f"SQL find failed: {rows}")
        result = json.loads(rows[0][0])
        operation = success["operation"]
        selected = operation["selected"]
        wanted = {"index": selected, "value": units[selected] if selected is not None else None,
                  "probability": next(one["probability"] for one in operation["probabilities"]
                                      if one["index"] == selected),
                  "candidates": operation["probabilities"]}
        same("find result", result, wanted)
        observed = backend.capture()
        same("captured body count", len(observed), 1)
        same("captured request", observed[0], exchanges[0]["request"])
        same("captured digest", digest(served, observed[0]), digest(served, exchanges[0]["request"]))
        same("request count", backend.count(), 1)
    elif kind == "single":
        calls = success.get("counters", {}).get("calls", 1)
        verb = next(name for name in TYPED if name in case["question"])
        steps = [["SELECT thinkthen_usage()", []]] + [["SELECT thinkthen_details(?, ?)", [question, texts[0]]]] * calls
        steps += [["SELECT thinkthen_usage()", []], [f"SELECT {TYPED[verb]}(?, ?)", [question, texts[0]]]]
        results = asked(steps, env)
        details, typed = json.loads(results[1][0][0]), results[-1][0][0]
        expected = answers[0]
        same("value", details["value"], expected["bare"])
        same("answer", details["answer"], expected["details"]["answer"])
        for name in ("model", "question_sha256", "usage", "requests_sent", "cached"):
            same(name, details["meta"].get(name, "absent"), expected["details"].get(name, "absent"))
        # A request stands for the list of its keys, so a row reads the flattened list.
        requests = [key for held in expected["details"].get("requests", []) for key in held]
        same("requests", details["meta"].get("requests", "absent"),
             requests if "requests" in expected["details"] else "absent")
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
    elif kind == "rank":
        sql = ("WITH scored AS MATERIALIZED (SELECT i, "
               "json_extract(thinkthen_details(?, t), '$.answer.probability') AS p FROM r) "
               "SELECT i, p FROM scored ORDER BY p DESC, i")
        rows = asked([[sql, [question]]], env, rows_table(texts))[0]
        same("ranking", [{"index": row[0], "probability": row[1]} for row in rows], success["operation"]["ranking"])
        same("judgments sent", backend.count(), len(texts))
    elif kind == "decide_many":
        packed = json.dumps({str(at): text for at, text in enumerate(texts)})
        steps = [["SELECT thinkthen_configure(?)", ['{"batch":1}']],
                 ["SELECT key,value FROM thinkthen_decide_many(?,?) ORDER BY key", [question, packed]]]
        results = asked(steps, env)
        same("bare", [{1: True, 0: False}.get(row[1]) for row in results[1]], [one["bare"] for one in answers])
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
        if relations:
            rows = asked([["SELECT thinkthen_relations(?, ?)", [case["text"], question]]], env)[0]
            if isinstance(rows, str):
                raise AssertionError(f"SQL recognize failed: {rows}")
            same("result", json.loads(rows[0][0]), answers[0]["bare"])
            same("request bodies", backend.capture(), [one["request"] for one in exchanges])
            same("request count", backend.count(), len(exchanges))
        else:
            sql = 'SELECT text, start, "end", length, kind, strength FROM thinkthen_recognize(?, ?)'
            rows = asked([[sql, [case["text"], question]]], env)[0]
            names = ("text", "start", "end", "length", "kind", "strength")
            same("result", {"entities": [dict(zip(names, row)) for row in rows]}, answers[0]["bare"])
    elif kind == "relate":
        entities = case["entities"]
        setup = f"db.execute('CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)')\ndb.executemany('INSERT INTO e VALUES (?, ?, ?)', [(at, one['name'], one['kind']) for at, one in enumerate({entities!r})])"
        sql = "SELECT relation, source, target, probability, either FROM thinkthen_relate('SELECT id, name, kind FROM e', ?)"
        rows = asked([[sql, [question]]], env, setup)[0]
        edges = [{"relation": row[0], "source": entities[row[1]], "target": entities[row[2]], "probability": row[3], **({"either": True} if row[4] == 1 else {})} for row in rows]
        same("result", edges, answers[0]["bare"])
    else:
        raise AssertionError(f"no SQL form is written for the {kind} kind")
    # ADR 0111: the case ran on the question store, one row per good answer.
    keys = {key for one in exchanges for key in question_keys(served, one["request"])}
    same("stored answers", stored(env["THINKTHEN_CACHE"]), len(keys) - success.get("failed_questions", 0))
    return None


def stored(folder: str) -> int:
    """The answer rows in a cache folder's `thinkthen.sqlite`."""
    store = pathlib.Path(folder) / "thinkthen.sqlite"
    if not store.is_file():
        return 0
    connection = sqlite3.connect(store)
    try:
        return connection.execute("SELECT count(*) FROM answers").fetchone()[0]
    finally:
        connection.close()


def main() -> int:
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT.parents[1] / "conformance" / "cases.json"
    document = json.loads(path.read_text())
    try:
        selected = selected_ids(document["cases"])
    except (OSError, ValueError) as error:
        print(f"FAIL     selector: {error}")
        return 1
    counts = {"pass": 0, "FAIL": 0, "not run": 0}
    for case in document["cases"]:
        if case["id"] not in selected:
            continue
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
    print(f"{counts['pass']} pass, {counts['FAIL']} FAIL, {counts['not run']} not run, {total} of {document['case_count']}; total={document['case_count']} selected={len(selected)} pass={counts['pass']} fail={counts['FAIL']} not_run={counts['not run']} unselected={document['case_count'] - len(selected)}")
    if total != len(selected) or len(document["cases"]) != document["case_count"]:
        print(f"FAIL     the counts sum to {total}, and the file holds {document['case_count']}")
        return 1
    return 1 if counts["FAIL"] else 0


if __name__ == "__main__":
    sys.exit(main())
