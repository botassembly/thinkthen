---
flow: build
priority: 42
opens: transforms demos/25-check-the-judge sdlc/planning
---

# 0047: Monitor actions and reviewer overturns

Status: proposed

## Outcome

One local transform reads saved policy audit rows and reports how often each action ran, how much of each action a person reviewed, and how often the person changed it. The review field is `input.reviewed_action`, the same public word page 16 uses for a person's recorded decision.

## Current facts and decisions

The triage reference says a monitor counts actions and reviewer overturns. Page 16 fixes the policy actions as `draft`, `block`, and `review`, and calls the person's decision `reviewed_action`. ADR 0016 says watching a pipeline belongs on page 25 as the same human-label check over last week's reviewed rows. A review-only queue cannot reveal quiet mistakes in automated actions, so the report must show review coverage for each action.

This ticket makes these decisions. Ian can overturn them:

1. `transforms/monitor/monitor.jq` reads JSONL audit rows after a policy transform. Run it as `jq -n -f monitor.jq ROWS`. It takes no argument and makes no request.
2. Every row has an object `input` with a unique string `id` and an object `policy` whose `action` is `draft`, `block`, or `review`. `input.reviewed_action` may be absent or null while no person has decided. A non-null reviewed action must use the same three words. Duplicate ids and malformed fields fail with fixed messages that echo no row data.
3. The report has `rows`, `reviewed`, `unreviewed`, `review_coverage`, `agreed`, `overturned`, `agreement_rate`, `actions`, and `changes`, in that order. `agreed` and `overturned` are counts of reviewed rows. `review_coverage` divides reviewed by all rows, and `agreement_rate` divides agreed by reviewed. Rates round to four decimals and are null when their denominator is zero.
4. `actions` is an object with `draft`, `block`, and `review` in that order. Each holds `rows`, `reviewed`, `review_coverage`, `agreed`, `overturned`, and `agreement_rate` with the same denominators inside that policy action. This makes a monitor that reviewed only the review queue visibly incomplete for automated draft and block actions.
5. `changes` lists each reviewed disagreement in input order as `{id,action,reviewed_action}`. The report contains no message body, subject, question, probability, or answer detail.
6. Empty input succeeds with zero counts, null rates, the three zeroed action entries, and an empty changes list. Non-null ordinary input, including `jq -s`, fails once with a fixed message naming `jq -n`.
7. Sampling stays policy outside this transform. Page 25 says to review the uncertain queue and a small sample of automated draft and block rows. The monitor shows review coverage for each action, but it cannot prove that the rows were selected randomly. It does not invent randomness or choose work for a person.

## Scope

Build the monitor red-green, add its focused test and executable transform example, and add one compact monitoring check to page 25 while keeping that page within its current limits. Reuse page 16's fictional rows and triage policy as an outside check. Make no live call.

Excluded: selecting a review sample, state or history, time windows, alerts, thresholds for alerts, free-form action vocabularies, question-level human labels, grouped sweeps, and product code.

## Acceptance

- Synthetic rows prove all three policy actions, reviewed and unreviewed rows, and the disagreement cycle `draft` to `block`, `block` to `review`, and `review` to `draft` in that input order. They pin top-level and per-action denominators, zero denominators, exact four-decimal rates, change order, and empty input.
- Missing, null, and valid `reviewed_action` values behave as specified. Duplicate ids, non-string ids, unknown policy or reviewed actions, malformed containers, and non-null ordinary input fail once with exact data-free diagnostics.
- The page-16 rows after `triage.jq` report six rows, two drafts, one block, three reviews, full review coverage, six agreed rows, no overturns, an agreement rate of 1, and no changes.
- The executable transform page and page 25 run without a key or network. Page 25 stays at no more than 120 lines and 900 words and uses `reviewed_action` for the person's decision everywhere.
- The focused test, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0046.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: one pure jq summary reads one fixed three-action vocabulary and changes one existing page. It adds no state, network, dependency, product surface, or generic monitor language.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the transform gains stored history, alerting, sampling, or configurable actions.

## Review

The independent design review rejected the first draft because `agreement` did not say whether it was a count or a rate, review coverage cannot prove random sampling, and a score of six routes to level 3 rather than level 2. Its second pass required an exact three-action disagreement cycle instead of the phrase "each direction." The corrected report names `agreed` and `overturned` as counts and `agreement_rate` as the rate over reviewed rows. It states the sampling limit plainly, pins every source and target action in order, and uses the level-3 Sol Medium route.
