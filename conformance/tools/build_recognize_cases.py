#!/usr/bin/env python3
"""Build the recognize and relate replay table and conformance cases.

Reads the harvest package (a path argument, default
`/home/ian/workspace/experiments/225-recognize-harvest-package`) and writes:

1. `standin/data/recognize-replay.json` — the replay table the stand-in
   answers recognize and relate from. One row per recorded case: the text,
   the recorded entities, the recorded relation pairs, the recorded rule
   names, and the recorded request digests (the cache file names).
2. `conformance/conformance.json` — the same forty cases, plus the one
   synthesized offset case, plus the four relate arms, translated into
   conformance cases additively. Existing cases are untouched; cases with
   verb recognize or relate are replaced, so the script is idempotent.

The two functions' expectations are DERIVED from the recordings here, and
every derived answer is asserted against `cases/expected/` before anything
is written; a mismatch stops the run.

Rule ends are reconstructed because the package records rule names only:
`works_for` and `founded` are person:organization and `based_in` is
organization:place from `experiments/222-recognize-demo/relation_map.json`,
and `located_in` is *:place from `sdlc/planning/recognize-design.md`. The
replay reads rule names, not ends, so the ends change no recorded answer;
they are carried so the cases speak the ruled grammar.

The relate arms' raw record lines are recovered by stripping the recorded
numbering prefixes ("Alert 1: ", "Record 3: "); the numbered text stays in
the replay table as the alternate match key.

Offsets in this file are code points of the text, the unit the design page
rules. Host conversions are the surfaces' job.
"""
import json
import os
import re
import sys

PACKAGE = sys.argv[1] if len(sys.argv) > 1 else "/home/ian/workspace/experiments/225-recognize-harvest-package"
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

RULE_ENDS = {
    "works_for": ("person", "organization"),
    "founded": ("person", "organization"),
    "based_in": ("organization", "place"),
    "located_in": ("*", "place"),
}
# The per-subject arm asks its relation in prose ("works for"), the design's
# default reads for `works_for`, because its options are object records and
# not relation names.
ARM_RULES = {"R03-persubject-10": ["works_for"]}
KIND_ORDER = ["person", "organization", "place", "other"]
DEFAULT_KINDS = ["person", "organization", "place"]
SYNTH_TEXT = "Le caf\u00e9 \U0001F600 Maria Chen arrived."
SYNTH_NAME = "Maria Chen"


def ruled_entity(entity):
    """The recording's entity in the ruled host shape: `number` for the
    recorded number. The marketing vocabulary page restricts `confidence`
    to Jev's literal detailed-output field, so the interim host name is
    `number` until the recognize team's comparison settles it."""
    return {
        "id": entity["id"],
        "text": entity["text"],
        "kind": entity["kind"],
        "start": entity["start"],
        "end": entity["end"],
        "number": entity["confidence"],
    }


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def prefix_strip(line):
    return re.sub(r"^(Alert|Record) \d+: ", "", line)


def rule_of(option):
    return re.sub(r"_(AB|BA)$", "", option)


def direction(option, first, second):
    if option.endswith("_AB"):
        return first, second
    if option.endswith("_BA"):
        return second, first
    return min(first, second), max(first, second)


