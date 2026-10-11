# 0542: Review all 0.2 code after the last migration lands

Status: OPEN.

Milestone: 0.2

Depends on: 0455
Depends on: 0530
Depends on: 0534
Depends on: 0535
Depends on: 0536
Depends on: 0537
Depends on: 0538
Depends on: 0539
Depends on: 0540
Depends on: 0541

## Outcome

A fresh reviewer accepts every code change that landed after the 0508 review, and every finding is fixed or filed. On the final commit, `sdlc/scripts/lint` and `sdlc/scripts/test` each pass in one uninterrupted run.

## Evidence

- Starts from: gap 9 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). 0508 landed at 38fa0cdfa before 19 of its 25 dependencies closed. Ruby's commits `2c71db0db` to `d97af8365` and six 0529 release-script commits (`6449c7d9b`, `20140fd29`, `9d611c558`, `bf8c02c0a`, `bf4a6318b`, `a7a2d8f6b`) have no review on their tickets.
- Keeps: the review rules in [the 2026-10-10 ruling](../decisions/2026-10-10-drive-0-2-to-done.md). Reviews cover code only.
- Changes: review the diff from 38fa0cdfa to the final commit by area: Rust core, each language family, databases, release scripts. Fix confirmed defects in this ticket. File a new ticket only for a defect that needs its own design.
- Proof: one accept per area on the final commit, and the lint and test logs of the single runs.
- Defers: the candidate runs belong to 0543.
