---
date: 2026-10-11
status: the full design is paused; a modest slice is a 0.3 candidate
source: Ian's dictation on 2026-10-11
---

# ThinkThen evals

Evals are a separate concept from the toolbox (`toolbox.md`) and get their own design. Ian, 2026-10-13: evals start in 0.3 and may not finish. The big picture must be designed now, because the database (`data.md`) is designed from it. The design must hold these from the start.

- **One evaluation suite.** `audit` and `diff` become parts of one evaluation suite, designed with Ian. Nothing is written down about it yet beyond "the decision bench and decision optimizer belong to ThinkThen" (Ian's strategy notes, 2026-10-02).
- **Every mode.** Datasets and evals work the same locally, through the proxy and on-premises, all from the command line.
- **Datasets by tool.** A dataset collects inputs and answers per tool. An agent curates them into blessed gold standards: inputs with accepted outputs.
- **Three uses of the gold set.** Benchmarking, optimization and fine-tuning.
- **Optimization is GEPA on the mutable text.** GEPA may change the wording of the question, the descriptions of choose options, and any context the tool adds beyond what the caller sends. It never changes the option values or labels themselves.
- **Optimization also fits the threshold.** For decide, it finds the best threshold. It also pushes answers apart, so true yeses score higher and true nos score lower against the gold set. Choose, score, rank and the other functions can be optimized too. That is a harder problem.
- **Ordering metrics.** For `rank`, `find` and `choose` tools, eval reports how often the right answer lands in the top k and at what position, not only agreement at a threshold. For retrieval, the ordering is the measure (an ask from a private trial-matching project).
- **"Not hot dog" fine-tuning.** Fine-tune a very small model to answer one question. A collection of these models, one per tool, lets the proxy route by question, not only by model or provider. This is the 2026-10-06 routing ruling made concrete.
- **Curation is fixing data (Ian, 2026-10-14).** With labels, curation finds where models and labels disagree. Without labels, a model gives first answers and curation puts the likely-wrong ones first: low confidence first, then disagreement between models different enough to fail differently.
- **Two interfaces manage datasets (Ian, 2026-10-14): the command line and, later, a UI.** Agents and notebooks use the command line. A notebook reads the published views, charts and selects, and hands its picks to the command line as a file. No library curation for now. Visual tools such as embedding plots with bulk selection may come to the UI for people who are not data scientists.

## The big picture: what the database must answer (lead's draft, 2026-10-13)

Each eval use is a question asked of the database. If the tables cannot answer one of these, the database is wrong.

| Use | The question it asks of the data | What it needs stored |
| --- | --- | --- |
| Save money | Did any acceptable setup already answer this question about this evidence? | Every answer, with the parts a caching rule can match on |
| Pick a threshold (`audit` today) | Which cut-off best matches the right answers for this model? | Raw probabilities, never only the yes or no, plus right answers |
| Compare two runs (`diff` today) | What changed between these two runs, and which change was right? | Runs as named groups of answers |
| Benchmark | How does each of five models score on this set, and at what cost and speed? | A frozen set of right answers, and per-answer tokens and time |
| Curate disagreements | Where do two setups disagree, and what is right? | Answers from both setups side by side, and a place to record the right answer with who decided |
| Optimize the wording (GEPA) | Does wording version 7 beat version 6 on the gold set? | Wording versions under one question id, each scored against the same right answers |
| Order quality for `rank` and `find` | Does the right item land in the top k, and how high? | Right answers that are grades or orders, not only labels |
| Pairwise and tournament ranking | Which of these two is better, and how do the comparisons add up? | A comparison of two items as its own kind of answer and of right answer |
| Watch a model over time | Has this model changed its answers since last month? | History: every answer kept with its time and the model version that gave it |
| Real outcomes | What happened downstream after this answer? | Answer ids, and outcomes recorded against them |
| Fine-tune | Export what is settled, with a random share of easy cases | Right answers joined to their question and evidence, exported as JSONL |

Two rules fall out of the table:

- **Right answers never depend on the model, the wording or the threshold.** They key on the question id and the evidence, as Ian's data-model notes say. One right answer grades every model and every wording ever tried.
- **Frozen sets.** A benchmark or a gold set is a named, versioned list of right answers. Adding a judgment later never changes a result already reported against an older version.

## A modest slice for 0.3: cases in the toolbox

A candidate, to be checked by an experiment before it joins 0.3.

- **What.** A tool may carry a cases file: inputs with their expected answers. `thinkthen eval TOOL` runs the tool over its cases and reports agreement, the not-sure rate and a suggested threshold. It reuses today's `audit` logic.
- **Why.** Every tool proves itself on its own cases, and the toolbox's self-description can show that result. The GEPA recipe needs a score to climb, and this provides one. The cases file is the seed of the later datasets.
- **Both modes.** The same command and output work locally and through the proxy.
- **Out of the slice.** Curation, frozen dataset versions, benchmarks across models, optimization inside ThinkThen and fine-tuning.
- **The experiment.** Build one toolbox from labeled public data, such as BANKING77, which earlier ThinkThen work already uses. Check that eval gives a stable, useful number and threshold, and that a GEPA recipe run against it moves the score. If both hold cheaply, the slice joins 0.3. If not, evals wait.
- **A second, real check.** A private trial-matching project offers to run eval on a private concept-lookup toolbox against a licensed curation dataset's answers and planted errors, after its first curated trial. That data is licensed and stays on its machine; only aggregate numbers come back. It tests ordering metrics and self-description on real retrieval. It does not gate the slice; BANKING77 does.

## To design with Ian

1. The dataset format and the curation flow on the command line.
2. The evaluation suite, with `audit` and `diff` folded in.
3. How benchmarking, optimization and fine-tuning each read the gold set.
