---
flow: build
priority: 47
opens: transforms/compare transforms/README.md demos/41-tune-a-question-file sdlc/planning
---

# 0042: Compare scalar values and honest probability movement

Status: in progress

## Outcome

`compare.jq` compares saved `decide`, `choose`, and `score` runs without hiding answer changes or filling an honest repeated-run report with small yes-or-no probability movements. Existing comparison fields keep their meanings for `decide` rows.

## Current facts and decisions

ADR 0014 item 6 requires `compare` to read any scalar `value` before the project judges whether a report command earns its cost. The transform currently refuses strings and numbers. The repeat measurement in `experiments/212-thinkthen-repeat/` compared one yes-or-no question over 100 public messages twice under model `jev-1.13.0`: 63 probabilities moved, by at most 0.08, and four answers flipped. That evidence covers one question kind, model, set, and day. It does not set a probability rule for choice or score.

This ticket makes these decisions. Ian can overturn them:

1. A comparable `value` may be null, boolean, string, or number. `same` counts exact scalar equality. `changed_values` counts comparable pairs whose values differ, and `compared == same + changed_values`.
2. `changes` is ordered lexically by id. It lists every changed scalar value as `id`, `before_value`, and `after_value`. It also lists a same-value yes-or-no pair when its probability movement exceeds the active tolerance. The existing `flips` object remains the boolean-or-null grouping and keeps its `yes`, `no`, and `unresolved` words.
3. Probability comparison applies only when both rows carry a valid `answer.kind` of `yes_no` and a numeric `answer.probability` from zero through one. Such a change entry also carries `before_probability`, `after_probability`, and `probability_delta`. Choice and score gain scalar comparison and no invented probability.
4. `--argjson probability_tolerance N` is optional. The transform reads it through `$ARGS.named.probability_tolerance`. An absent name selects 0.08; an explicitly supplied `null` is invalid. The argument accepts a number from zero through one and otherwise fails with `compare: probability_tolerance must be a number from 0 through 1`. Zero lists every nonzero same-answer probability movement. `yes_no_probability` has the exact fields `tolerance`, `compared`, `changed`, `summarized_same_value`, and `largest_summarized_delta`. `changed` counts valid yes-or-no pairs with a nonzero probability delta.
5. Every value change stays visible at every probability delta. Missing `answer` on a legacy row makes probability unavailable. A present malformed `yes_no` answer fails safely. Calculated deltas lose binary display residue at twelve decimal places, while the before and after probabilities remain unchanged.

## Scope

Add one focused transform test to the test rung, update `compare.jq`, its header and example, the executable transform page, and how-to 41. The how-to states that a fair pair is two live runs or two replays, and that 0.08 is one limited yes-or-no measurement rather than a promise. Keep it within 120 lines and 900 words.

Use experiment 212 as a read-only outside check. Do not copy its message text into this repository and make no paid call.

Excluded: choice or score probability movement, arrays and objects such as `tag` and `annotate` values, repeated-trial averaging, sweep, page 16 polish, the cost transform repair, monitors, grouped sweep, and the human-label check.

## Acceptance

- A red-green focused test covers null, boolean, string, and number values; every boolean and unresolved direction; exact lexical id order; and the partition `compared == same + changed_values`.
- Existing duplicate-id, missing-row, changed-input, changed-label, changed-question, model, threshold, legacy, and empty-run behavior stays pinned.
- A yes-or-no value change appears below, at, and above 0.08. A same-value delta of 0.08 is summarized, 0.09 is listed, and tolerance zero lists every movement. The active tolerance appears in output, and invalid tolerances earn the fixed safe error.
- Legacy rows with no answer still compare. Present malformed yes-or-no answers fail without echoing their contents. Choice and score value changes work without probability fields. Missing, array, and object values fail safely.
- The outside repeat check reports 100 comparable rows, 96 same values, four changed values and existing flips, and `yes_no_probability` with `compared: 100`, `changed: 63`, `summarized_same_value: 59`, and `largest_summarized_delta: 0.08`. No same-value movement exceeds the default.
- How-to 41 demonstrates the new fields, explains the fair-pair rule and evidence limit, and passes its page limits. The transform page, focused test, all four repository rungs, and `git diff --check` pass.

## Dependencies

None.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: the transform gains a small public result surface and must preserve old boolean reports while proving three scalar kinds, legacy rows, malformed-row failures, the optional argument, and a measured yes-or-no tolerance. It has no state, network, concurrency, or durable migration.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation needs a non-scalar value, a probability rule for choice or score, or a change outside saved-row transforms and their pages.

## Review

- Design review: accepted after one rejection. The first draft understated compatibility proof, left the summary field names implicit, and would have let an explicit null look absent. The corrected ticket raises the route to level 3, fixes every summary name and count, and distinguishes an omitted argument from invalid null.
- Code review: pending
