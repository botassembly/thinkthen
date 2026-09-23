#!/usr/bin/env python3
"""Write the method-H question sets for the relate recordings. Sends nothing.

Main ruled method H for every relation on 2026-09-23: one yes/no question
per legal pair per relation, and each direction of a one-way relation is
its own question. This script follows the relate-methods bake-off's lean
wording. The state carries the numbered records after one preface line.
Each question carries only "Does this hold: <statement>?".

Every set in `sets/` names kind `*` on every record and every rule end, so
every ordered pair is legal for a one-way rule and every unordered pair for
a both-ways rule. For each set the script writes `runs/<set>/H/`:
`records.jsonl` (one record: the numbered list), `qset-NN.json` (one per
request, at most CHUNK_BYTES of questions), and `edges.json` (question
name to the edge its yes means). `job.sh` asks and records them.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CHUNK_BYTES = 90_000


def arm(name, spec):
    items = [item["text"] for item in spec["items"]]
    listing = "\n".join(f"Item {k}: {text}" for k, text in enumerate(items, 1))
    preface = " ".join(x for x in (spec["intro"], spec["note"], "Each question names items by number.") if x)
    questions, edges = {}, {}
    for rule in spec["relations"]:
        for a in range(1, len(items) + 1):
            for b in range(1, len(items) + 1):
                if a == b or (rule["either"] and b < a):
                    continue
                if rule["either"]:
                    statement = f"Item {a} and Item {b} {rule['phrase']}"
                else:
                    statement = f"Item {a} {rule['phrase']} Item {b}"
                key = f"a_{rule['name']}_{a}_{b}"
                questions[key] = {"decide": f"Does this hold: {statement}?"}
                edges[key] = {"yes": [rule["name"], a, b]}
    chunks, current, size = [], {}, 0
    for key, question in questions.items():
        width = len(json.dumps(question, ensure_ascii=False))
        if current and size + width > CHUNK_BYTES:
            chunks.append(current)
            current, size = {}, 0
        current[key] = question
        size += width
    if current:
        chunks.append(current)
    folder = f"{HERE}/runs/{name}/H"
    os.makedirs(folder, exist_ok=True)
    for old in os.listdir(folder):
        if old.startswith("qset-"):
            os.remove(f"{folder}/{old}")
    with open(f"{folder}/records.jsonl", "w", encoding="utf-8") as handle:
        handle.write(json.dumps({"e": {"text": f"{preface}\n\n{listing}"}}, ensure_ascii=False) + "\n")
    for index, chunk in enumerate(chunks):
        with open(f"{folder}/qset-{index:02d}.json", "w", encoding="utf-8") as handle:
            json.dump({"version": 1, "questions": chunk}, handle, indent=1, ensure_ascii=False)
            handle.write("\n")
    with open(f"{folder}/edges.json", "w", encoding="utf-8") as handle:
        json.dump(edges, handle, indent=1)
        handle.write("\n")
    print(f"{name}: {len(questions)} questions, {len(chunks)} requests")


def main():
    for file in sorted(os.listdir(f"{HERE}/sets")):
        with open(f"{HERE}/sets/{file}", encoding="utf-8") as handle:
            arm(file[:-5], json.load(handle))


if __name__ == "__main__":
    main()
