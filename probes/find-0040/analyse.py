#!/usr/bin/env python3
"""Summarize the safe derived facts and enforce ticket 0040's probe gates."""

import json
import math
import pathlib
import sys


def rows(path):
    return [json.loads(line) for line in pathlib.Path(path).read_text().splitlines() if line]


def safe_sum(held, field):
    values = [row[field] for row in held]
    if any(isinstance(value, bool) or not isinstance(value, int) or value < 0 for value in values):
        raise ValueError(f"{field} is not a safe count")
    return sum(values)


def traffic(held, logical_field=None):
    logical = len(held) if logical_field is None else safe_sum(held, logical_field)
    replayed = sum(row["replayed"] for row in held) if logical_field is None else safe_sum(held, "replayed_judgments")
    live = logical - replayed if logical_field is None else safe_sum(held, "live_requests")
    billed = sum(row["input_tokens"] for row in held if not row["replayed"]) if logical_field is None else safe_sum(held, "billed_input_tokens")
    if live < 0 or replayed < 0 or live + replayed != logical:
        raise ValueError("live and replayed counts do not add to logical judgments")
    return {
        "logical_judgments": logical,
        "live_backend_requests": live,
        "replayed_judgments": replayed,
        "billed_input_tokens": billed,
    }


def hits_by_size(held):
    return {
        str(size): {
            "judgments": len(part),
            "hits": sum(row["value"] == row["trusted"] for row in part),
        }
        for size in sorted({row["size"] for row in held})
        for part in ([row for row in held if row["size"] == size],)
    }


def find_facts(found):
    answerable = [row for row in found if row["trusted"] is not None]
    blank_none = [row for row in found if row["trusted"] is None and row["none"]]
    by_policy = {}
    for policy in (False, True):
        arm = [row for row in answerable if row["none"] is policy]
        by_policy["with_none" if policy else "without_none"] = {
            "answerable": len(arm),
            "hits": sum(row["value"] == row["trusted"] for row in arm),
            "hits_by_size": hits_by_size(arm),
            **traffic(arm),
        }
    return {
        "arms": by_policy,
        "blank_with_none": len(blank_none),
        "blank_none_hits": sum(row["value"] == "none" for row in blank_none),
        "false_none": sum(row["value"] == "none" for row in answerable if row["none"]),
        "find_traffic": traffic(found),
    }


def require_find_gate(facts):
    if any(arm["answerable"] != 6 or arm["hits"] < 5 for arm in facts["arms"].values()):
        raise ValueError("an answerable find arm did not hit at least 5 of 6")
    if facts["blank_with_none"] != 3 or facts["blank_none_hits"] != 3:
        raise ValueError("find --none did not answer none on all 3 blank documents")
    if facts["false_none"] != 0:
        raise ValueError("find --none falsely refused an answerable document")


def feasibility(path):
    found = rows(path)
    expected = {
        ("s100-a1", False), ("s100-b1", True),
        ("s175-a1", False), ("s175-b1", True),
        ("s250-a1", False), ("s250-b1", True),
        ("b255-a1", False), ("b254-b1", True),
    }
    observed = {(row["case"], row["none"]) for row in found}
    if len(found) != 8 or observed != expected:
        raise ValueError("feasibility did not complete its eight fixed find calls")
    summary = {
        "stage": "feasibility",
        **traffic(found),
        "accepted": 8,
        "unit_appearances": sum(row["size"] for row in found),
        "billed_unit_appearances": sum(row["size"] for row in found if not row["replayed"]),
        "billed_input_tokens": sum(row["input_tokens"] for row in found if not row["replayed"]),
        "largest_without_none": max(row["size"] for row in found if not row["none"]),
        "largest_with_none": max(row["size"] for row in found if row["none"]),
    }
    print(json.dumps(summary, separators=(",", ":")))


def preliminary(path):
    found = rows(path)
    if len(found) != 18:
        raise ValueError("comparison did not complete its 18 fixed find calls")
    facts = find_facts(found)
    require_find_gate(facts)
    print(json.dumps({"stage": "find_gate", **facts}, separators=(",", ":")))


def comparison(find_path, rank_path):
    found = rows(find_path)
    ranked = rows(rank_path)
    facts = find_facts(found)
    require_find_gate(facts)
    if len(ranked) != 6:
        raise ValueError("comparison did not complete its six rank runs")
    rank_hits = sum(row["value"] == row["trusted"] for row in ranked)
    if any(arm["hits"] < rank_hits for arm in facts["arms"].values()):
        raise ValueError("an answerable find arm trailed rank")
    wrong = [row for row in found if row["trusted"] is not None and row["value"] != row["trusted"]]
    right = [row for row in found if row["trusted"] is not None and row["value"] == row["trusted"]]
    wrong_high = max((row["winning_probability"] for row in wrong), default=None)
    right_low = min((row["winning_probability"] for row in right), default=None)
    summary = {
        "stage": "comparison",
        **facts,
        "rank": {
            "hits": rank_hits,
            "hits_by_size": hits_by_size(ranked),
            **traffic(ranked, "logical_judgments"),
        },
        "wrong_winning_probability_max": wrong_high,
        "right_winning_probability_min": right_low,
        "floor_separates_observed_errors": wrong_high is not None and right_low is not None and wrong_high < right_low,
    }
    print(json.dumps(summary, separators=(",", ":")))


def reservation(path):
    measured = json.loads(pathlib.Path(path).read_text())
    if measured.get("stage") != "feasibility" or measured.get("accepted") != 8:
        raise ValueError("a complete feasibility summary is required")
    observed_rate = measured["billed_input_tokens"] / measured["billed_unit_appearances"]
    old_find_rate = 11183 / 239
    rank_estimate = 69143 / 239 * 1050
    estimate = rank_estimate + max(observed_rate, old_find_rate) * 3150
    reserved = math.ceil(estimate * 1.15 / 1000) * 1000
    if reserved > 525000:
        raise ValueError("the derived comparison reservation exceeds 525000")
    print(json.dumps({
        "observed_find_tokens_per_unit": observed_rate,
        "estimated_comparison_tokens": estimate,
        "comparison_max_tokens": reserved,
    }, separators=(",", ":")))


def main():
    try:
        if len(sys.argv) == 3 and sys.argv[1] == "feasibility":
            feasibility(sys.argv[2])
        elif len(sys.argv) == 3 and sys.argv[1] == "find-gate":
            preliminary(sys.argv[2])
        elif len(sys.argv) == 4 and sys.argv[1] == "comparison":
            comparison(sys.argv[2], sys.argv[3])
        elif len(sys.argv) == 3 and sys.argv[1] == "reservation":
            reservation(sys.argv[2])
        else:
            raise ValueError("give feasibility, find-gate, comparison, or reservation inputs")
    except (KeyError, TypeError, ValueError, ZeroDivisionError) as error:
        print(f"find-0040: {error}", file=sys.stderr)
        raise SystemExit(2) from None


if __name__ == "__main__":
    main()
