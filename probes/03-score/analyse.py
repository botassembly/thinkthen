"""Read probe 3: how well `score` places a text, beside `choose` over the same
five labels on the same texts.

jq has no rank correlation and no square root worth writing twice, so this
short script sits beside the probe. Everything else it reports could be jq.

    python3 analyse.py [ROWS_DIR]

Counts are exact. Rates and correlations are rounded to four decimals.
"""

import json
import math
import pathlib
import sys

LEVELS = ["none", "minor", "moderate", "major", "total"]


def rows(path):
    with open(path, encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]


def rounded(number):
    """Round half away from zero, which is what a reader expects of a level."""
    return int(math.floor(number + 0.5))


def spearman(left, right):
    """The rank correlation of two lists, with ties given the average rank."""

    def ranks(values):
        order = sorted(range(len(values)), key=lambda i: values[i])
        out = [0.0] * len(values)
        place = 0
        while place < len(order):
            stop = place
            while stop + 1 < len(order) and values[order[stop + 1]] == values[order[place]]:
                stop += 1
            average = (place + stop) / 2 + 1
            for i in range(place, stop + 1):
                out[order[i]] = average
            place = stop + 1
        return out

    a, b = ranks(left), ranks(right)
    n = len(a)
    mean_a, mean_b = sum(a) / n, sum(b) / n
    cov = sum((x - mean_a) * (y - mean_b) for x, y in zip(a, b))
    var_a = math.sqrt(sum((x - mean_a) ** 2 for x in a))
    var_b = math.sqrt(sum((y - mean_b) ** 2 for y in b))
    if var_a == 0 or var_b == 0:
        return None
    return round(cov / (var_a * var_b), 4)


def rate(part, whole):
    return round(part / whole, 4) if whole else None


def main():
    here = pathlib.Path(__file__).parent
    folder = here / (sys.argv[1] if len(sys.argv) > 1 else "runs")
    score_rows = rows(folder / "score.jsonl")
    choose_rows = rows(folder / "choose.jsonl")

    truth = [row["input"]["level"] for row in score_rows]
    hard = [row["input"]["hard"] for row in score_rows]
    ids = [row["input"]["id"] for row in score_rows]
    assert [row["input"]["id"] for row in choose_rows] == ids, "the two runs differ in their cases"

    weighted = [row["value"] for row in score_rows]
    score_pick = [LEVELS.index(row["answer"]["level"]) for row in score_rows]
    # A choose row with no threshold always names a pick, and `value` equals it.
    choose_pick = [LEVELS.index(row["answer"]["pick"]) for row in choose_rows]

    def agreement(guess, name):
        exact = sum(1 for g, t in zip(guess, truth) if g == t)
        within = sum(1 for g, t in zip(guess, truth) if abs(g - t) <= 1)
        exact_hard = sum(1 for g, t, h in zip(guess, truth, hard) if h and g == t)
        off = [
            {"id": i, "truth": LEVELS[t], "said": LEVELS[g], "hard": h}
            for i, g, t, h in zip(ids, guess, truth, hard)
            if g != t
        ]
        return {
            "name": name,
            "rows": len(truth),
            "exact": exact,
            "exact_rate": rate(exact, len(truth)),
            "within_one": within,
            "within_one_rate": rate(within, len(truth)),
            "exact_hard": exact_hard,
            "hard_rows": sum(1 for h in hard if h),
            "disagreements": off,
        }

    report = {
        "levels": LEVELS,
        "score_rounded": agreement([rounded(v) for v in weighted], "score, value rounded"),
        "score_top_level": agreement(score_pick, "score, the level with the most probability"),
        "choose": agreement(choose_pick, "choose over the same five labels"),
        "spearman": {
            "score_value_against_truth": spearman(weighted, truth),
            "score_top_level_against_truth": spearman(score_pick, truth),
            "choose_pick_against_truth": spearman(choose_pick, truth),
        },
        "mean_absolute_error": {
            "score_value": round(sum(abs(v - t) for v, t in zip(weighted, truth)) / len(truth), 4),
            "choose_pick": round(
                sum(abs(c - t) for c, t in zip(choose_pick, truth)) / len(truth), 4
            ),
        },
        "the_two_verbs_agree": sum(1 for s, c in zip(score_pick, choose_pick) if s == c),
        "split_distributions": [
            {"id": i, "level": row["answer"]["level"], "probabilities": row["answer"]["probabilities"]}
            for i, row in zip(ids, score_rows)
            if max(row["answer"]["probabilities"].values()) < 0.6
        ],
    }
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
