# Ideas carried from the design captures

Status: Reference. A backlog of vetted ideas. No work is owed.

Found 2026-09-19. The design captures that came before this repository were checked against every ADR, the roadmap, the plan, and the specification before they were retired. Every verb, option, file format, and exit code in them is accepted, held with a reason, or struck. What the records lacked is advice and a few small rules. Each item names the slice that should pick it up. None of it widens the command surface.

## Advice for help text and pages

- **"Not stated" is a different answer from "false".** "Does the document establish X?" and "Is X true?" are different questions. When the difference matters, the user asks `choose` with labels such as `supported`, `contradicted`, `not_stated`, and `ambiguous`. This sits beside the rule about wording a question so that yes permits the action. Slice 5 (`choose`).
- **Name a saved question for what the evidence shows.** `expressed_cancellation_intent` is a good name and `will_cancel` is a bad one. The name also says whose state it is: "this message expresses frustration" and "this customer is dissatisfied" are different questions. Slice 9 (`annotate`).
- **One number hides the shape of a score.** The level distributions `[0,1,0]` and `[0.5,0,0.5]` both score 1.0. The `score` help says this beside the verb's measured weakness, and a recipe over the stored distribution tells a confident middle from a split. Slice 5 (`score`) and slice 10 (recipes).
- **A chained run keeps `--field` on the original text.** A second pass then never reads the first pass's own answer as evidence. Slice 7 (records).
- **One request can carry a routing question and both branches' questions.** The evidence is billed once, so the unused answers cost nothing and one round trip is saved. Slice 9 (`annotate`), as a worked example.

## Rules for combining answers

- **Combine answers by rule.** Any required no makes no. Otherwise any required unresolved makes unresolved. Otherwise yes. Never multiply the stored probabilities of separate questions into one confidence, because their errors are correlated.
- **A run with several checks has no overall grade unless a rule is written down.** A recipe that prints one applies the rule above and says so.

## Material for the recipes slice

- **A metric needs its definition stated.** A recipe for precision, recall, and F1 declares the label set, the averaging method, and what a zero denominator yields. A case with no label gets a stated policy and is never dropped silently.
- **`//` in `jq` treats `false` and `null` alike.** `.x // false` quietly turns unresolved into no. Every recipe uses an explicit three-way `if`, and every aggregate keeps the unresolved group visible.
- **A comparison of two runs checks more than the case id.** It checks that the paired cases carry the same input and the same trusted label. A changed label means the two runs measured different things.
- **A comparison says why a value changed.** The cause is the evidence, the question, the model, or the cut. `meta.model` and `meta.questions_sha256` tell them apart. The roadmap's entry on history across runs should say the same.
- **A digest names content and does not keep it.** The question file and the case file are stored beside the run. The recipes page says so.
- **Change one threshold in a saved question file with `jq --argjson`,** check the file, then run it. This removes any case for templating inside the tool.
- **Turn judged flags into a tag list** with `to_entries | map(select(.value == true) | .key)`. This is the argument for several yes/no questions over one `choose` read as many labels.
- **Judge nested records by exploding the children with `jq`,** annotating them, and regrouping with `group_by`. A parent with no children disappears from the regrouped output, and the recipe says so.
- **Accumulate over a large run.** `jq -s` loads every row into memory.
- **Bash and `jq` idioms worth a place in `recipes/` or the help:** a predicate returns a status; tell "no match" from "could not search" with `case $?`; keep a failure when capturing output; read every stage of a pipeline with `PIPESTATUS`; read lines without damaging them; pass file names delimited by NUL; command substitution drops trailing newlines; `tee` keeps the `--details` rows while a count runs; clean up temporary files with `trap`; publish a finished file and never overwrite the input; wait for every background job; build JSON with `--arg` and `--argjson`; require exactly one document with `jq -e -s`; reduce a stream one row at a time; join truth labels by id with `INDEX` and `--slurpfile`; cross from `jq` to Bash with `--raw-output0`; keep a policy in an annotated `.jq` file read with `jq -f`; turn examples into executable pages; run `bash -n` and ShellCheck over a generated script.

## Small input rules for the records slice

`specification/records.md` never states these, and slice 7 settles them: refuse duplicate keys in a JSON record, invalid UTF-8, and `NaN` or `Infinity`; strip a carriage return before the line feed under `--lines`; refuse `$.body`, `#/id`, wildcards, and negative indices with a message that names RFC 6901, and never guess a pointer language.

## One safety fixture

The candidate text that an eval grades is untrusted, and it can carry instructions aimed at the judge. Careful wording of the question helps and proves nothing. The eval fixtures of slice 9 include a hostile candidate answer, and the live probe measures whether it moves the judgment.
