#!/usr/bin/env python3
"""Add the method-H relate recordings to the stand-in's replay table.

Main ruled on 2026-09-23 that every relation uses method H: one yes/no
question per legal pair per relation, and each direction of a one-way
relation is its own question (`sdlc/planning/relate-design.md`). The rows
come from two folders of the same layout (`sets/`, `runs/<set>/H/`):

- `conformance/relate-h/`, in this repository: the three record sets the
  older harvest package asked by the withdrawn pick-one method, recorded
  again under method H. Each set names the package rows it `replaces`.
- The relate-methods bake-off (a path argument, or `$THEN_RELATE_METHODS`),
  outside this repository: two kind-free sets. Without the path the script
  leaves those rows as they are.

Each row has form `yes-no`. Each entry is one recorded question: the rule,
the ordered pair it asked, and the recorded probability of yes. The row
replaces any earlier row with the same records, so the script is
idempotent, and the pick-one rows a set replaces leave the table. A set
that names a `case` id writes that conformance case from its recording,
in the place of the package cases of the rows it replaces, and those
package cases leave the file. The script sends nothing and reads only
recorded answers.

`build_recognize_cases.py` rewrites the table from the older package. It
keeps every `yes-no` row and every `yes-no` case this script wrote.
"""
import json
import os
import re
import sys

BAKE_OFF = sys.argv[1] if len(sys.argv) > 1 else os.environ.get("THEN_RELATE_METHODS", "")
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TABLE = f"{ROOT}/standin/data/recognize-replay.json"
CASES = f"{ROOT}/conformance/conformance.json"
LOCAL = f"{ROOT}/conformance/relate-h"
# From the bake-off, the kind-free sets small enough to read in a review: a
# both-ways relation and a one-way chain.
BAKE_OFF_SETS = ["s2-contradictions", "s3-cause-chain"]


def row(folder, name):
    with open(f"{folder}/sets/{name}.json", encoding="utf-8") as handle:
        spec = json.load(handle)
    if any(item["kind"] != "*" for item in spec["items"]):
        sys.exit(f"{name}: the records carry kinds, which the relate replay refuses")
    with open(f"{folder}/runs/{name}/H/edges.json", encoding="utf-8") as handle:
        edges = json.load(handle)
    with open(f"{folder}/runs/{name}/H/run-00.jsonl", encoding="utf-8") as handle:
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
    requests = sorted(file[:-5] for file in os.listdir(f"{folder}/runs/{name}/H/cache") if file.endswith(".json"))
    # A set that replaces package rows keeps their numbered text ("Alert 1:
    # ...") as the second match key, as the package rows had it.
    label = spec.get("label")
    text = "\n".join(f"{label} {k}: {record}" for k, record in enumerate(records, 1)) if label else "\n".join(records)
    return {
        "id": f"H-{name}",
        "case": spec.get("case"),
        "replaces": spec.get("replaces", []),
        "requests": requests,
        "form": "yes-no",
        "text": text,
        "records": records,
        "rules": [relation["name"] for relation in spec["relations"]],
        "either": {relation["name"]: relation["either"] for relation in spec["relations"]},
        "entries": entries,
    }


def case(new):
    """The conformance case a row answers, under the set's case id. The
    edges are every recorded yes at or above the 0.5 bar."""
    relations = []
    for name in sorted(new["rules"]):
        rule = {"name": name, "source": "*", "target": "*"}
        if new["either"][name]:
            rule["either"] = True
        relations.append(rule)
    edges = [
        {"name": entry["rule"], "source": entry["pair"][0], "target": entry["pair"][1],
         "probability": entry["options"]["yes"]}
        for entry in new["entries"] if entry["options"]["yes"] >= 0.5
    ]
    edges.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
    return {
        "id": new["case"],
        "source": f"conformance/relate-h runs/{new['id'][2:]}/H (method-H recording, 2026-09-23)",
        "verb": "relate",
        "records": new["records"],
        "question": {"relations": relations, "threshold": 0.5},
        "requests": new["requests"],
        "pairs": len(new["entries"]),
        "exchanges": [],
        "form": "yes-no",
        "expect": {"edges": edges},
    }


def write(path, data):
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=1, ensure_ascii=False)
        handle.write("\n")


def main():
    with open(TABLE, encoding="utf-8") as handle:
        table = json.load(handle)
    names = sorted(file[:-5] for file in os.listdir(f"{LOCAL}/sets") if file.endswith(".json"))
    rows = [row(LOCAL, name) for name in names]
    if BAKE_OFF:
        rows += [row(BAKE_OFF, name) for name in BAKE_OFF_SETS]
    retired = {arm for new in rows for arm in new["replaces"]}
    by_records = {tuple(new["records"]): new for new in rows}
    kept = [
        by_records.pop(tuple(old["records"]), old) for old in table["relate"]
        if old["id"] not in retired or tuple(old["records"]) in by_records
    ]
    table["relate"] = kept + list(by_records.values())
    write(TABLE, table)
    with open(CASES, encoding="utf-8") as handle:
        data = json.load(handle)
    rewritten = 0
    for new in rows:
        if not new["case"]:
            continue
        stale = {f"-relate-{arm}" for arm in new["replaces"]}
        spots = [
            index for index, old in enumerate(data["cases"])
            if old["id"] == new["case"] or (old["verb"] == "relate" and old["id"].endswith(tuple(stale)))
        ]
        at = spots[0] if spots else len(data["cases"])
        data["cases"] = [old for index, old in enumerate(data["cases"]) if index not in spots]
        data["cases"].insert(at, case(new))
        rewritten += 1
    data["case_count"] = len(data["cases"])
    write(CASES, data)
    print(f"wrote {len(rows)} yes-no rows, {sum(len(r['entries']) for r in rows)} recorded questions, "
          f"{rewritten} relate cases, retired {len(retired)} older rows")


if __name__ == "__main__":
    main()
