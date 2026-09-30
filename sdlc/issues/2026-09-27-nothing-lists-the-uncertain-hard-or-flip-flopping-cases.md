# Tuning loop asks: cases to label, repeated runs, and cost beside the score

Status: open, deferred past 0.1 by the tuning review of 2026-09-28: the need varies, and existing commands cover useful parts of it. Ian can overturn this placement. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297 (`experiments/296-gepa-question-tuning/`, `experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md` in the workspace). Merged on 2026-09-30 from three issues; the repeat and cost issues are in `closed/`.

Owner: a future ticket after 0.1 for each part. They serve the ideal state's tuning loop, where ThinkThen supplies rows for Optimizer, and no phase goal. Part 3 depends on the run facts shape in `2026-09-26-every-surface-should-give-back-run-facts.md`.

## 1. Nothing lists the uncertain, hard, or flip-flopping cases a person should label

### What happens today

`audit` reports a whole question: counts, measures, a coverage curve, and a suggested cut. It never ranks the cases inside the question. `diff` names the cases that changed between two runs, which is a different question: it needs two runs, and it does not rank by how much a label would be worth.

### Why it matters

Every tuning round begins by choosing a few cases to label. The Optimizer ideal state says the round "picks the uncertain cases and a random audit sample." Nothing in ThinkThen does that, so each consumer writes its own selector, and a poor selector wastes the only expensive resource the loop has: a person's labels.

### Measured

Local experiment 297, `runs/labels.json`. The saved probabilities of the bench's 2026-09-26 run over all 228 yes/no questions. A fixed third, 76 cases, was held out. Policies picked labels from the other 152, the cut was tuned on the picked labels, and accuracy was measured on the fixed held set.

| Labels | Random | Uncertainty | Uncertainty plus a 20% audit share |
| --- | --- | --- | --- |
| 10 | 0.699 | 0.724 | 0.724 |
| 30 | 0.704 | 0.724 | 0.724 |
| 60 | 0.711 | 0.724 | 0.722 |
| 120 | 0.713 | 0.697 | 0.697 |
| All 152 | 0.697 | 0.697 | 0.697 |

Held at the default cut 0.5: 0.711. In this cohort and simulation, ten to sixty uncertainty-picked labels beat random, the default, and tuning on all 152 labels. The 120-label and all-label outcomes were lower on this held set; this one experiment does not establish a universal label budget or cause.

The same experiment measured the flip side, in `runs/noise-summary.json`: fifty `multi-hop` same-month yes/no cases asked three times each with no cache produced one answer flip, on a case at p 0.49 to 0.52. The median probability spread across repeats was 0.020 and the maximum 0.070. A selector wants the cases near the cut and the cases that disagree between runs.

### What to change

A subcommand that ranks cases for labeling: distance from the cut, disagreement between two saved runs, and a flip across repeats, with a seeded random share for quiet drift. The uncertainty queue reads saved `--details` rows, sends nothing, and needs no key; ranking labeled hard misses uses a key when requested. A tag or annotate candidate identifies its criterion as well as its record.

The label simulation above motivates the selector; its 10-to-60-label accuracy advantage is not a general acceptance threshold. A new command is additive. Its proof should pin exact saved-row predicates and order: near versus far cut, labeled hard miss, paired-run disagreement, repeat flip, criterion member, stable ties and seeded random share, with zero sends.

### Factual preparation, 2026-09-28

At main `e58aceae`, `audit` groups scored cases and `diff` names changes; neither supplies the proposed label-order queue. Per-case parsing would help, but uncertainty, disagreement, hard-case and seeded audit-share criteria remain distinct. Experiment 297’s 10–60-label advantage was observed on one cohort and must not become a required empirical accuracy gate. A small saved-row table can prove deterministic ordering, tie handling and zero sends. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

### Added 2026-09-28: the shape of the miss, and the criterion

The follow-up reading adds three selection signals beyond distance from the cut.

