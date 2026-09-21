---
flow: build
priority: 44
opens: transforms/sweep transforms/README.md demos/13-pick-a-threshold sdlc/planning
---

# 0045: Sweep decisions, picks, and scores

Status: in progress

## Outcome

One `sweep.jq` reads detailed `decide`, `choose`, or `score` rows and reports the threshold trade-off that each command actually has. Existing decision sweeps keep their result unchanged. Choice sweeps show accuracy beside coverage at a minimum winning probability. Score sweeps show binary metrics at each meaningful boundary between named levels.

## Current facts and decisions

ADR 0014 item 6 says the sweep must serve the original three judgment verbs before the project judges a report command again. The choice probe already carries the needed rule: cut the winning probability and show accuracy for both covered and refused rows. The score specification says users cut the numeric value locally, but a cut between levels changes what counts as the positive class and has no honest automatic winner.

This ticket makes these decisions. Ian can overturn them:

1. Empty input and `answer.kind: "yes_no"` use the exact ticket 0008 decision JSON value, including its automatic highest-F1 `pick`.
2. A choice report says `mode: "choose"`, reports `rows`, `labeled`, `unlabeled`, and the shared `options`, then gives the 19 cuts from 0.05 through 0.95. Each line has `cut`, `resolved`, `unresolved`, `ties`, `coverage`, `accuracy_resolved`, and `accuracy_unresolved`. The trusted `input.label` must be one of the options or the row is listed as unlabeled and enters no count or rate. `resolved`, `unresolved`, and `ties` count labeled rows; coverage is resolved divided by labeled. A unique leader resolves when its probability reaches the cut. `accuracy_resolved` divides correct resolved picks by resolved. `accuracy_unresolved` divides correct unique leaders below the cut by all unique leaders below the cut. An exact tie stays unresolved at every cut, increments `ties`, and enters neither accuracy because `answer.pick` is only its arbitrary first maximum. No cut is picked automatically because accuracy and coverage are a trade-off.
3. A score report says `mode: "score"`, reports `rows`, `labeled`, `unlabeled`, and the shared `levels`, then sweeps integer cuts 1 through K minus 1. At cut N, a predicted score and a trusted numeric `input.label` are positive at or above N. Each line has `cut`, `accuracy`, `precision`, `recall`, and `f1`. A trusted label must be an integer from zero through K minus 1 or it is listed as unlabeled. No cut is picked because each boundary names a different operational question.
4. One nonempty run has one answer kind and one ordered option or level list. A choice row has object-valued `answer.probabilities` with exactly the option keys and numeric members from zero through one. `answer.pick` is the first option at the maximum probability. A non-null choice `value` equals that pick; null remains valid because the saved run may have used a cut or seen a tie. The sweep trusts the product's already-validated distribution total. A score row has a numeric `value` from zero through K minus 1. It does not inspect the saved score distribution or leader because its sweep reads the weighted value alone. Malformed or mixed fields the sweep reads fail with fixed messages that echo no row data.
5. The probability grid stays at twentieths. Current backend probabilities have two decimal places, so a cut finer than one hundredth adds no resolution to these rows, and neighboring cuts often tie. The page states both facts without treating the observed precision as a backend promise.
6. `tag` waits for the planned grouped sweep because it has one independent probability per label. Named `annotate` answers wait for the same work. `find` stays out because ADR 0014 says its `none` option is the whole rule and it takes no threshold.

## Scope

Write a focused red-green sweep test, refactor shared arithmetic only where it removes duplication, extend `sweep.jq` and its header, add executable choice and score examples to the transform page, and tighten how-to 13's wording about grid precision and ties without adding a fifth step. Use the committed probe rows as read-only outside checks and make no live call.

Excluded: tag and annotate grouping, find, confidence as an alternate choice cut, custom grids, automatic choice or score picks, band and score transform changes, monitors, repeated trials, and a report command.

## Acceptance

- The committed decision fixture produces the same JSON value as ticket 0008, including empty input, the 19 cuts, and its picked cut.
- A synthetic choice fixture proves a unique winner above, at, and below a cut; an exact tie that is counted but enters neither accuracy; valid and unlabeled truth; labeled-only denominators; exact ordered options; every output field; null denominators; and neighboring cuts with identical results.
- The read-only choice probe reports 60 labeled rows, 58 right at cut 0.5 with full coverage, and at cut 0.95 reports 53 resolved, 7 unresolved, 0.8833 coverage, 0.9623 resolved accuracy, and 1.0 refused accuracy.
- A synthetic score fixture proves every level boundary, labels at both endpoints, an unlabeled row, exact metrics, and the absence of an automatic pick. The read-only score probe, after mapping its trusted `input.level` to `input.label`, reports at cut 2: 40 labeled, accuracy 0.925, precision 1, recall 0.875, and F1 0.9333.
- Mixed kinds, inconsistent option or level lists, missing or extra choice probability keys, invalid choice probability members, a stored choice leader other than the first maximum, choice values that disagree with a non-null leader, invalid score values, and malformed fields the sweep reads fail with fixed diagnostics. Hostile ids, labels, option text, level text, and answer fields do not appear in stderr.
- How-to 13 remains within 120 lines and 900 words. The focused test, executable transform page, outside checks, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0044.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 1
- Proof: 3
- Cost of error: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: one public transform gains two type-specific reports while preserving its decision report exactly. The proof must cover three meanings of a cut, malformed rows, numeric edges, and two live-derived outside fixtures. It adds no state, network, or product command.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation changes the decision report, needs a custom grid, or changes another transform.

## Review

The independent design review rejected the first draft because it left choice denominators undefined, could have scored an exact tie through an arbitrary stored leader, and mixed partial validation decisions with broader acceptance claims. The accepted design makes every choice denominator explicit, reports ties while excluding them from both accuracy rates, fully validates the choice fields it reads, and narrows score validation to the weighted value and level list the score sweep uses. The reviewer accepted the trusted product validation boundary, score boundary, probe counts, exclusions, page-limit approach, and level-3 Sol Medium route.
