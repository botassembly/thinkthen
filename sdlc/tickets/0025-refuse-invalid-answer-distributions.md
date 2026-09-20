---
flow: build
priority: 89
opens: crates/thinkthen-core/src/answer.rs crates/thinkthen-core/src/adapters/systemone specification/backends.md specification/score.md sdlc/ratchet.json
---

# 0025: Refuse invalid answer distributions

Status: ready

## Outcome

`choose` and `score` accept only a complete probability distribution over the labels the question sent. A malformed backend reply is exit 4. An accepted score stays inside its documented scale.

## Current Facts

The decoder validates each member and requires every requested label, but it neither checks the total nor refuses extra labels. Three score members of `1.0` are accepted and produce `3.0` on a scale whose valid positions are 0 through 2. ADR 0019 defines exact label membership and a total within `member count × f64::EPSILON`. All 497 saved choice and score probability objects fit; the largest measured distance from one is `1.1102230246251565e-16`.

## Scope

- Make the backend-neutral `Distribution` refuse members whose total differs from one by more than `member count × f64::EPSILON`.
- Make the `systemone` adapter refuse a probability key that does not name an option or level the question sent.
- Preserve the reported members when the total is within tolerance. Compute a score from the weighted sum divided by the measured total, then keep its existing rounding, without rewriting the reported distribution.
- Add one decode error that names the question and the distribution rule without quoting a reply value.
- Amend `backends.md` to the validation rule of ADR 0019. Amend `score.md` so its formula divides the weighted sum by the measured total before the existing twelve-decimal rounding.

Excluded: changing the confidence field, yes/no answers, thresholds, recording identity, or request bytes.

## Acceptance

- Focused core tests first accept the current bad choice and score totals, then fail when they require refusal.
- Table cases refuse totals below and above one, including three members of `1.0`, for both `choose` and `score`. Focused cases pin values immediately inside and outside the derived tolerance.
- Cases accept exact one and the largest floating-point remainder already present in saved replies, preserving every member as reported.
- A case refuses an extra choice label and an extra numbered score level. Missing labels are checked first and keep their current error; out-of-range members keep their current error.
- A case pins the complete new error sentence and places a sentinel label in the reply, then proves the sentence does not contain it or any reply value.
- Boundary cases keep every accepted score from zero through the highest level position. Every committed recording replays, the source ceiling equals the measured total, and the full ladder passes.
- The two specification pages state the validation and score arithmetic that the tests enforce.

## Dependencies

ADR 0019, decided with this ticket. No code ticket.

## Complexity

- Contract score: 2
- State and timing score: 0
- Reach score: 1
- Proof score: 1
- Cost of error score: 1
- Total: 5
- Minimum level floor: none
- Final level: 2
- Reasons: the tolerance settles a previously unstated compatibility rule; the invariant spans the generic answer type and one adapter; choice, score, both sides of the tolerance, and exact wire labels need proof; accepting bad data produces wrong public results.
- Selected model: `gpt-5.6-luna` with high reasoning

## Review

- Design review: accepted after two corrections. The reviewer required a derived tolerance and bounded score arithmetic, then required `score.md` to state that arithmetic. It confirmed the ownership, compatibility evidence, acceptance cases, and level 2 route.
- Code review: pending
