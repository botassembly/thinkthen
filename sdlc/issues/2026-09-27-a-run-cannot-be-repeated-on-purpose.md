# A run cannot be repeated on purpose, so a loop cannot measure its own noise

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in the workspace at `experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`. Deferred past 0.1 by the tuning review of 2026-09-28: the need varies, and existing commands cover useful parts of it. Ian can overturn this placement.

Priority: rank 22 of 25 in `../planning/issue-priorities-2026-09-30.md`. Owner: a future ticket after 0.1.

## What happens today

The cache answers a repeated request for free, so asking the same question again through the cache returns the same bytes. `--no-cache` forces every request live, but no command repeats a record a fixed number of times, and no command reports how the answers varied. `audit` reads two saved runs or one run at two cuts; it has no repeat input.

## Why it matters

Jev's probabilities move between identical requests. A loop that compares one run against another cannot separate a wording effect from rerun noise, and a model's lean can look like a wording win. Experiment 249 met this first and paired every variant with a fresh control run by hand. Experiment 297 did the same with a custom script.

## Measured

Local experiment 297, `runs/noise-summary.json`. Fifty `multi-hop` same-month yes/no cases had three fresh answers each, or 150 answer observations. The file's `live_requests: 710` is a cumulative experiment ledger, not an attributable count of these samples' transport sends:

- One case changed its answer across the three runs, and it sat at p 0.49 to 0.52.
- Probability spread across repeats: median 0.020, 90th percentile 0.040, maximum 0.070.
- Mean accuracy at the run cut per repeat: 0.527.

In these observations most answers stayed the same while probabilities moved. The one flipped case sat on the cut, making it a useful case to inspect or label.

## What to change

`--repeat N` on `decide`, `filter`, `rank`, and `choose`, default 1. Each repeat obtains a fresh model answer and prints its own row or its own share, so a script can count flips and see the spread. Transport sends must be counted separately when records are packed. Repeats must bypass cached answers or the option measures nothing.

A smaller alternative is a stability section in `audit` over a repeated run. The option is the more direct form, it is additive, and a default of 1 changes no existing run.

## Factual preparation, 2026-09-28

At main `86d5cff1`, both `--no-cache` and `--refresh-cache` support an external fresh-call loop. The latter requires an enabled answer cache, replaces complete cached answers and conflicts with `--no-cache`, `--record` and `--replay`; it does not add a repeat count or stability output. A built-in repeat needs explicit record/repeat identity and whole-run attempt facts. Under default packing, four logical answers may use fewer than four transport sends; a four-send proof must specify `--batch 1`. This is optional tuning work, not a 0.1 correctness blocker. The observed one flip among 150 answer observations is evidence, not a future test threshold. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

## Added 2026-09-28: the scale of the flip rate

Arize compared Jev against five LLM judges over 517 labeled examples with ten runs each (the Jev-as-judge post, read 2026-09-28). Jev changed its answer on 0.97% of examples as a drop-in and 0.19% native, the lowest of any judge there. ThinkThen is the native form by construction. Local experiment 297 measured one flip among 150 answer observations, on a case at p 0.49 to 0.52, and a median probability spread of 0.020. That observed flip was at the cut. The repeat feature is for finding such cases.

## Factual preparation refresh, 2026-09-28

At source `9b766cf9`, `cli/args.rs` and `cli/asking/folders.rs` confirm that `--no-cache` and `--refresh-cache` enable an external fresh-call loop, but neither supplies a repeat count, repeat identity or variation result. Refresh requires an enabled cache and replaces complete answers. `--facts` gives whole-command totals; default `--batch max` can put multiple logical answers in one transport request and cache key. Thus the original built-in-repeat criterion remains distinct from both options. The 50 cases times three runs in `runs/noise-summary.json` are 150 answer observations; its `live_requests: 710` is the cumulative experiment ledger then, not a noise-only send count. The one local flip and the separate 517-example external judge percentages are cohort-specific, not acceptance thresholds. First decide repeat output/order, cache policy and per-repeat accounting; prove fresh values and exact sends with `--batch 1` when a four-send oracle is wanted. See `sdlc/records/2026-09-28-tuning-evidence-refresh.md`.

## Scope note, 2026-09-28

The need varies and existing commands cover part of it. `--no-cache` forces fresh requests, and running the
question twice plus `diff` answers whether two runs differ. The measured noise (one flip in 150 answer observations, on a case at
p 0.49 to 0.52, with a median probability spread of 0.020) remains the case for a per-record repeat option when a
project needs one.
