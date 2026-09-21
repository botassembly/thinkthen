---
flow: build
priority: 43
opens: transforms sdlc/planning
---

# 0046: Average repeated trials once per case

Status: proposed

## Outcome

A user can concatenate repeated detailed runs, average each case's stored probabilities, and feed one derived row per case to the existing metric transforms. A case tried five times has the same weight as a case tried once. The metric transforms refuse duplicate case ids instead of silently giving repeated cases extra weight.

## Current facts and decisions

ADR 0008 says rows with one id are repeated trials and metrics average within a case first. The roadmap says a shell loop produces the trials and a transform averages them. The open review item from ticket 0008 names five transforms that currently count duplicate ids twice: `counts`, `score`, `sweep`, `band`, and `calibration`. The read-only fixture in `experiments/212-thinkthen-repeat/` has 100 public SMS cases judged three times with one unchanged yes-or-no question. Its answers differ enough to prove that averaging is real work.

This ticket makes these decisions. Ian can overturn them:

1. `transforms/trials/trials.jq` reads detailed scalar `decide`, `choose`, or `score` rows and writes one derived JSONL row per case id, in the order each id first appeared. It accepts rows from several files in one invocation. `tag`, `annotate`, and `find` wait for grouped transforms because their output needs another grouping rule.
2. Trials with one id must carry the same `input`, `question`, `threshold`, schema, tool version, question digest, URL, model, and answer kind. A difference is a fixed failure. Usage and replay state may differ because each trial is a separate observation.
3. A derived row has schema `thinkthen.trials/1`, the shared input, question, and threshold, an averaged answer, and a recomputed value. Its `trials` object says how many rows were averaged and how many were live or replayed. Its `meta` keeps the shared tool, question digest, URL, and model. It carries no usage total because it is a metric input rather than a bill.
4. Yes-or-no averages the probability of yes, then reapplies the shared cut or band. Choice averages each option probability, chooses the first option at the maximum, and reapplies its cut; an exact tie stays unresolved. Score averages each level probability and recomputes the weighted position. Means are not rounded.
5. A row with a unique id still becomes a one-trial derived row. This gives one explicit preparation step for every metric run instead of behavior that changes only when a duplicate happens.
6. `counts`, `score`, `sweep`, `band`, and `calibration` accept derived rows and keep their reports unchanged. Raw input with a repeated id fails with a fixed sentence that says to run `trials.jq` first. `compare` keeps listing repeated ids, and `cost` keeps counting every paid trial.

## Scope

Build the trials transform red-green, add one focused test, add duplicate-id guards to the five metric transforms, and add one executable example to `transforms/README.md`. Use the three-run experiment as a read-only outside check. Do not copy its SMS text into the repository and make no live call.

Excluded: a command flag that repeats calls, pairing trials between two comparison runs, tag and annotate question grouping, find, confidence, statistical intervals, significance claims, and changes to product code.

## Acceptance

- Synthetic yes-or-no rows prove first-id order, one and several trials, exact mean probability, cut and band boundaries, changed values after averaging, live and replay counts, and no usage field.
- Synthetic choice rows prove per-option means, option order, the first maximum, an exact tie, and a cut above the averaged leader. Synthetic score rows prove per-level means and the recomputed weighted value.
- Missing or repeated ids, mixed kinds, malformed probabilities, inconsistent case/question/run facts, unsupported kinds, and malformed containers fail once with fixed diagnostics that echo no row data.
- Each of the five metric transforms refuses a raw duplicate id and accepts the unique derived rows without changing its existing report shape. Existing unique-run fixtures remain byte-for-byte unchanged.
- The read-only three-run fixture produces 100 derived rows from 300 observations. At a 0.5 cut the means give 11 true positives, 10 false positives, 74 true negatives, and 5 false negatives: accuracy 0.85, precision 0.5238, recall 0.6875, and F1 0.5946. Its largest within-case probability range is 0.08, subject only to ordinary floating representation.
- The focused test, executable transform page, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0045.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 2
- Proof: 3
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: one new public transform normalizes three answer shapes and five existing metric transforms gain a refusal. It adds no network, process, product surface, dependency, or durable state.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the implementation changes a metric report, adds another answer kind, or needs product code.
