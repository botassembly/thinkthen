#!/usr/bin/env python3
"""The DuckDB conformance slice: run the one conformance file's cases as
SQL through the stock CLI, offline.

Only the cases a database surface can spell run here; the file's grammar
and digests are the validator's job, not this driver's. A case the shared
table holds back prints its skip or diverge line (the dispositions are
defined in conformance/skiptable.py); every other case is compared.

The driver can fail: answers compare exactly, a `FAILED` line forces a
nonzero exit, and the cases file is the argument when one is passed, so a
corrupted copy can prove the exit code.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# The cases file: the first argument when one is given, the shared file
# otherwise, so a corrupted copy proves this driver's exit code.
CASES = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT.parent.parent / "conformance" / "conformance.json"
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
# The fixture-armed extension, built beside the default one by check.sh:
# the stand-in's partial-failure opt-in is a compile-time door, so the one
# case that replays the failed marker needs a build that carries it.
FIXTURE_EXTENSION = (
    Path(os.environ["ENGINE_FIXTURE_EXTENSION"]).resolve()
    if os.environ.get("ENGINE_FIXTURE_EXTENSION")
    else None
)
DUCKDB = ROOT / "duckdb-bin" / "duckdb"
sys.path.insert(0, str(ROOT.parent.parent / "conformance"))
import skiptable  # noqa: E402 — the one shared skip-table reader
# Whether the extension this runner loads answers from a wire stub: the
# caller says so, and the reader decides `unless: wire` from it. The
# check runs this replay on the null backend, so it is unset there.
WIRE = (bool(os.environ.get("ENGINE_BASE_URL")) or bool(os.environ.get("THINKTHEN_BASE_URL")))

# How many checks failed; `main` turns any into a nonzero exit.
FAILURES = 0


def central_skip(case: dict):
    """The shared skip table's decision for this case, through the one
    reader every surface imports (`conformance/skiptable.py`), with every
    facet the table selects on. A reader that fails raises: a lookup error
    never turns into a run (surfaces-review-5: this runner passed only the
    kind and ran the case on any lookup failure).
    """
    asked = {
        "kind": case.get("expect", {}).get("error", {}).get("kind"),
        "form": case.get("form"),
        "record": "null" if any(r is None for r in case.get("records") or []) else None,
        "none": case.get("none"),
        "error": "error" in case.get("expect", {}),
    }
    return skiptable.lookup_reason("duckdb", case["id"], asked, wire=WIRE)


def run(sql: str, fixture: bool = False) -> str:
    """One statement batch through the stock CLI, values one per line.
    The whole batch rides in `-c` so the output is the plain list form
    exact comparison needs; an init file renders the interactive boxes.
    `fixture` loads the fixture-armed extension for the one case that
    replays the failed marker; the opt-in is compile-time, so no
    environment variable can arm a shipped build."""
    extension = FIXTURE_EXTENSION if fixture and FIXTURE_EXTENSION else EXTENSION
    env = {**os.environ, "ENGINE_NULL": "1"}
    result = subprocess.run(
        [
            str(DUCKDB),
            "-unsigned",
            "-noheader",
            "-list",
            "-c",
            f"LOAD '{extension}'; {sql}",
        ],
        capture_output=True,
        text=True,
        env=env,
        check=False,
    )
    return result.stdout + result.stderr


def check(name: str, sql: str, want: str, fixture: bool = False) -> None:
    global FAILURES
    got = run(sql, fixture).strip()
    if got == want:
        print(f"ok       {name}")
    else:
        FAILURES += 1
        print(f"FAILED   {name}: want {want!r}, got {got!r}")


def check_error(name: str, sql: str, want_kind: str) -> None:
    """An error case: the output is an error whose text begins with the
    expected kind, so a value that merely contains the words cannot pass."""
    global FAILURES
    got = run(sql).strip()
    first = got.splitlines()[0] if got else ""
    marker = first.find("Error: ")
    body = first[marker + len("Error: ") :] if marker >= 0 else ""
    if body.startswith(want_kind):
        print(f"ok       {name}")
    else:
        FAILURES += 1
        print(f"FAILED   {name}: want an error starting {want_kind!r}, got {got!r}")


def sql_string(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def main() -> int:
    global FAILURES
    data = json.loads(CASES.read_text())
    for case in data["cases"]:
        name = case["id"]
        verb = case["verb"]
        evidence = case.get("evidence")
        records = case.get("records")
        central = central_skip(case)
        if central is not None:
            why, disposition = central
            print(f"{disposition:<8} {name}: {why}")
            continue
        if verb == "recognize":
            check_recognize(name, case)
            continue
        if verb == "relate":
            check_relate(name, case)
            continue
        # A question-set file case: this surface's own door, @path, where
        # the error the file cannot produce becomes the local kind's own
        # refusal (the table stopped skipping local cases for the file
        # doors — surfaces-review-4).
        if "question_file" in case:
            file_arg = sql_string("@" + case["question_file"])
            wanted = f"thinkthen {case['expect']['error']['kind']}"
            check_error(name, f"SELECT thinkthen_decide({file_arg}, {sql_string(case.get('evidence') or '')});", wanted)
            continue
        question = case["question"]
        expect = case["expect"]
        kind = question.get("choose", question.get("score", question.get("tag", question.get("decide"))))
        members = question.get("options") or question.get("levels") or question.get("labels")
        arg = json.dumps(question)

        if "error" in expect:
            wanted = f"thinkthen {expect['error']['kind']}"
            text = evidence or (records[0] if records else "")
            if verb in ("choose", "score", "tag"):
                listing = "[" + ",".join(sql_string(m) for m in members) + "]"
                check_error(name, f"SELECT thinkthen_{verb}({sql_string(kind)}, {sql_string(text)}, {listing});", wanted)
            elif verb == "filter":
                FAILURES += 1
                print(f"FAILED   {name}: a filter error case reached the runner; the table holds it back (conformance/skiptable.py)")
            else:
                check_error(name, f"SELECT thinkthen_decide({sql_string(arg)}, {sql_string(text)});", wanted)
            continue

        if verb == "decide":
            wanted = expect["answer"]
            check(name, f"SELECT thinkthen_decide({sql_string(arg)}, {sql_string(evidence)});",
                  "true" if wanted is True else ("false" if wanted is False else "NULL"))
        elif verb in ("choose",):
            wanted = expect.get("answer")
            listing = "[" + ",".join(sql_string(m) for m in members) + "]"
            check(name, f"SELECT thinkthen_choose({sql_string(kind)}, {sql_string(evidence)}, {listing});",
                  "NULL" if wanted is None else wanted)
        elif verb == "score":
            listing = "[" + ",".join(sql_string(m) for m in members) + "]"
            check(name, f"SELECT thinkthen_score({sql_string(kind)}, {sql_string(evidence)}, {listing});",
                  str(expect["answer"]))
        elif verb == "tag":
            listing = "[" + ",".join(sql_string(m) for m in members) + "]"
            held = expect.get("held") or expect.get("labels") or expect.get("answer")
            if held is None:
                print(f"skip     {name}: the case's tag shape has no SQL assertion here")
            else:
                check(name, f"SELECT thinkthen_tag({sql_string(kind)}, {sql_string(evidence)}, {listing});",
                      "[" + ", ".join(held) + "]")
        elif verb == "filter":
            kept = len(expect["indexes"])
            listing = ",".join(sql_string(r) for r in records)
            check(name, f"SELECT count(*) FROM (SELECT unnest([{listing}]) AS text) WHERE thinkthen_decide({sql_string(arg)}, text);",
                  str(kept))
            if expect.get("rows"):
                # The ruled record row (go-ahead item 4): SQL's own two
                # columns are the row, `input` and `value`, in input order.
                # Case 06's empty list is covered by its count above.
                pairs = "~".join(
                    f"{row['input']}|{'true' if row['value'] else 'false'}"
                    for row in expect["rows"]
                )
                check(
                    name + " rows",
                    "SELECT string_agg(text || '|' || thinkthen_decide("
                    f"{sql_string(arg)}, text), '~' ORDER BY i) "
                    f"FROM (SELECT * FROM unnest([{listing}]) WITH ORDINALITY AS t(text, i)) "
                    f"WHERE thinkthen_decide({sql_string(arg)}, text);",
                    pairs,
                )
        elif verb == "decide_many":
            # A JSON null record is SQL NULL: no request is sent for it and
            # its answer stays NULL (the ruled row mapping).
            listing = ",".join("NULL" if r is None else sql_string(r) for r in records)
            wanted = "[" + ", ".join(
                "NULL" if a is None else ("true" if a else "false") for a in expect["answers"]
            ) + "]"
            check(name, f"SELECT list(thinkthen_decide({sql_string(arg)}, text)) FROM (SELECT unnest([{listing}]) AS text);",
                  wanted)
            if expect.get("rows"):
                pairs = "~".join(
                    f"{row['input'] if row['input'] is not None else '<null>'}|"
                    + ("<null>" if row["value"] is None else ("true" if row["value"] else "false"))
                    for row in expect["rows"]
                )
                check(
                    name + " rows",
                    "SELECT string_agg(coalesce(text, '<null>') || '|' || "
                    f"coalesce(thinkthen_decide({sql_string(arg)}, text)::varchar, '<null>'), '~' ORDER BY i) "
                    f"FROM unnest([{listing}]) WITH ORDINALITY AS t(text, i);",
                    pairs,
                )
        elif verb == "details":
            check(name, f"SELECT thinkthen_details({sql_string(arg)}, {sql_string(evidence)}).digest;",
                  expect["details"]["question_sha256"])
            if "requests" in expect["details"]:
                wanted = "[" + ", ".join(expect["details"]["requests"]) + "]"
                check(name + " requests",
                      f"SELECT (thinkthen_details({sql_string(arg)}, {sql_string(evidence)})).requests;",
                      wanted)
        elif verb == "annotate":
            set_json = json.dumps({"version": 1, "questions": case["set"]})
            if expect.get("rows") is not None:
                check_annotate_rows(name, case, set_json, records)
                continue
            answers = expect["answers"]
            # The failed marker rides the stand-in's compile-time opt-in;
            # only a case whose expectation carries a `failed` member needs
            # the fixture-armed extension.
            fixture = any("failed" in member for member in answers.values())
            # The fields come back in the set's own order, the order the
            # file names, so the expectation follows the set, not the
            # alphabet.
            parts = []
            fields_in_order = list(case["set"]) + [f for f in answers if f not in case["set"]]
            for field in fields_in_order:
                member = answers[field]
                if "failed" in member:
                    failed = member["failed"]
                    parts.append(
                        f'"{field}":{{"failed":{{"kind":"{failed["kind"]}","cause":"{failed["cause"]}"}}}}'
                    )
                else:
                    parts.append(
                        f'"{field}":' + ("null" if member["answer"] is None else "true" if member["answer"] is True else "false")
                    )
            fields = ",".join(parts)
            check(name, f"SELECT thinkthen_annotate({sql_string(set_json)}, {sql_string(evidence)});",
                  "{" + fields + "}", fixture)
        else:
            # Every case the table does not hold back has an arm here; a
            # case with none fails rather than skipping silently.
            FAILURES += 1
            print(f"FAILED   {name}: the runner has no arm for verb {verb}; add one or a table entry")
    if FAILURES:
        print(f"{FAILURES} case(s) failed")
        return 1
    return 0


def check_recognize(name: str, case: dict) -> None:
    """One recognize case: the recorded entities as rows, in order, and
    the slice invariant — `body[start + 1 : end]` must be the name, on
    every case, the accent-and-emoji one included. The relation rules a
    case carries ride through `thinkthen_relations`; the scalar takes the
    kinds, the shape the deck draws."""
    text = case["text"]
    kinds = case["question"].get("kinds") or ["person", "organization", "place"]
    listing = "[" + ",".join(sql_string(k) for k in kinds) + "]"
    wanted = "|".join(
        f"{e['text']}:{e['kind']}:{e['start']}:{e['end']}"
        for e in sorted(case["expect"]["entities"], key=lambda e: e["id"])
    )
    # An empty entity list aggregates to no rows, which `string_agg`
    # renders as NULL.
    if not wanted:
        wanted = "NULL"
    check(
        name,
        f"SELECT string_agg(t.text || ':' || t.kind || ':' || t.start || ':' || t.end, '|' ORDER BY t.start) "
        f"FROM (SELECT unnest(thinkthen_recognize({sql_string(text)}, {listing})) AS t);",
        wanted,
    )
    if case["expect"].get("relations"):
        # The relations half, compared (surfaces-review-5: it printed an
        # uncounted diverge). thinkthen_relations takes the case's own
        # recognize question as JSON and names each end by its text.
        by_id = {e["id"]: e["text"] for e in case["expect"]["entities"]}
        want_rel = "|".join(sorted(
            f"{r['name']}~{by_id[r['source']]}~{by_id[r['target']]}~{r['probability']:.4f}"
            for r in case["expect"]["relations"]
        ))
        check(
            name + " relations",
            "SELECT string_agg(r.name || '~' || r.source || '~' || r.target || '~' || printf('%.4f', r.probability), '|' "
            "ORDER BY r.name || '~' || r.source || '~' || r.target || '~' || printf('%.4f', r.probability)) "
            f"FROM (SELECT unnest(thinkthen_relations({sql_string(text)}, {sql_string(json.dumps(case['question']))})) AS r);",
            want_rel,
        )
    check(
        name + " slice",
        f"SELECT count(*) FROM (SELECT unnest(thinkthen_recognize({sql_string(text)}, {listing})) AS t) "
        f"WHERE {sql_string(text)}[t.start + 1:t.end] <> t.text;",
        "0",
    )


def check_annotate_rows(name: str, case: dict, set_json: str, records: list) -> None:
    """A multi-record annotate case: one answer object a record, in input
    order, and a NULL record answers NULL. The field shape is the
    surface's own (a decision is true/false, a score is its object with
    the nearest level and the position), and a numeric expectation reads
    the position, so the case pins the bare answers."""
    global FAILURES
    listing = ",".join("NULL" if record is None else sql_string(record) for record in records)
    fixture = any(
        isinstance(member, dict) and "failed" in member
        for row in case["expect"]["rows"]
        for member in row["value"].values()
    )
    got_raw = run(
        "SELECT thinkthen_annotate(" + sql_string(set_json) + ", text)::varchar "
        f"FROM (SELECT * FROM unnest([{listing}]) WITH ORDINALITY AS t(text, i)) ORDER BY i;",
        fixture=fixture,
    ).strip()
    got_lines = got_raw.splitlines() if got_raw else []
    wanted_rows = case["expect"]["rows"]
    problems = []
    if len(got_lines) != len(wanted_rows):
        problems.append(f"expected {len(wanted_rows)} rows, got {len(got_lines)}")
    else:
        for index, (line, wanted) in enumerate(zip(got_lines, wanted_rows)):
            if line == "NULL":
                problems.append(f"row {index}: NULL where the case expects {wanted['value']}")
                continue
            try:
                got = json.loads(line)
            except json.JSONDecodeError:
                problems.append(f"row {index}: not JSON: {line!r}")
                continue
            for field, value in wanted["value"].items():
                held = got.get(field)
                if isinstance(value, dict) and "failed" in value:
                    if held != {"failed": value["failed"]}:
                        problems.append(f"row {index} {field}: {held!r}, expected the marker")
                elif isinstance(value, (int, float)) and not isinstance(value, bool):
                    if not isinstance(held, dict) or "position" not in held \
                            or abs(held["position"] - value) > 1e-9:
                        problems.append(f"row {index} {field}: {held!r}, expected {value}")
                elif held != value:
                    problems.append(f"row {index} {field}: {held!r}, expected {value!r}")
    if problems:
        FAILURES += 1
        print(f"FAILED   {name}: {'; '.join(problems)}")
    else:
        print(f"ok       {name} rows: {len(wanted_rows)} records in input order")


def check_relate(name: str, case: dict) -> None:
    """One relate case: the recorded edges as rows over the query's own
    ids. The ruled form is `pairs`; the per-subject arm is the engine's
    own and the stand-in serves the pairs row when the texts collide."""
    records = case["records"]
    values = ",".join(
        f"({i + 1}, {sql_string(record)})" for i, record in enumerate(records)
    )
    rules = "[" + ",".join(
        sql_string(rule["name"]) for rule in case["question"]["relations"]
    ) + "]"
    wanted = "|".join(
        f"{e['name']}:{e['source']}:{e['target']}:{e['probability']}"
        for e in case["expect"]["edges"]
    )
    check(
        name,
        "CREATE TABLE tt_case AS SELECT * FROM (VALUES "
        + values
        + ") AS t(id, body); "
        + "SELECT string_agg(name || ':' || source || ':' || target || ':' || probability, '|' ORDER BY CAST(source AS BIGINT), CAST(target AS BIGINT), name) "
        + f"FROM thinkthen_relate('SELECT id, body FROM tt_case', {rules});",
        wanted,
    )


if __name__ == "__main__":
    sys.exit(main())
