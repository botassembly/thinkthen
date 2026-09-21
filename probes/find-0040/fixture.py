#!/usr/bin/env python3
"""Construct and verify ticket 0040's deterministic made-up documents."""

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent
CASES = ROOT / "cases.json"


def cases():
    return {case["id"]: case for case in json.loads(CASES.read_text())}


def uid(place):
    return f"u{place:03d}"


def question(case):
    marker = f"TARGET-{case['id'].upper()}"
    return f"Which unit states the cobalt permit duration for marker {marker}?"


def rank_question(case):
    marker = f"TARGET-{case['id'].upper()}"
    return f"This unit states the cobalt permit duration for marker {marker}."


def units(case):
    held = []
    for place in range(1, case["size"] + 1):
        if place == case["answer"]:
            marker = f"TARGET-{case['id'].upper()}"
            held.append(f"The cobalt permit duration for marker {marker} is {20 + place % 37} days.")
        else:
            marker = f"OTHER-{case['id'].upper()}-{place:03d}"
            held.append(
                f"Marker {marker} uses an amber checklist reviewed every {2 + place % 19} days."
            )
    return held


def document(case):
    return [
        {"id": uid(place), "evidence": text}
        for place, text in enumerate(units(case), start=1)
    ]


def expected_options(case, with_none):
    options = [uid(place) for place in range(1, case["size"] + 1)]
    if with_none:
        options.append("none")
    return options


def load_result(path):
    return json.loads(pathlib.Path(path).read_text())


def find_row(case, with_none, result):
    options = expected_options(case, with_none)
    if result.get("question", {}).get("options") != options:
        raise ValueError("find result options differ from the planned request")
    probabilities = result.get("answer", {}).get("probabilities", {})
    if result.get("answer", {}).get("kind") != "choice" or list(probabilities) != options:
        raise ValueError("find result does not carry every planned choice in order")
    if any(isinstance(value, bool) or not isinstance(value, (int, float))
           for value in probabilities.values()):
        raise ValueError("find result probabilities are not numbers")
    winning = max(probabilities.values())
    leaders = [option for option in options if probabilities[option] == winning]
    value = "none" if "none" in leaders else leaders[0]
    meta = result.get("meta", {})
    replayed = meta.get("replayed")
    input_tokens = meta.get("usage", {}).get("input_tokens")
    if not isinstance(replayed, bool):
        raise ValueError("find result does not say whether it was replayed")
    if isinstance(input_tokens, bool) or not isinstance(input_tokens, int) or input_tokens < 0:
        raise ValueError("find result does not carry safe input usage")
    return {
        "case": case["id"],
        "size": case["size"],
        "trusted": None if case["answer"] is None else uid(case["answer"]),
        "none": with_none,
        "value": value,
        "winning_probability": winning,
        "replayed": replayed,
        "input_tokens": input_tokens,
    }


def rank_row(case, results):
    if len(results) != case["size"]:
        raise ValueError("rank did not return every judgment")
    by_id = {}
    live_requests = 0
    replayed_judgments = 0
    billed_input_tokens = 0
    for result in results:
        if result.get("answer", {}).get("kind") != "yes_no":
            raise ValueError("rank result is not a yes/no judgment")
        row_id = result.get("input", {}).get("id")
        probability = result.get("answer", {}).get("probability")
        replayed = result.get("meta", {}).get("replayed")
        input_tokens = result.get("meta", {}).get("usage", {}).get("input_tokens")
        if row_id in by_id or row_id not in expected_options(case, False):
            raise ValueError("rank result ids differ from the planned records")
        if isinstance(probability, bool) or not isinstance(probability, (int, float)):
            raise ValueError("rank result does not carry a probability")
        if not isinstance(replayed, bool):
            raise ValueError("rank result does not say whether it was replayed")
        if isinstance(input_tokens, bool) or not isinstance(input_tokens, int) or input_tokens < 0:
            raise ValueError("rank result does not carry safe input usage")
        by_id[row_id] = probability
        if replayed:
            replayed_judgments += 1
        else:
            live_requests += 1
            billed_input_tokens += input_tokens
    ordered = expected_options(case, False)
    value = max(ordered, key=lambda row_id: by_id[row_id])
    return {
        "case": case["id"],
        "size": case["size"],
        "trusted": uid(case["answer"]),
        "value": value,
        "probability": by_id[value],
        "logical_judgments": len(results),
        "live_requests": live_requests,
        "replayed_judgments": replayed_judgments,
        "billed_input_tokens": billed_input_tokens,
    }


def verify(all_cases):
    expected = {
        "s100-a1": (100, 7), "s100-a2": (100, 83), "s100-b1": (100, None),
        "s175-a1": (175, 31), "s175-a2": (175, 149), "s175-b1": (175, None),
        "s250-a1": (250, 63), "s250-a2": (250, 227), "s250-b1": (250, None),
        "b255-a1": (255, 251), "b254-b1": (254, None),
    }
    observed = {name: (case["size"], case["answer"]) for name, case in all_cases.items()}
    if observed != expected:
        raise ValueError("the preregistered case table changed")
    for case in all_cases.values():
        made = document(case)
        if len(made) != case["size"] or [row["id"] for row in made] != expected_options(case, False):
            raise ValueError(f"{case['id']} does not have its fixed ordered units")
        matches = [row for row in made if f"TARGET-{case['id'].upper()}" in row["evidence"]]
        if len(matches) != (case["answer"] is not None):
            raise ValueError(f"{case['id']} does not have its fixed trusted answer")


def main():
    if len(sys.argv) < 2:
        raise SystemExit("fixture.py: give one operation")
    operation = sys.argv[1]
    all_cases = cases()
    if operation == "verify":
        verify(all_cases)
        return
    if len(sys.argv) < 3 or sys.argv[2] not in all_cases:
        raise SystemExit("fixture.py: name a preregistered case")
    case = all_cases[sys.argv[2]]
    if operation == "question":
        print(question(case))
    elif operation == "rank-question":
        print(rank_question(case))
    elif operation == "state":
        sys.stdout.write(json.dumps(document(case), separators=(",", ":")))
    elif operation == "options":
        with_none = len(sys.argv) == 4 and sys.argv[3] == "none"
        print(" ".join(expected_options(case, with_none)))
    elif operation == "rank-input":
        for row in document(case):
            print(json.dumps({"id": row["id"], "text": row["evidence"]}, separators=(",", ":")))
    elif operation == "find-row":
        if len(sys.argv) != 5 or sys.argv[3] not in {"plain", "none"}:
            raise SystemExit("fixture.py: find-row needs plain|none and a result")
        row = find_row(case, sys.argv[3] == "none", load_result(sys.argv[4]))
        print(json.dumps(row, separators=(",", ":")))
    elif operation == "rank-row":
        if len(sys.argv) != 4 or case["answer"] is None:
            raise SystemExit("fixture.py: rank-row needs an answerable case and result")
        results = [json.loads(line) for line in pathlib.Path(sys.argv[3]).read_text().splitlines() if line]
        print(json.dumps(rank_row(case, results), separators=(",", ":")))
    else:
        raise SystemExit(f"fixture.py: unknown operation {operation}")


if __name__ == "__main__":
    main()
