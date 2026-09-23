#!/usr/bin/env python3
"""Add the method-H relate recordings to the stand-in's replay table.

Main ruled on 2026-09-23 that every relation uses method H: one yes/no
question per legal pair per relation, and each direction of a one-way
relation is its own question (`sdlc/planning/relate-design.md`). The
bake-off that settled it recorded H on seven sets. This script reads that
experiment's folder (a path argument, default `$THEN_RELATE_METHODS`) and
writes one replay row per set whose records carry no kind, because the
stand-in's relate replay refuses kinds.

Each row has form `yes-no`. Each entry is one recorded question: the rule,
the ordered pair it asked, and the recorded probability of yes. The row
replaces any earlier row of that form with the same records, so the script
is idempotent. It sends nothing and reads only recorded answers.

`build_recognize_cases.py` rewrites the table from the older package and
keeps every `yes-no` row this script wrote.
"""
import json
import os
import re
import sys

FOLDER = sys.argv[1] if len(sys.argv) > 1 else os.environ.get("THEN_RELATE_METHODS", "")
if not FOLDER:
    sys.exit("usage: build_relate_yes_no.py <relate-methods experiment> (or set THEN_RELATE_METHODS)")
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TABLE = f"{ROOT}/standin/data/recognize-replay.json"
# The kind-free sets small enough to read in a review: a both-ways relation
# and a one-way chain.
SETS = ["s2-contradictions", "s3-cause-chain"]


def row(name):
    with open(f"{FOLDER}/sets/{name}.json", encoding="utf-8") as handle:
        spec = json.load(handle)
    if any(item["kind"] != "*" for item in spec["items"]):
        sys.exit(f"{name}: the records carry kinds, which the relate replay refuses")
    with open(f"{FOLDER}/runs/{name}/H/edges.json", encoding="utf-8") as handle:
        edges = json.load(handle)
    with open(f"{FOLDER}/runs/{name}/H/run-00.jsonl", encoding="utf-8") as handle:
        results = [json.loads(line) for line in handle if line.strip()]
    entries = []
    for result in results:
        for question, answer in result["answers"].items():
            rule, source, target = edges[question]["yes"]
            probability = answer["answer"]["probability"]
            if answer["answer"]["kind"] != "yes_no" or not re.fullmatch(r"a_\w+_\d+_\d+", question):
                sys.exit(f"{name}: {question} is not a recorded yes/no question")
            entries.append({
                "pair": [source, target],
                "rule": rule,
                "options": {"yes": probability},
                "pick": "yes" if probability >= 0.5 else "no",
            })
    expected = len(edges)
    if len(entries) != expected:
        sys.exit(f"{name}: {len(entries)} answers for {expected} questions")
    entries.sort(key=lambda entry: (entry["pair"], entry["rule"]))
    records = [item["text"] for item in spec["items"]]
    return {
        "id": f"H-{name}",
        "form": "yes-no",
        "text": "\n".join(records),
        "records": records,
        "rules": [relation["name"] for relation in spec["relations"]],
        "either": {relation["name"]: relation["either"] for relation in spec["relations"]},
        "entries": entries,
    }


def main():
    with open(TABLE, encoding="utf-8") as handle:
        table = json.load(handle)
    rows = [row(name) for name in SETS]
    keys = {tuple(new["records"]) for new in rows}
    table["relate"] = [
        old for old in table["relate"]
        if not (old.get("form") == "yes-no" and tuple(old["records"]) in keys)
    ] + rows
    with open(TABLE, "w", encoding="utf-8") as handle:
        json.dump(table, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    print(f"wrote {len(rows)} yes-no rows, {sum(len(r['entries']) for r in rows)} recorded questions")


if __name__ == "__main__":
    main()