- **The shape of the miss.** Cluster the misses before choosing a case to label. A wrong value one step off points at knowledge, an answer at the cut points at scale, a tie or null points at ambiguity. The chained album-year family's 38 misses are one or two years off in both directions, so the lever is evidence or a split chain, not a label.
- **The criterion, not only the record.** A decision model grades a whole rubric in one request (AutoRubric), and ThinkThen's `tag` and `annotate` return a probability for each label or each named question. The unit to select is the uncertain criterion as well as the uncertain record.
- **The shortlist.** The truth sits in Jev's top two on 70% of the chained album-year cases against 37% for the top pick, and 95% on the single-hop song-to-album question. A second-position answer is a case worth a label or a stronger judge.

Evidence: `experiments/297-gepa-loop-tests/LESSONS.md` sections 11 to 13.

### Factual preparation refresh, 2026-09-28

At source `9b766cf9`, `audit` groups saved outcomes and `diff` pairs changed cases, but neither orders an unlabeled candidate queue, reserves a seeded audit share or aligns repeat flips. The per-case audit issue can share its parser and identity; it does not fulfill this ranking criterion. The `runs/labels.json` advantage came from one 228-case yes/no pool, fixed held 76, label pool 152 and 300 trials. The original 10-to-60 advantage is retained as evidence, not a universal pass/fail gate. A future design must specify record plus tag/annotate criterion identity, key-free uncertainty versus labeled hard cases, missing probabilities, run/repeat pairing, stable ties and seeded share. Top-two evidence applies only when a suitable choice distribution exists. A small saved table can prove exact order/share, near-cut and hard misses, disagreement and flip, with zero sends; it need not reproduce the experiment's held-set accuracy. See `sdlc/records/2026-09-28-tuning-evidence-refresh.md`.

### Scope note, 2026-09-28

The need varies and existing commands cover part of it. `audit --cases` prints a row per case with its outcome and
probability, so a person can rank cases by proximity to the cut today, and `audit --optimize
accuracy|precision|recall|f1` tunes the suggested cut for the named measure. The label simulation's evidence (ten to
sixty uncertainty-picked labels held 0.724 against 0.711 at the default cut and 0.697 for the full-pool tune)
remains the case for a built-in selector when a project needs one.

### The Beatles Bench plan, 2026-09-30

The public Beatles Bench plans three pieces that cover part of this need from outside thinkthen. Bench ticket 0022 writes one JSON summary per function, pinned to thinkthen tags. Bench ticket 0023 runs one sample at a time. A repeat pass lists the answers near the cut and the answers that flip between runs. A ticket that takes this issue can check its order against those lists. `../planning/bench-handoff.md` gathers the bench's other asks.

## 2. A run cannot be repeated on purpose, so a loop cannot measure its own noise

### What happens today

The cache answers a repeated request for free, so asking the same question again through the cache returns the same bytes. `--no-cache` forces every request live, but no command repeats a record a fixed number of times, and no command reports how the answers varied. `audit` reads two saved runs or one run at two cuts; it has no repeat input.

### Why it matters

Jev's probabilities move between identical requests. A loop that compares one run against another cannot separate a wording effect from rerun noise, and a model's lean can look like a wording win. Experiment 249 met this first and paired every variant with a fresh control run by hand. Experiment 297 did the same with a custom script.

### Measured

Local experiment 297, `runs/noise-summary.json`. Fifty `multi-hop` same-month yes/no cases had three fresh answers each, or 150 answer observations. The file's `live_requests: 710` is a cumulative experiment ledger, not an attributable count of these samples' transport sends:

- One case changed its answer across the three runs, and it sat at p 0.49 to 0.52.
- Probability spread across repeats: median 0.020, 90th percentile 0.040, maximum 0.070.
- Mean accuracy at the run cut per repeat: 0.527.

In these observations most answers stayed the same while probabilities moved. The one flipped case sat on the cut, making it a useful case to inspect or label.

### What to change