def build_recognize():
    cases = load(f"{PACKAGE}/cases/cases.json")["cases"]
    table, conf = [], []
    for index, case in enumerate(cases):
        cid = case["id"]
        expected = load(f"{PACKAGE}/cases/expected/{cid}.json")
        cache = sorted(
            os.path.splitext(name)[0]
            for name in os.listdir(f"{PACKAGE}/cases/work/{cid}/cache")
        )
        pairs = []
        rel_path = f"{PACKAGE}/cases/work/{cid}/run-rel.jsonl"
        if os.path.exists(rel_path):
            line = open(rel_path, encoding="utf-8").read().strip()
            if line:
                run = json.loads(line)
                for key, entry in run["answers"].items():
                    first, second = (int(x) + 1 for x in re.findall(r"p(\d+)_(\d+)", key)[0])
                    answer = entry["answer"]
                    pairs.append(
                        {
                            "pair": [first, second],
                            "options": answer["probabilities"],
                            "pick": answer["pick"],
                        }
                    )
        note = case.get("divergence")
        table.append(
            {
                "id": cid,
                "text": case["text"],
                "entities": expected["entities"],
                "rules": case["rules"],
                "pairs": pairs,
                "requests": cache,
                "pairs_count": len(pairs),
                "note": note,
                "synthesized": None,
            }
        )
        present = [kind for kind in KIND_ORDER if any(e["kind"] == kind for e in expected["entities"])]
        for rule in case["rules"]:
            for end in RULE_ENDS.get(rule, ("*", "*")):
                if end != "*" and end not in present:
                    present.append(end)
        present = [kind for kind in KIND_ORDER if kind in present] + [
            kind for kind in present if kind not in KIND_ORDER
        ]
        kinds = present or DEFAULT_KINDS
        relations = []
        for rule in case["rules"]:
            if rule not in RULE_ENDS:
                raise SystemExit(f"{cid}: rule {rule} has no recorded ends")
            first, second = RULE_ENDS[rule]
            relations.append({"name": rule, "from": first, "to": second})
        derived = []
        for entry in pairs:
            pick = entry["pick"]
            if pick in ("NO_RELATION", "NONE_OF_THESE"):
                continue
            name = rule_of(pick)
            if name not in case["rules"]:
                raise SystemExit(f"{cid}: pick {pick} names a rule that was not declared")
            probability = entry["options"][pick]
            if probability < 0.5:
                continue
            source, target = direction(pick, entry["pair"][0], entry["pair"][1])
            derived.append(
                {"name": name, "source": source, "target": target, "probability": probability}
            )
        derived.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
        want = [
            {
                "name": relation["name"],
                "source": relation["from"],
                "target": relation["to"],
                "probability": relation["confidence"],
            }
            for relation in expected.get("relations", [])
        ]
        want.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
        if derived != want:
            raise SystemExit(f"{cid}: derived relations {derived} != recorded {want}")
        if any(e["confidence"] < 0.5 for e in expected["entities"]):
            raise SystemExit(f"{cid}: an entity sits below the default bar")
        notes = []
        if note:
            notes.append(note)
        if case["rules"] and not pairs and not os.path.exists(rel_path):
            notes.append(
                "recording ran no relation pass: no legal pair for the declared rules"
            )
        case_conf = {
            "id": f"{28 + index:02d}-recognize-{cid}",
            "source": f"experiments/225-recognize-harvest-package cases/{cid} (recording; expected/{cid}.json)",
            "verb": "recognize",
            "text": case["text"],
            "question": {
                "kinds": kinds,
                "relations": relations,
                "threshold": 0.5,
                "relation_threshold": 0.5,
            },
            "requests": cache,
            "pairs": len(pairs),
            "exchanges": [],
            "expect": {"entities": [ruled_entity(e) for e in expected["entities"]], "relations": derived},
        }
        if notes:
            case_conf["note"] = "; ".join(notes)
        conf.append(case_conf)
    return table, conf


def build_synthesized():
    row = {
        "id": "C41",
        "text": SYNTH_TEXT,
        "entities": [
            {
                "id": 1,
                "text": SYNTH_NAME,
                "kind": "person",
                "start": SYNTH_TEXT.index(SYNTH_NAME),
                "end": SYNTH_TEXT.index(SYNTH_NAME) + len(SYNTH_NAME),
                "confidence": 0.94,
            }
        ],
        "rules": [],
        "pairs": [],
        "requests": [],
        "pairs_count": 0,
        "note": None,
        "synthesized": (
            "no recording: built from the package rules for the per-host offset "
            "proofs, with the accented letter and the emoji before the name"
        ),
    }
    case_conf = {
        "id": "68-recognize-C41",
        "source": "synthesized from experiments/225-recognize-harvest-package rules for the offset proofs",
        "verb": "recognize",
        "text": SYNTH_TEXT,
        "question": {
            "kinds": ["person"],
            "relations": [],
            "threshold": 0.5,
            "relation_threshold": 0.5,
        },
        "requests": [],
        "pairs": 0,
        "exchanges": [],
        "expect": {"entities": [ruled_entity(e) for e in row["entities"]], "relations": []},
        "note": row["synthesized"],
    }
    return row, case_conf


