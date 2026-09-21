# 0047: Monitor actions and reviewer overturns

Date: 2026-09-21

Status: landed

## Result

`monitor.jq` reads saved policy audit rows and reports action volume, human review coverage, agreement, and overturns overall and within `draft`, `block`, and `review`. A missing or null `input.reviewed_action` remains unreviewed. Reviewed disagreements appear in input order with only id, policy action, and the person's action.

The report makes counts and rates explicit. `agreed` and `overturned` are counts. `review_coverage` divides reviewed rows by all rows, and `agreement_rate` divides agreements by reviewed rows. A zero denominator yields null. Per-action coverage exposes a monitor that reviewed only the uncertain queue, while the page states that coverage cannot prove random sampling.

## Proof and review

The focused test byte-pins report key order, action order, every overall and per-action count and rate, four-decimal rounding, null denominators, empty input, absent and null reviews, the `draft` to `block`, `block` to `review`, and `review` to `draft` change order, fixed safe failures, duplicate ids, and the `jq -n` guard.

The page-16 outside rows report six cases, two drafts, one block, three reviews, full coverage, six agreements, and no changes. Page 25 runs the monitor beside the existing human-label checks and remains within its limits at 109 lines and 775 words.

Design review made the count and rate names, sampling claim, implementation route, and disagreement proof exact before code. The independent code reviewer accepted the implementation with no finding after running the focused test, example, page blocks, lint, full test rung, and diff check.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass. No live or paid call ran.
