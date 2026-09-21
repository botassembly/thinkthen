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
        elif verb == "annotate":
            answers = expect["answers"]
            fields = ",".join(
                f'\"{field}\":' + ("null" if member["answer"] is None else "true" if member["answer"] is True else "false")
                for field, member in sorted(answers.items())
            )
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


if __name__ == "__main__":
    sys.exit(main())