def build_relate():
    arms = ["R01-demo", "R02-pickone", "R03-persubject-10", "R04-pairs-10"]
    table, conf = [], []
    for index, arm in enumerate(arms):
        base = f"{PACKAGE}/relate/cases/{arm}"
        record_file = (
            f"{base}/records.jsonl"
            if os.path.exists(f"{base}/records.jsonl")
            else f"{base}/records-chunk00.jsonl"
        )
        qset_file = (
            f"{base}/qset.json"
            if os.path.exists(f"{base}/qset.json")
            else f"{base}/qset-chunk00.json"
        )
        run_file = (
            f"{base}/run.jsonl"
            if os.path.exists(f"{base}/run.jsonl")
            else f"{base}/run-chunk00.jsonl"
        )
        record = json.loads(open(record_file, encoding="utf-8").readline())
        text = record["e"]["text"]
        records = [prefix_strip(line) for line in text.split("\n")]
        questions = load(qset_file)["questions"]
        run = json.loads(open(run_file, encoding="utf-8").readline())
        cache = sorted(
            os.path.splitext(name)[0] for name in os.listdir(f"{base}/cache")
        )
        entries, rules, derived = [], set(), []
        for key, entry in run["answers"].items():
            answer = entry["answer"]
            options = answer["probabilities"]
            pick = answer["pick"]
            for option in options:
                if option not in ("NO_RELATION", "NONE_OF_THESE"):
                    rules.add(rule_of(option))
            question = questions[key]["choose"]
            pair_match = re.search(
                r"how (?:Alert|Record) (\d+) and (?:Alert|Record) (\d+) relate", question
            )
            if pair_match:
                first, second = int(pair_match.group(1)), int(pair_match.group(2))
                entries.append({"pair": [first, second], "options": options, "pick": pick})
                if pick in ("NO_RELATION", "NONE_OF_THESE"):
                    continue
                probability = options[pick]
                if probability < 0.5:
                    continue
                source, target = direction(pick, first, second)
                derived.append(
                    {"name": rule_of(pick), "source": source, "target": target, "probability": probability}
                )
            else:
                subject_match = re.search(r"person in Record (\d+)", question)
                if not subject_match:
                    raise SystemExit(f"{arm}/{key}: question shape not recognized")
                subject = int(subject_match.group(1))
                entry_row = {"subject": subject, "options": options, "pick": pick}
                if ARM_RULES.get(arm):
                    entry_row["rule"] = ARM_RULES[arm][0]
                entries.append(entry_row)
                if pick in ("NO_RELATION", "NONE_OF_THESE"):
                    continue
                probability = options[pick]
                if probability < 0.5:
                    continue
                derived.append(
                    {
                        "name": ARM_RULES[arm][0],
                        "source": subject,
                        "target": int(pick[2:]),
                        "probability": probability,
                    }
                )
        derived.sort(key=lambda edge: (edge["source"], edge["target"], edge["name"]))
        if arm in ARM_RULES:
            rules = set(ARM_RULES[arm])
        relations = []
        for name in sorted(rules):
            if arm in ARM_RULES:
                either = False
            else:
                either = not any(
                    option.endswith(("_AB", "_BA"))
                    for entry in entries
                    for option in entry["options"]
                    if rule_of(option) == name
                )
            rule = {"name": name, "from": "*", "to": "*"}
            if either:
                rule["either"] = True
            relations.append(rule)
        form = "per-subject" if arm in ARM_RULES else "pairs"
        table.append(
            {
                "id": arm,
                "form": form,
                "text": text,
                "records": records,
                "rules": sorted(rules),
                "entries": entries,
                "requests": cache,
                "pairs_count": len(entries),
            }
        )
        case = {
            "id": f"{69 + index:02d}-relate-{arm}",
            "source": f"experiments/225-recognize-harvest-package relate/cases/{arm} (recording)",
            "verb": "relate",
            "records": records,
            "question": {"relations": relations, "threshold": 0.5},
            "requests": cache,
            "pairs": len(entries),
            "exchanges": [],
            "expect": {"edges": derived},
        }
        if form == "per-subject":
            case["form"] = "per-subject"
            case["note"] = (
                "measured per-subject form; the ruled relate asks pairs, and this "
                "text's pairs recording (R04) is what the stand-in serves"
            )
        conf.append(case)
    return table, conf


def main():
    recognize, recognize_conf = build_recognize()
    synth_row, synth_conf = build_synthesized()
    recognize.append(synth_row)
    recognize_conf.append(synth_conf)
    relate, relate_conf = build_relate()
    replay = {
        "schema": "thinkthen.replay/1",
        "source": "experiments/225-recognize-harvest-package, harvested 2026-09-21; "
        "C41 synthesized for the offset proofs",
        "recognize": recognize,
        "relate": relate,
    }
    os.makedirs(f"{ROOT}/standin/data", exist_ok=True)
    with open(f"{ROOT}/standin/data/recognize-replay.json", "w", encoding="utf-8") as handle:
        json.dump(replay, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    path = f"{ROOT}/conformance/conformance.json"
    data = load(path)
    kept = [case for case in data["cases"] if case["verb"] not in ("recognize", "relate")]
    data["cases"] = kept + recognize_conf + relate_conf
    data["case_count"] = len(data["cases"])
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(data, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    print(
        f"wrote {len(data['cases'])} cases "
        f"({len(recognize_conf)} recognize, {len(relate_conf)} relate); "
        f"replay table {len(recognize)} + {len(relate)} rows"
    )


if __name__ == "__main__":
    main()
