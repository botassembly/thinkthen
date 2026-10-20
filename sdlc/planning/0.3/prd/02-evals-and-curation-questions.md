---
date: 2026-10-13
status: draft PRD
source: Ian's rulings in `evals.md` and `data.md` of 2026-10-11 to 2026-10-13; PRD 00 and its questions; Ian's ThinkThen notes (data-model, fixed-and-tunable, decision-knowledge, strategy, later/interface, the 2026-09-27 optimization lessons, the curation tool market); ThinkThen 0.2's `specification/audit.md`, `diff.md`, `threshold.md`, `result.md`, `cache.md` and `thinkthen runs --help`; outside sources listed at the end
---

# PRD 02 working file: the hard questions of evals and curation

This file backs [02-evals-and-curation.md](02-evals-and-curation.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the three that need his direction. The rest are recommendations the PM can carry into tickets, and Ian can overturn any of them. Claims about 0.2 cite pages in [specification/](../../../../specification/).

Words follow PRD 00: a **right answer** is one settled row, **settle** is the verb, an **answer key** is a frozen numbered set, a **setup** is one combination of backend, model, wording, context and reading rule, and a **pick** is one line of the curation queue.

## Right answers

**Q1. What does a right answer key on, and what does it hold?**
Options: key on the record id, as 0.2's key file does; key on the cache key; key on the question id and the evidence. A record id means nothing outside one file, and the cache key holds the model, so a right answer keyed on it could grade one model only. `data-model.md` names the truth key, question plus evidence, and `evals.md` makes it a rule: "Right answers never depend on the model, the wording or the threshold." The value must take each function's shape, as 0.2's key already does per verb ([audit.md](../../../../specification/audit.md), "Inputs"): yes or no, an option, a level, a label set, a unit, spans, edges. The curation market note warns that "insufficient evidence is not false" and that an unreviewed item must never become a soft 0.5. Recommendation: the truth key is the question id plus the evidence digest. The value is the function's own value shape, or `cannot tell` with one of two reasons: the evidence is insufficient, or the question is ambiguous. A `cannot tell` row leaves grading and is counted. An ambiguous-question row is a signal to fix the wording.

**Q2. What does a right answer look like for `rank` and `find`?**
Options: store the right order as a list; store the one right pick; store a grade per member. An order breaks when the candidate set changes, and a pick breaks when the right item leaves the shortlist. `fixed-and-tunable.md` already says a `choose` check "stores the right entity's id, not its place in the list", so it survives a new shortlist. TREC's relevance judgments (qrels) grade each document against a query, and every order metric derives from them. Recommendation: for `rank`, `filter` and `find` tools, a right answer is a grade per member, keyed on the question, the shared inputs that make it a query, and the member's evidence. Grades default to yes or no and may use a small scale, such as 0 to 3. The right order sorts by grade. The right `find` pick is the highest-graded member present, or `none` when none is relevant. One set of grades then serves a `filter`, a `rank` and a `find` tool over the same pool. Pairwise right answers ("A is better than B") wait on the ranking experiment in `functions.md`. The table keeps room for a judgment over two pieces of evidence, as `data.md` already rules.

**Q3. Two people disagree. What is the right answer?**
Options: the newest settle wins; a majority wins; the item is disputed until someone resolves it. Newest-wins hides disagreement, which is itself evidence that the question is unclear. Majority needs three judges for most items, which a one-person builder lacks. Dawid and Skene (1979) model each judge's error rate to combine labels, which needs many judges and many items. Labelbox and INCEpTION keep each judgment and adopt a reference answer as a separate act. Recommendation: every settle is a new row. A later settle by the same actor replaces that actor's earlier one. A settle that differs from another actor's live answer marks the item disputed. A disputed item leaves grading and key freezing, is counted and listed, and appears in the queue. A settle that names the rows it replaces with `--replaces` resolves the dispute and records who resolved it. When the same items carry settles from two people, `eval grade --judges` reports their agreement and Cohen's kappa.

**Q4. How does an agent settle safely?**
Ian ruled that the tool never calls a large model, and that an agent's change records the agent and the person it acts for (`data.md`). The risks: the agent copies the model it is checking, the agent is confidently wrong, and an agent's settles flood a key. `decision-knowledge.md` says the large-model resolver "is itself ungraded" and needs "a human audit sample and a calibration record". Recommendation: an agent settles through the same `curate settle` command. Its rows record the basis `agent`, the agent's name from `THINKTHEN_AGENT`, its person, the pick it answered and what it saw. The queue is blind by default (Q6). A key built with `--trust person` takes only person-settled rows. The default trust takes person rows and agent rows whose agent has been audited (Q5). Each key reports the share of agent rows and the agent's measured agreement.

**Q5. How does a person audit the agent?**
Options: a person re-checks every agent settle; a person checks none; a person checks a random sample. Checking everything wastes the agent. Checking none leaves the agent's error rate unknown. The idea that crystallizes this: a judge is a setup. An agent's settles are answers, and person settles on the same items grade them, as any model is graded. Recommendation: `curate queue TOOL --audit AGENT` draws a random sample of that agent's settles, blind to the agent's value, for a person. `eval grade --judges` prints each agent's agreement with person rows on shared items, with a Wilson interval. An agent counts as audited once its sample holds at least 30 person checks, and the key reports the interval rather than a pass or fail. The experiment's arm D measures whether 30 checks bound an agent's error usefully.

**Q6. Should the queue show the models' answers?**
Showing the answers speeds a person and anchors them. An agent shown the model's answer tends to agree with it, which defeats the check. `later/interface.md` asks for "a blind mode" that "hides the model's answer so reviewers are not anchored". Recommendation: the queue is blind by default. It shows the question, the options and the evidence, and says why the item was picked, such as "two setups disagree", without saying which said what. `--show-answers` reveals answers and probabilities, and each settle records whether the actor saw them. A key can later be filtered to blind settles.

## The curation queue

**Q7. What does the queue sort by: probability or disagreement?**
0002 asked which signal finds a model's wrong answers fastest on 1,501 Beatles questions. After the 0020 correction, disagreement with d1free2 found half of Jev's 448 errors in 367 reviews and Jev's own probability took 397 (`decision-knowledge.md`, section 3). Probability needs no second model. `strategy.md` records the ruling: sort by the model's own probability, and "when a second model's answers already exist, disagreements can go first". `decision-knowledge.md` adds the sharper point that the two signals answer two questions. Probability finds errors. Disagreement between production and a candidate is the whole behavior change if the candidate is promoted. Recommendation: the default queue sorts by the model's own probability of its answer, nearest the cut first, which is "lowest first" for the answer given. `--between SETUP SETUP` sorts by disagreement, widest probability gap first, and serves the promotion question. Disputed items and outcome conflicts come before both. Arm B checks the order on a new domain.

**Q8. How large is the random share, and what does it buy?**
Queue picks are hard cases. A key built only from them understates accuracy, shifts a tuned cut and teaches a fine-tune a world of edge cases. `decision-knowledge.md` says "every gold set needs a random stratum of agreed cases", and all four partners in 0002 agreed with Jev on 37 errors no disagreement signal can flag. Active testing (Kossen and others, 2021) shows that actively chosen test labels give biased estimates unless each is weighted by its chance of selection. The 0.2-era label-budget test found that ten to sixty uncertainty-picked labels tuned a better cut than random labels (optimization lessons, section 8). So hard picks help tuning and hurt reporting. Recommendation: every queue mixes in a random share, 20% by default and set with `--random`. Each row records its stratum. Tuning uses every settled answer. The headline accuracy and its interval come from the random stratum alone, and the other strata print as their own rows. Weighting queue picks by their selection chance is the statistically better tool and comes later, once picks record that chance reliably.

**Q9. Does the queue write anything?**
PRD 00 principle 4 says to store raw and read by rule, which suggests the queue is a pure read. But a right answer's stratum is a fact about how it was chosen, and nothing can recompute it later, because new answers change the order. Recommendation: `curate queue` writes one pick row per line it prints, with the rule, the stratum, the actor and the time. `curate settle PICK VALUE` names the pick, which also gives an agent a short handle. A settle with no pick records the stratum `hand-picked`, which never counts as random.

## Answer keys

**Q10. What does a key version pin?**
Options: pin the items; pin the items and their current right answers. Pinning items alone means version 3 changes when someone corrects a label tomorrow, which breaks Ian's rule that "adding a judgment later never changes a result already reported" (`evals.md`). Braintrust and the curation market note both pin the answer revision. Recommendation: `curate key NAME` freezes the next version. Each member row pins one right-answer row, its stratum and its part. A version refuses disputed items and lists them. It records its question id and contract digest, so grading a setup with other labels refuses (`fixed-and-tunable.md`, layer 1). Correcting an answer and freezing again makes a new version, and old evals re-score from stored answers.

**Q11. How do tune and held parts stay honest across versions?**
0.2's audit draws a seeded split each time it grades ([audit.md](../../../../specification/audit.md), "The split"). A key that grows by versions cannot do that. A reshuffle would move held items into the tune part, and an optimizer would then see the answers it is checked on. The optimization lessons found that "val selection overfits on small sets": a second family rose on the validation split and fell on the held-out test. Recommendation: a member's part comes from a hash of the key name and its truth key, compared with the held share (30% by default). The same item lands in the same part in every version. A key file line that already carries 0.2's `part` keeps it. The eval record counts every grade that read the held part for a tool, and `eval grade` warns when the count passes one, because each look at the held part spends its honesty.

**Q12. How does this work with 0.2's answer key file?**
0.2's key is JSONL with `id`, `value` and an optional `part` of `tune` or `held`. It keys on a record id read through `--id` from each result's `input`, and it holds no question and no evidence ([audit.md](../../../../specification/audit.md), "Inputs"). Builders and the GEPA lessons already hold such files. Recommendation: three paths. First, `eval grade RESULTS KEY` is 0.2's `runs audit` with the same options, output, exit codes and guarantee that it reads no key, configuration or cache. Second, `curate settle --import KEY --results RESULTS` joins each key line to its result line by id, takes the question and evidence from the result, and stores right answers with the basis `imported`. Third, `curate export NAME@N --format key` writes a 0.2 key file plus the matching inputs, so file-mode grading and old scripts keep working. PRD 08 adds a spreadsheet form of the import, where each reviewed cell carries its answer id; those rows take the basis `imported` and the stratum `hand-picked` unless the sheet came from a queue.

**Q13. How does a tool's cases file relate to an answer key?**
The `evals.md` slice gives a tool a cases file: inputs with expected answers, kept beside the tool and shown in its self-description. A store key is private and grows. Recommendation: a cases file is an exported answer key that travels with the toolbox. `curate export NAME@N --format cases` writes one, and `curate settle --import` reads one back. `eval bench TOOL` with no `--key` runs the tool over its cases file. The toolbox's self-description shows that result, as `evals.md` asks.

## Grading, comparing, benchmarking

**Q14. Which eval commands send requests?**
0.2's audit and diff each "sends no request and reads no API key" ([audit.md](../../../../specification/audit.md), [diff.md](../../../../specification/diff.md)). The `evals.md` slice proposed `thinkthen eval TOOL` that runs a tool and grades it. One word that sometimes sends and sometimes does not is a surprise and a cost risk. Recommendation: `eval grade` and `eval compare` never send. `eval bench` is the only eval word that sends. It runs the tool or question over a key's members under each setup, stores the answers, writes an eval record and prints `eval grade`'s rows per setup. The slice's "run a tool over its cases" is `eval bench TOOL` with one setup.

**Q15. How does a benchmark control cost?**
Five models on a thousand-member key is five thousand decisions, cheap on a decision model and costly on a large one. The optimization lessons found that "reusing saved answers makes baselines free", with an order of magnitude fewer live calls than metric calls. PRD 00 Q18 says `eval bench` "always asks fresh under an exact rule". Fresh and exact-rule reuse differ. Recommendation: `eval bench` takes `--plan` and the spend caps, as PRD 00 principle 10 requires, and prints requests and tokens per setup before sending. It reuses a stored answer only under the exact rule, with the model version as answered, and reports how many it reused and their age. `--fresh` asks again. `--sample N` takes a seeded random subset of the members for a cheap first pass. The batch setting is part of the setup, because batching changes answers (`decision-knowledge.md`; audit's batch warning). This narrows PRD 00 Q18's word "fresh" to "exact rule, reuse allowed"; the PM should align the two.

**Q16. What does an eval record pin, and is it a "run"?**
PRD 00 Q2 drops the run table and keeps an eval record. A key pins right answers. If a bench pinned nothing on the answer side, a later answer under the same setup could change the reported number on re-read. Recommendation: an eval record holds the key version, each setup, the actor, the time, the reuse count and one row per member and setup naming the stored answer used. Re-reading it gives the same numbers forever, and a new threshold re-scores it for free. `eval grade` and `eval compare` compute on read and write nothing, except the held-part count in Q11.

**Q17. What is a setup on the command line?**
`data-model.md` defines a setup as wording, context recipe, model and reading rule. ADR 0119 gives each engine one route, and PRD 00 Q13 has bench build one engine per setup. Recommendation: `--setup NAME` names a backend from the configuration file, as 0.2's named backends do (ADR 0114). A setup may add a tool version, such as a wording version, and a threshold. The setup's full description is stored with the eval record. Wordings compare as setups too, which is how GEPA candidates and A/B wordings share the same machinery (`decision-knowledge.md`, section 7). A proxy rule's arm is a setup too, so PRD 05's shadow and serve arms compare with `eval compare` and feed `curate queue --between`.

**Q18. How does eval stay statistically honest on small sets?**
The optimization lessons give the warning in one line: "Small test sets settle nothing. Four gains and no losses still gives p = 0.125." Miller (2024) recommends standard errors on every eval number, paired differences when two models answer the same questions, and clustered errors when questions share a source. 0.2 already prints a Wilson interval and runs an exact McNemar test ([audit.md](../../../../specification/audit.md), [diff.md](../../../../specification/diff.md)). Recommendation: every rate prints with its 95% interval. Every comparison of two setups is paired over the same members and prints the gap with a paired interval and McNemar's p. When the interval holds zero, the line says "tied within noise" and ranks nothing. `tag` labels and annotate members pool by record, as 0.2 does, because their errors move together. A bench summary names the smallest gap its key can detect. Rerun noise is reported only when the store holds repeated answers, as 0.2's calibration note already says.

## Thresholds and calibration

**Q19. How does a goal such as "a yes is right 95% of the time" become a cut?**
`data-model.md` names the goal form. 0.2's audit suggests the cut that maximizes accuracy, precision, recall or f1 on a tune part and reports how steady it is ([audit.md](../../../../specification/audit.md), "The suggested cut"). A point estimate of precision on a small set can promise 95% and deliver 85%. Learn then Test (Angelopoulos and others, 2021) gives a guarantee: test each cut in a fixed order and stop at the first failure, which needs no correction for testing many cuts. Recommendation: `--goal yes:0.95` walks the cuts from high to low on the held part. Each cut passes when the one-sided 95% Clopper-Pearson lower bound on its yes precision reaches 0.95. The answer is the lowest passing cut, with its coverage. `--goal no:0.98` does the same for no, and two goals make a band with a not-sure middle. When no cut passes, the refusal says how many held answers would be needed at the observed rate. The arithmetic is plain: with every yes right, 59 held yeses are the least that clear 95%. `--write` writes the cut for the graded model into the tool, as PRD 00 Q36 says.

**Q20. Should the store keep calibration maps?**
Platt scaling and isotonic maps turn a model's probability into a better one. Each map belongs to one model, needs its own held-out data and its own versioning, and adds a reading rule step to every answer. The 0037 experiment found both calibrations hit the temperature ceiling. A goal cut (Q19) meets the business need, "a yes is right 95% of the time", without calibrated probabilities. Recommendation: 0.3 reports calibration as 0.2 does and stores no map. Goals carry the threshold work.

## Order metrics, outcomes and export

**Q21. Which order metrics, and over what?**
`evals.md` asks how often the right answer lands in the top k and at what position, and a private trial-matching project's retrieval ask says the ordering is the measure. 0.2 prints R-precision for `rank` only. The lessons found "the truth in the model's top two" on 70% of chained cases against 37% for the top pick, which is the signal for handing a shortlist to a stronger judge. Recommendation: for `rank` and `find`, from graded right answers, print recall at k, the position of the first right member, mean reciprocal rank, NDCG at k (Järvelin and Kekäläinen, 2002) and R-precision. For `choose`, print top-1 and top-2 agreement from the stored distribution. k defaults to 1, 5 and 10.

**Q22. Is an outcome a right answer?**
The proxy design accepts "what happened downstream" against an answer id, and 0.2 results already carry `answer_id` ([result.md](../../../../specification/result.md)). An outcome is often about the world, not the question: a refund denied for an unrelated reason says nothing about whether the message asked for one. PRD 00 Q37 says an outcome "never changes the stored answer". Recommendation: `curate outcome ANSWER_ID LABEL` stores the caller's label, an optional value in the answer's shape, the actor and a note. An outcome with a value that contradicts the answer feeds the queue under the stratum `outcome`. A person or agent settles it into a right answer with the basis `outcome`. Nothing turns outcomes into right answers by itself.

**Q23. What does the training export hold?**
Ian ruled that export is JSONL (`data.md`). Fine-tuning needs the base rate, so a set of only hard picks teaches the wrong world (Q8). Exporting the held part leaks the test into the model. Unsettled model answers as labels are distillation of the model's own errors. The 0037 experiment raised held-out correctness from 86 to 95 of 100 with own-label tuning. Recommendation: `curate export NAME@N --format train` writes one line per member: the question id, the wording text and options as sent, the evidence, the context, the right answer, the stratum, the part, the basis and the right-answer id. The default is `--part tune`. `--easy-share 0.2` caps the random stratum's share by sampling, and the export header states the share it reached. Only settled answers export. Provider formats such as chat messages ship as jq recipes, because each provider and each decision model frames requests differently.

**Q24. What does the GEPA recipe need from ThinkThen?**
GEPA (Agrawal and others, 2025) evolves prompt text with a reflection model that reads per-case feedback. The lessons found the loop works for four cents of Jev tokens, that `audit` lacked per-case rows (0.2 has since added `--cases`), that `improvement_or_equal` is needed when a second objective exists, and that a blind rewriter at equal budget matched reflection on one task. Ian ruled GEPA ships as a recipe and may change wording, option descriptions and added context, never labels (`evals.md`). Recommendation: the recipe owns the reflection model, so ThinkThen still calls none. ThinkThen supplies `eval bench --part tune` and `eval grade --cases` for per-case feedback, stores each candidate as a wording version under one question id, refuses a candidate that changes the contract, and checks the winner once with `eval compare --part held`. The recipe includes the blind-rewrite arm and reports the result even when wording does not help, which the lessons say is common.

## Scope and proof

**Q25. Which commands ship in 0.3? (Ian)**
The README says "0.3 itself ships proxying and caching, not curation". `data.md` says both strategies live in the data model from day one. `evals.md` says evals start in 0.3 and may not finish. PRD 00 Q1 recommends `eval grade` and `eval compare`, with `curate` and `eval bench` gated by the experiment. But store-backed grading needs right answers in the store, which needs `settle` and `key`. Recommendation: ship every table this PRD names, and ship `curate settle`, `curate key`, `curate outcome`, `curate export`, `eval grade` and `eval compare`. Gate `curate queue` and `eval bench` on the BANKING77 experiment. Marketing stays on caching and the proxy. This ships some curation, which the README's line rules out, so Ian must choose.

**Q26. What does the BANKING77 experiment test, and what admits the slice?**
`evals.md` proposes BANKING77 because an earlier data presentation already uses it. 0035 pinned the publisher revision, its CC BY 4.0 license and a 4,000-row draw. The full set has a label on every row, so every curation policy can be simulated for free once the models have answered. Recommendation: build three `decide` tools for single intents and one `choose` tool over a family of confusable card intents. Answer 2,000 messages on two backends. Arm A: draw 200-case keys twenty times and check interval width (about five points at 85% accuracy), the steadiness of the suggested cut, and a goal cut on the held part. Arm B: errors found per hundred reviews under unsure-first, disagreement-first, mixed and random orders. Arm C: compare the true accuracy with the random-stratum estimate and with a queue-only estimate. Arm D: an agent settles 300 picks through `curate settle` with labels hidden, and 30 checks bound its error. Arm E: the GEPA recipe on the `choose` tool, with a blind arm at equal budget. A and E admit the slice and `eval bench`. B sets the queue default. C confirms the random-share rule. D sets the trust default. A null GEPA gain does not block the slice, because the lessons show wording is rarely the lever.

**Q27. May a key built from agent settles alone be used by default? (Ian)**
An agent can settle hundreds of items for little money. A builder working alone may never check any. The risk is a key that grades every model against the agent's errors, which then become thresholds and training data. Recommendation: the default trust takes person rows and audited agent rows (Q5). `--trust agent` allows unaudited agent rows, and every grade over such a key prints the agent's measured agreement, or "agent unaudited". This is a product choice about how much ThinkThen protects a user from their own agent, so Ian decides.

**Q28. May the experiment spend money? (Ian)**
The repo runs paid calls only through its live ledger under Ian's authorization. By 0035's rates, 4,000 decisions on Jev cost under a cent per step, so 16,000 decisions on two decision backends cost well under a dollar. The agent settler and the GEPA reflection model cost more. Recommendation: approve a cap of ten dollars for the whole experiment.

**Q29. What does "who acted" mean with no sign-in?**
0.3 has no access control. `login NAME` records a name with no password (PRD 00 Q38), and any caller to a proxy can claim to be anyone. A person basis is therefore a claim. Recommendation: every row records the claimed user and agent, and the docs say plainly that 0.3 trust labels guard against mistakes, not against a hostile caller. The proxy records the address each write came from beside the claim. Sign-in later turns claims into verified identities with no table change.

**Q30. Does watching a model over time belong here?**
`evals.md` lists drift: "Has this model changed its answers since last month?" The store keeps every answer with its time and model version, so the data exists. A drift report is a new read with its own design. Recommendation: in 0.3, `eval compare` accepts two setups that differ only by time window, such as answers before and after a date, and reports changed answers with McNemar's test. A scheduled drift report stays later, per PRD 00's U24.

## What the database must hold

These rows extend `data.md`'s draft tables. The database PRD should check each against its schema.

| Table | One row is | Fields this PRD needs |
| --- | --- | --- |
| right answer (`data.md`'s judgment) | One settle | Truth key, value or cannot-tell reason, basis, user, agent, pick, stratum, what was seen, reason, quotes, replaces, time |
| pick | One queue line | Rule, stratum, setups compared, user, agent, time |
| answer key version | One frozen version | Name, number, question id, contract digest, trust rule, held share, user, time |
| answer key member | One right answer in one version | Version, right-answer id, part, stratum |
| outcome | One downstream result | Answer id, label, optional value, note, user, agent, time |
| eval | One bench or saved grade | Key version, setups, reuse count, held reads, user, agent, time |
| eval member | One stored answer used | Eval, member, setup, answer id |

## Contradictions found

- The README's "0.3 itself ships proxying and caching, not curation" conflicts with this PRD's Q25 and with PRD 00's `curate` job. Ian decides in Q25.
- PRD 00 Q18 says `eval bench` "always asks fresh under an exact rule". This PRD reuses exact matches and asks fresh only with `--fresh` (Q15).
- `evals.md`'s slice names `thinkthen eval TOOL`. PRD 00 has no bare `eval`; this PRD maps the slice to `eval bench TOOL` so that `eval grade` never sends (Q14).
- `data.md` drafts a run table. PRD 00 Q2 drops it, and this PRD follows PRD 00 with the eval record (Q16).
- The thinkthen repo's agent rules say "the proxy owns ... curation and automatic threshold tuning" and "add no SDK business policy". Goal cuts and settles in the SDK need the boundary amendment PRD 00 Q14 already names.
- `toolbox.md` records "No model comparisons" as a marketing rule. `eval bench` compares models on the user's own data, which that rule allows. ThinkThen's own materials should still publish no cross-model ranking from it.

## Sources

ThinkThen's own:

- [specification/audit.md](../../../../specification/audit.md), `diff.md`, `threshold.md`, `result.md`, `cache.md`; `thinkthen runs --help` on the 0.2 build.
- Ian's notes: `decision-knowledge.md` (0002, 0015 and 0020 findings; the random audit; the ungraded resolver), `data-model.md` (three keys; goal thresholds), `fixed-and-tunable.md` (five layers; held-out split), `strategy.md` (queue ruling), `later/interface.md` (blind mode).
- Ian's architecture notes: the 2026-09-27 optimization lessons (GEPA cost, label budget, noise floor, small-set warning) and `curation-tool-market.md` (prediction, judgment and reference kept apart).
- ThinkThen experiment 0035, Jev in your database at scale (BANKING77 draw and cost), and experiment 0037 (own-label tuning).

Outside:

- Agrawal and others, "GEPA: Reflective Prompt Evolution Can Outperform Reinforcement Learning", arXiv:2507.19457, 2025.
- Khattab and others, "DSPy: Compiling Declarative Language Model Calls into Self-Improving Pipelines", arXiv:2310.03714, 2023. Its train and dev sets and metric functions are the model for the recipe's inputs.
- Angelopoulos, Bates, Candès, Jordan and Lei, "Learn then Test: Calibrating Predictive Algorithms to Achieve Risk Control", arXiv:2110.01052, 2021.
- Kossen, Farquhar, Gal and Rainforth, "Active Testing: Sample-Efficient Model Evaluation", ICML 2021, arXiv:2103.05331.
- Miller, "Adding Error Bars to Evals: A Statistical Approach to Language Model Evaluations", arXiv:2411.00640, 2024.
- Polo and others, "tinyBenchmarks: evaluating LLMs with fewer examples", arXiv:2402.14992, 2024. Small, chosen subsets can estimate a full score, which supports `--sample` as a first pass.
- Dawid and Skene, "Maximum Likelihood Estimation of Observer Error-Rates Using the EM Algorithm", Applied Statistics, 1979.
- Northcutt, Jiang and Chuang, "Confident Learning: Estimating Uncertainty in Dataset Labels", arXiv:1911.00068, 2021. Cleanlab treats its output as a review priority, not a correction.
- Clopper and Pearson, 1934; Wilson, 1927; McNemar, 1947. Järvelin and Kekäläinen, "Cumulated Gain-Based Evaluation of IR Techniques", ACM TOIS, 2002. TREC relevance judgments (qrels).
- Casanueva and others, "Efficient Intent Detection with Dual Sentence Encoders", 2020 (BANKING77, CC BY 4.0).
- Tools: promptfoo (a matrix of test cases by providers by prompts, with a response cache; the model for `eval bench`); Braintrust (versioned datasets with expected values, experiments compared by scorer); Langfuse (datasets, dataset runs, scores and annotation queues on an open core); Argilla (model suggestions kept apart from human responses, several responses per record); Label Studio (predictions kept apart from annotations; prediction-score sorting); Prodigy (review recipes, and its warning that accepting agreement between models accepts their shared mistakes).
