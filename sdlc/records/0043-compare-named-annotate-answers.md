# 0043: Compare named `annotate` answers

Date: 2026-09-21

Status: landed

## Result

`compare.jq` now pairs two detailed `annotate` runs by record id and compares each shared named answer independently. The report lists added and removed question names, record-level exclusions, question and threshold changes, exact value changes for decide, choose, tag, and score, and the existing yes-or-no probability summary for each decision question. Scalar reports keep their ticket 0042 shape and values.

The transform now requires `jq -n`. A nonempty ordinary input or `jq -s` stops after one fixed `compare: run with jq -n` message. Jq invokes no filter for empty ordinary input, so an empty stream still prints nothing. The header, ticket, and transform page state that limit.

## Proof and review

The focused test started red on the missing annotation report and the `jq -s` trap. It covers all six decision flip directions, tolerance edges, choice and score changes, ordered tag arrays, question changes, one-sided names, record exclusions, empty runs, mixed shapes, and fixed diagnostics that echo no hostile row data. How-to 14 replays the grading fixture twice and compares the `correct` answer without a network call.

Design review fixed the exact report keys, empty-run mode, per-run question-name consistency, malformed-row boundary, and per-question `compared` count before implementation. Code review then found that absent nested `value` and `threshold` members could pass as null, and that ordinary multi-row input repeated the refusal. The repair requires both members and uses one fail-fast literal. The same reviewer accepted the corrected tree with no remaining blocker.

The focused test, `install`, `lint`, `test`, `spec`, and `git diff --check` pass. The specification rung reports 26 specification checks, two transform checks, and 19 green how-tos with none red. No live or paid call ran.