`--repeat N` on `decide`, `filter`, `rank`, and `choose`, default 1. Each repeat obtains a fresh model answer and prints its own row or its own share, so a script can count flips and see the spread. Transport sends must be counted separately when records are packed. Repeats must bypass cached answers or the option measures nothing.

A smaller alternative is a stability section in `audit` over a repeated run. The option is the more direct form, it is additive, and a default of 1 changes no existing run.

### Factual preparation, 2026-09-28

At main `86d5cff1`, both `--no-cache` and `--refresh-cache` support an external fresh-call loop. The latter requires an enabled answer cache, replaces complete cached answers and conflicts with `--no-cache`, `--record` and `--replay`; it does not add a repeat count or stability output. A built-in repeat needs explicit record/repeat identity and whole-run attempt facts. Under default packing, four logical answers may use fewer than four transport sends; a four-send proof must specify `--batch 1`. This is optional tuning work, not a 0.1 correctness blocker. The observed one flip among 150 answer observations is evidence, not a future test threshold. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

### Added 2026-09-28: the scale of the flip rate

Arize compared Jev against five LLM judges over 517 labeled examples with ten runs each (the Jev-as-judge post, read 2026-09-28). Jev changed its answer on 0.97% of examples as a drop-in and 0.19% native, the lowest of any judge there. ThinkThen is the native form by construction. Local experiment 297 measured one flip among 150 answer observations, on a case at p 0.49 to 0.52, and a median probability spread of 0.020. That observed flip was at the cut. The repeat feature is for finding such cases.

### Factual preparation refresh, 2026-09-28

At source `9b766cf9`, `cli/args.rs` and `cli/asking/folders.rs` confirm that `--no-cache` and `--refresh-cache` enable an external fresh-call loop, but neither supplies a repeat count, repeat identity or variation result. Refresh requires an enabled cache and replaces complete answers. `--facts` gives whole-command totals; default `--batch max` can put multiple logical answers in one transport request and cache key. Thus the original built-in-repeat criterion remains distinct from both options. The 50 cases times three runs in `runs/noise-summary.json` are 150 answer observations; its `live_requests: 710` is the cumulative experiment ledger then, not a noise-only send count. The one local flip and the separate 517-example external judge percentages are cohort-specific, not acceptance thresholds. First decide repeat output/order, cache policy and per-repeat accounting; prove fresh values and exact sends with `--batch 1` when a four-send oracle is wanted. See `sdlc/records/2026-09-28-tuning-evidence-refresh.md`.

### Scope note, 2026-09-28

The need varies and existing commands cover part of it. `--no-cache` forces fresh requests, and running the
question twice plus `diff` answers whether two runs differ. The measured noise (one flip in 150 answer observations, on a case at
p 0.49 to 0.52, with a median probability spread of 0.020) remains the case for a per-record repeat option when a
project needs one.

## 3. No cost beside the audit score

### What exists

- `--facts` prints whole-run requests and tokens, including rows `filter` drops and `rank --top` cuts (ticket 0170). With caller prices set, it adds `estimated_cost_usd` (ticket 0300, ADR 0108).
- `audit --cases` carries each result line's `meta.usage` with `usage_scope`.
- The `cost` transform totals printed detail rows at a caller-supplied input price.

### What is missing

`audit` puts no cost beside its score. A tuning loop weighs accuracy against cost for every candidate, so it must sum `meta.usage` over the rows itself. The ask is a score-adjacent total in machine-readable output: the reported `meta.usage` shares over the printed rows, and optionally money at a named caller price. The design must keep absent usage unknown, tell cached rows apart, and never add row shares to their whole-batch `meta.batch.usage`. A printed-row subtotal cannot stand for all live work, so it is distinct from `--facts`.

### Measured

- Experiment 296, the chained album-year question: the seed took 450 input tokens a call and the winning wording 630, 40% more, for four gains and no losses on a held-out fifteen.
- Experiment 296, the knowledge blob: 3,713 input tokens a call for the seed against 2,794 for a compressed blob that held accuracy, 25% less.
