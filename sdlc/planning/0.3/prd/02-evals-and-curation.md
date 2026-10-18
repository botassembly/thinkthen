---
date: 2026-10-13
status: draft PRD
release: 0.4
source: Ian's rulings in `evals.md` and `data.md` of 2026-10-11 to 2026-10-13; PRD 00 and its questions; Ian's ThinkThen notes (data-model, fixed-and-tunable, decision-knowledge, strategy, the optimization lessons of 2026-09-27, the curation-tool market); ThinkThen 0.2's `specification/audit.md`, `diff.md`, `threshold.md`, `result.md` and `thinkthen runs --help`. The 30 worked questions and sources are in [02-evals-and-curation-questions.md](02-evals-and-curation-questions.md)
---

# PRD 02: Evals and curation

One design for every eval use in `evals.md`, built on PRD 00's `curate` and `eval` jobs. It adds no top-level word.

## Who and what's in it for them

- **The builder** learns how good a tool is on their own data, with an honest margin, and sets a cut from a goal such as "a yes is right 95% of the time". Proof: the BANKING77 experiment shows a 200-case key gives a number within about five points, a goal cut that holds on held-out cases, and a refusal naming the count still needed when cases are too few.
- **The agent** settles right answers for its person, and the person sees how often the agent is right. Proof: the experiment grades an agent settler against BANKING77's labels.
- **The reviewer** spends time on the answers most likely wrong, plus a fixed random share that keeps every report honest. Proof: errors found per hundred reviews by each queue order, against random, on two models.
- **The data person** exports settled answers as JSONL training data with a stated share of easy cases. Proof: a demonstrated export.

## Use cases

U17, U23, U25 to U28, U30 to U35 from PRD 00. U24's drift report and U29's review screen stay later.

## The design

Five rules carry everything.

1. **A right answer keys on the question id and the evidence,** never on the model, wording or threshold. It holds a typed value, or "cannot tell" with a reason. For `rank` and `find` it holds a grade per member, and the right order and the right pick derive from the grades.
2. **Every right answer records who acted, how the item was picked and what they saw.** The basis is a person or an agent with its person. The stratum is the queue rule that picked it: unsure, disagreement, random, outcome, imported or hand-picked. Nothing is overwritten. A settle that differs from another actor's live answer marks the item disputed until someone settles it again and names what they replace.
3. **An answer key is a frozen, numbered set.** Each member pins one right-answer row, its stratum and its part, tune or held. A member's part comes from a hash of its truth key, so it never moves between versions.
4. **An eval pins answers the way a key pins right answers.** A bench records which stored answer each setup gave for each member, so a reported number never moves.
5. **Grade on all settled answers, report population numbers from the random stratum.** Queue picks are hard cases by design. Rows show each stratum apart, and the headline rate and interval come from the random stratum alone.

The surface follows PRD 00:

```text
thinkthen curate queue TOOL [--between SETUP SETUP] [--audit AGENT] [--random 0.2] [--show-answers] [--limit N]
thinkthen curate settle PICK VALUE [--reason TEXT] [--quote TEXT] [--replaces ID] [--cannot-tell REASON]
thinkthen curate settle --import KEY.jsonl --results RESULTS.jsonl
thinkthen curate key NAME [--tool TOOL] [--trust person|agent] [--held 0.3]
thinkthen curate outcome ANSWER_ID LABEL [--value VALUE] [--note TEXT]
thinkthen curate export NAME@N [--format train|key|cases] [--part tune|held|all] [--easy-share 0.2]
thinkthen eval grade RESULTS KEY                       # 0.2's runs audit, unchanged
thinkthen eval grade TOOL --key NAME@N --setup S [--goal yes:0.95] [--goal no:0.98] [--part tune|held] [--judges] [--cases] [--write]
thinkthen eval compare A B [--key NAME@N] [--part tune|held]   # 0.2's runs diff, plus setups
thinkthen eval bench TOOL [--key NAME@N] --setup S... [--part tune|held] [--plan] [--fresh] [--sample N]
```

- **Grade and compare never send a request,** as 0.2's `runs audit` promises. `eval bench` is the one eval word that sends. It previews, caps and reuses exact stored answers unless told `--fresh`. With no key it runs the tool's own cases, which is the `evals.md` slice.
- **The queue** sorts by the model's own probability nearest the cut, which needs no second model. With `--between` it sorts by disagreement, the promotion question. Imported labels count as a setup, so `--between labels SETUP` lists where the model disagrees with an existing dataset's labels. Each queue mixes in a random share, 20% by default, and is blind by default. Each line is a pick that `settle` names.
- **Judges are graded like setups.** `eval grade --judges` scores each agent against the person-settled answers both share, with an interval. `curate queue --audit AGENT` draws a random sample of an agent's settles for a person to check blind. Keys default to person-settled and audited-agent answers.
- **A goal picks a cut** by testing cuts from the top down on the held part and keeping the lowest whose 95% lower bound meets the goal. Two goals make a band. A goal of 95% needs at least 59 held yeses, all right, before any cut passes.
- **Ordering metrics** for `rank`, `find` and `choose`: right item in the top k, mean reciprocal rank, NDCG from grades, and 0.2's R-precision.
- **GEPA ships as a recipe.** The recipe's own large model rewrites the tool's wording. ThinkThen supplies per-case grading on the tune part, stored wording versions and `eval compare` on the held part. The eval record counts each read of the held part.
- **Rust and the command line only,** by Ian's 2026-10-14 ruling; not MCP. Python and R users read the published views as data frames. Every word works locally and through a proxy.

## What must be true

| Risk | What retires it |
| --- | --- |
| A tool's number and cut are too noisy to use | BANKING77 arm A: intervals and cut steadiness over repeated key draws |
| No queue order beats random on a new domain | Arm B: errors per hundred reviews, unsure against disagreement against random, two models, plus a run with 5% of BANKING77's labels flipped to test disagreement with imported labels |
| Queue-built keys mislead | Arm C: the random-stratum estimate covers the true rate and the queue-only estimate does not |
| An agent settler is wrong too often to trust | Arm D: agent agreement with the labels, and a 30-item audit that bounds it |
| The GEPA recipe cannot run on a tool | Arm E: an end-to-end run with a blind-rewrite arm at equal budget |
| The tables miss a use | A schema review against all eleven rows of `evals.md`'s table before tickets |

Arms A and E admit the slice and `eval bench`. Arms B to D set the queue default and the trust default.

## Out of 0.3

The review screen and keyboard grading. Pairwise right answers and tournaments, which wait on the ranking experiment. Calibration maps. Weighted estimates from queue picks. A drift report. Provider-specific training formats, which ship as recipes. Answering from settled answers with no model call.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 18 and 22. Answered: curation ships in 0.4 under the three-release split, and how curation works (Ian, 2026-10-14).
