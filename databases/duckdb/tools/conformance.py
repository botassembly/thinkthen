#!/usr/bin/env python3
"""The DuckDB conformance slice: run the one conformance file's cases as
SQL through the stock CLI, offline.

Only the cases a database surface can spell run here; the file's grammar
and digests are the validator's job, not this driver's. The known
divergences print as `diverge` with the reason, matching the other
surfaces' drivers.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CASES = ROOT.parent.parent / "conformance" / "conformance.json"
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
DUCKDB = ROOT / "duckdb-bin" / "duckdb"


def run(sql: str) -> str:
    with tempfile.NamedTemporaryFile("w", suffix=".sql", delete=False) as handle:
        handle.write(f"LOAD '{EXTENSION}';\n{sql}\n")
        name = handle.name
    result = subprocess.run(
        [str(DUCKDB), "-unsigned", "-noheader", "-list", "-init", name],
        capture_output=True,
        text=True,
        env={**os.environ, "ENGINE_NULL": "1"},
        check=False,
    )
    Path(name).unlink()
    return result.stdout + result.stderr


def check(name: str, sql: str, want: str) -> None:
    got = run(sql).strip()
    if want in got:
        print(f"ok       {name}")
    else:
        print(f"FAILED   {name}: want {want!r}, got {got!r}")


def sql_string(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def main() -> int:
    data = json.loads(CASES.read_text())
    for case in data["cases"]:
        name = case["id"]
        verb = case["verb"]
        evidence = case.get("evidence")
        records = case.get("records")
        if "question_file" in case:
            print(f"skip     {name}: the local kind needs a file door")
            continue
        if verb == "recognize":
            check_recognize(name, case)
            continue
        if verb == "relate":
            check_relate(name, case)
            continue
        if verb in ("rank", "find"):
            print(f"skip     {name}: {verb} is not this surface's SQL shape; the surface check owns it")
            continue
        question = case["question"]
        expect = case["expect"]
        kind = question.get("choose", question.get("score", question.get("tag", question.get("decide"))))
        members = question.get("options") or question.get("levels") or question.get("labels")
        arg = json.dumps(question)

        if "error" in expect:
            if verb == "cancel":
                print(f"diverge  {name}: the CLI's Ctrl-C owns cancel; see NOTES")
                continue
            if expect["error"]["kind"] == "deadline":
                print(f"skip     {name}: the spent-budget case needs a deadline door this driver does not carry")
                continue
            wanted = f"thinkthen {expect['error']['kind']}"
            text = evidence or (records[0] if records else "")
            if verb in ("choose", "score", "tag"):
                listing = "[" + ",".join(sql_string(m) for m in members) + "]"
                check(name, f"SELECT thinkthen_{verb}({sql_string(kind)}, {sql_string(text)}, {listing});", wanted)
            elif verb == "filter":
                print(f"diverge  {name}: SQL spells filter as WHERE, and a band under WHERE reads as NULL by rule 6; the refusal belongs to the surface check")
            else:
                check(name, f"SELECT thinkthen_decide({sql_string(arg)}, {sql_string(text)});", wanted)
            continue

        if verb == "decide":
            wanted = expect["answer"]
            check(name, f"SELECT thinkthen_decide({sql_string(arg)}, {sql_string(evidence)});",
                  "true" if wanted is True else ("false" if wanted is False else ""))
        elif verb in ("choose",):
            wanted = expect.get("answer")
            listing = "[" + ",".join(sql_string(m) for m in members) + "]"
            check(name, f"SELECT thinkthen_choose({sql_string(kind)}, {sql_string(evidence)}, {listing});",
                  "" if wanted is None else wanted)
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
        elif verb == "decide_many":
            listing = ",".join(sql_string(r) for r in records)
            wanted = "[" + ", ".join("true" if a else "false" for a in expect["answers"]) + "]"
            check(name, f"SELECT list(thinkthen_decide({sql_string(arg)}, text)) FROM (SELECT unnest([{listing}]) AS text);",
                  wanted)
        elif verb == "details":
            check(name, f"SELECT thinkthen_details({sql_string(arg)}, {sql_string(evidence)}).digest;",
                  expect["details"]["question_sha256"])
            if "requests" in expect["details"]:
                wanted = "[" + ", ".join(expect["details"]["requests"]) + "]"
                check(name + " requests",
                      f"SELECT (thinkthen_details({sql_string(arg)}, {sql_string(evidence)})).requests;",
                      wanted)
        elif verb == "annotate":
            answers = expect["answers"]
            parts = []
            for field, member in sorted(answers.items()):
                if "failed" in member:
                    failed = member["failed"]
                    parts.append(
                        f'"{field}":{{"failed":{{"cause":"{failed["cause"]}","kind":"{failed["kind"]}"}}}}'
                    )
                else:
                    parts.append(
                        f'"{field}":' + ("null" if member["answer"] is None else "true" if member["answer"] is True else "false")
                    )
            fields = ",".join(parts)
            set_json = json.dumps({"version": 1, "questions": case["set"]})
            check(name, f"SELECT thinkthen_annotate({sql_string(set_json)}, {sql_string(evidence)});",
                  "{" + fields + "}")
        elif verb == "usage":
            print(f"diverge  {name}: the disk cache waits on the real engine (named)")
        elif verb == "cancel":
            print(f"diverge  {name}: the CLI's Ctrl-C owns cancel; see NOTES")
        else:
            print(f"skip     {name}: {verb} has no SQL spelling on this surface")
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
    check(
        name,
        f"SELECT string_agg(t.text || ':' || t.kind || ':' || t.start || ':' || t.end, '|' ORDER BY t.start) "
        f"FROM (SELECT unnest(thinkthen_recognize({sql_string(text)}, {listing})) AS t);",
        wanted,
    )
    if case["expect"].get("relations"):
        print(
            f"diverge  {name}: the relations half rides thinkthen_relations; the scalar is the kinds-only shape"
        )
    check(
        name + " slice",
        f"SELECT count(*) FROM (SELECT unnest(thinkthen_recognize({sql_string(text)}, {listing})) AS t) "
        f"WHERE {sql_string(text)}[t.start + 1:t.end] <> t.text;",
        "0",
    )


def check_relate(name: str, case: dict) -> None:
    """One relate case: the recorded edges as rows over the query's own
    ids. The ruled form is `pairs`; the per-subject arm is the engine's
    own and the stand-in serves the pairs row when the texts collide."""
    if case.get("form") == "per-subject":
        print(
            f"diverge  {name}: per-subject is the engine-internal arm; the ruled form is pairs and the stand-in serves it on a text collision"
        )
        return
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
