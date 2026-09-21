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

ADR 0008 says rows with one id are repeated trials and metrics average within a case first. The roadmap says a shell loop produces the trials and a transform averages them. The open review item from ticket 0008 names five transforms that currently count duplicate ids twice: `counts`, `score`, `sweep`, `band`, and `calibration`. The read-only fixture in `experiments/212-thinkthen-repeat/` has two 100-row runs over public SMS cases. The matching 100-row subset of `experiments/206-thinkthen-accuracy/M1/arm1.jsonl` is the third run with the same question. Its answers differ enough to prove that averaging is real work.

This ticket makes these decisions. Ian can overturn them:

1. `transforms/trials/trials.jq` reads detailed scalar `decide`, `choose`, or `score` rows and writes one derived JSONL row per case id, in the order each id first appeared. Duplicate ids form the trials of one case. It accepts rows from several files in one invocation and refuses non-null ordinary input with one fixed message naming `jq -n`. `tag`, `annotate`, and `find` wait for grouped transforms because their output needs another grouping rule.
2. Trials with one id must carry the same `input`, `question`, `threshold`, schema, tool version, question digest, URL, model, and answer kind. A difference is a fixed failure. Usage and replay state may differ because each trial is a separate observation.
3. A derived row has `schema`, `input`, `value`, `question`, `answer`, `threshold`, `trials`, and `meta`, in that order. Its schema is `thinkthen.trials/1`. `trials` is `{count,live,replayed}`, and `count` equals `live + replayed`. `meta` is `{tool,question_sha256,url,model}` and omits both `usage` and `replayed`; the replay counts live under `trials`. The row is a metric input rather than a claim that one backend call made an averaged answer.
4. Yes-or-no averages the probability of yes and writes `answer` as `{kind,probability}`, then reapplies the shared cut or band. Choice averages each option probability in question order and writes `{kind,pick,probabilities}`. It omits backend confidence, recomputes the first maximum as `pick`, resolves a unique leader when no cut applies, reapplies a numeric cut when present, and leaves an exact tie unresolved. Score averages each level probability in question order and writes `{kind,level,probabilities}`. It recomputes `level` as the first maximum and computes `value` as the probability-weighted position divided by the averaged distribution's measured total, rounded to twelve decimal places like the product. Probability means stay unrounded.
5. A row with a unique id still becomes a one-trial derived row. This gives one explicit preparation step for every metric run instead of behavior that changes only when a duplicate happens.
6. `counts`, `score`, `sweep`, `band`, and `calibration` accept derived rows and keep their reports unchanged. Any input with a repeated id, including repeated derived rows, fails with a fixed sentence that says to run `trials.jq` first. `compare` keeps listing repeated ids, and `cost` keeps counting every paid trial.
7. The transform validates only the fields it reads or copies. Each source row must have schema `thinkthen.result/1`; an object `input` with a string `id`; and an object `question` with string `text` and a verb matching the answer kind. A choice has 2 to 255 unique, nonblank, control-free string options in order. A score has 2 to 10 such levels. Its answer has the exact option or level probability keys and numeric probabilities from zero through one. Its threshold has the valid form for that kind. `meta` is an object with string `tool`, `url`, and `model`, a 64-character lowercase hexadecimal `question_sha256`, and boolean `replayed`. Distribution totals use the product's one-hundredth allowance. This is not a second full result-schema validator.

## Scope

Build the trials transform red-green, add one focused test, add duplicate-id guards to the five metric transforms, and add one executable example to `transforms/README.md`. Use the three-run experiment as a read-only outside check. Do not copy its SMS text into the repository and make no live call.

Excluded: a command flag that repeats calls, pairing trials between two comparison runs, tag and annotate question grouping, find, confidence, statistical intervals, significance claims, and changes to product code.

## Acceptance

- Synthetic yes-or-no rows prove first-id order, one and several trials, exact mean probability, cut and band boundaries, changed values after averaging, live and replay counts, and no usage field.
- Synthetic choice rows prove per-option means, option order, the first maximum, an exact tie, and a cut above the averaged leader. Synthetic score rows prove per-level means and the recomputed weighted value.
- Missing or non-string ids, mixed kinds, malformed probabilities, inconsistent duplicate rows, inconsistent case/question/run facts, unsupported kinds, malformed containers, and a non-null ordinary input fail once with fixed diagnostics that echo no row data. Valid duplicate ids become one derived case.
- Each of the five metric transforms refuses any duplicate id and accepts the unique derived rows without changing its existing report shape. Existing unique-run fixtures remain byte-for-byte unchanged.
- The two experiment-212 runs plus their matching subset from experiment 206 produce 100 derived rows from 300 observations. At a 0.5 cut the means give 11 true positives, 10 false positives, 74 true negatives, and 5 false negatives: accuracy 0.85, precision 0.5238, recall 0.6875, and F1 0.5946. The source fixture's largest within-case probability range is 0.08, subject only to ordinary floating representation.
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

## Review

The independent design review rejected the first draft because it contradicted the transform by saying repeated ids fail, did not name experiment 206 as the third fixture source, left the public derived answer and provenance fields incomplete, and omitted the `jq -n` guard and exact validation boundary. Its second pass found that copied question and metadata fields still lacked types. The corrected contract makes duplicate ids the grouping key, fixes every derived field for all three supported answer kinds, validates every field it reads or copies, and makes all five metric guards apply to any duplicate input.
