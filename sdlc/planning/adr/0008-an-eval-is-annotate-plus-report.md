# ADR 0008: An eval is `annotate` plus `report`

- Status: Proposed
- Date: 2026-09-19

An agent proposal in answer to Ian's six requirements for evals. It becomes Accepted on his word. He can strike any line cheaply, because no code depends on it yet.

## Context

Ian wants to run evals through `thinkthen`: single cases and benchmarks of many cases, reusable definitions, results that keep every probability, artifacts that can be traced, local metrics and comparison, and a way to check the judge against human labels. His central requirement is to separate assessment from interpretation. He likes a flat case record: an `id`, the `input`, the candidate `output`, and an optional `gold`.

ADR 0007 already has both halves. `annotate` obtains named judgments for each record. `report` reads saved results and calls no model. Four things are missing: a flat case cannot show different fields to different checks, a saved row does not name the definition or the tool that made it, `report` has no options, and ADR 0007 sent comparison across runs outside the tool.

## Decision

1. **No eval engine and no `eval` command.** A case is one flat JSON object with any field names. A definition is an `annotate` file. A run is the file of `--details` rows. `report` interprets a run. The whole workflow is two commands with one file between them.
2. **`on` and `--field` take one pointer or several.** Several pointers build an evidence object. Each member is keyed by the last part of its pointer, and two members with one key are a usage error. A correctness check can see `/input`, `/gold`, and `/output`. A grounding check can see `/context` and `/output` and never the gold answer. Nothing else in the case leaves the machine.
3. **A saved row says what made it.** `meta` gains `tool`, the name and version of the binary. Under `annotate`, `meta` gains `questions_sha256`, the digest of the definition file. Each answer gains `request`, the digest that also names its recording entry. The model version that answered is already there.
4. **`report` does the interpreting.**

```text
thinkthen report [RUN] [--truth LEFT=POINTER]... [--threshold NAME=RULE]... [--baseline RUN] [--id POINTER]
```

- With no option it prints, for each check, the counts of yes, no, and unresolved, the label counts, and the score summary. It also prints the run's facts: the models that answered, the definition digests, the tool versions, and how many rows were replayed. A run that mixes two model versions says so.
- `--truth LEFT=POINTER` compares LEFT with a trusted label inside the case. LEFT is a check's name, or a pointer into the case when it starts with `/`. With a check's name it scores the judge. A yes/no check gets the four counts, accuracy, precision, recall, and F1. Unresolved rows are counted apart and are never scored as right or wrong. The same table is printed for each cut in a sweep, and a calibration table sets each probability band beside the share of cases that were truly yes. A `choose` check gets precision, recall, and F1 for each label, and the macro average.
- `--truth /POINTER=/POINTER` compares two fields of the case exactly. No judgment is involved. This covers a benchmark where the candidate's label meets a gold label.
- `--threshold NAME=RULE` applies another rule to the stored probabilities. No request is made.
- `--baseline RUN` matches cases by `--id`, which defaults to `/id`. It prints the change in every metric, the cases that flipped in each direction, and the cases present in only one run. Rows that share an id inside one run are repeated trials. A metric averages within a case first, so a case tried five times weighs the same as a case tried once.

5. **A missing field and a failed request stop the run.** Exit 2 or 4 names the record. A rerun on the same recording folder answers the finished cases from disk. A completed run therefore holds a judgment for every case. `--on-error continue` stays on the roadmap, and a long benchmark is the job most likely to bring it in.
6. **Exact checks beyond equality are `jq` fields.** The definition file holds questions and nothing else.
7. **History across many runs stays outside the tool.** `report` compares two files and keeps nothing.

Three details came out of writing demos 13 and 14. `NAME=` may be left out of `--truth` and `--threshold` when the run holds a single check, as a run made by `decide --jsonl --details` does. An exact check is keyed in the report by its left pointer. `report.md` fixes the JSON shape of the report before any code is written, because only scripts read it. The independent review wrote that shape, with a sweep of 19 cuts from 0.05 to 0.95 and ten probability bands. Both numbers are Draft.

Every option of `report` is Draft until demo 13 (pick a threshold) and a new demo 14 (grade a batch and compare it with a baseline) drive it.

## How many requests each command makes

One request carries one piece of evidence and every question asked of it. The backend answers the questions of one request independently. Two pieces of evidence never share a request, because records must not see each other and a check must not see a field outside its `on`.

| Command | Requests | Why |
| --- | --- | --- |
| `decide`, `choose`, `score` on one document | 1 | One evidence, one question |
| `annotate` on one document | 1 for each distinct `on` | Every question with the same evidence rides together |
| `decide`, `choose`, `score`, `filter`, `rank` over N records | N | One evidence per record. `rank` sorts locally and makes no pairwise calls |
| `annotate` over N records | N times the distinct `on` sets | As above |
| `segment` | 1 | One evidence, one question per gap. ADR 0009 drops the windowed form |
| `report`, `config`, `--dry-run`, `--replay` | 0 | |

Every request inside one command is independent of every other, so a command is one round, and the round runs in parallel up to the `jobs` setting with output order kept. The vendor's rate limit is the real ceiling. No version-one command needs an earlier answer to form a later request. The held verbs `reduce`, `state`, and `patch` do, and that is one more reason they are held.

## Consequences

`annotate.md`, `report.md`, `records.md`, and `result.md` change. Demo 14 is added. The roadmap entry for comparison across runs narrows to history. Slice 9 and slice 10 of the plan carry the work.
