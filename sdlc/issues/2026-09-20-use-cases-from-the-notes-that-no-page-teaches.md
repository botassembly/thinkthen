# Use cases from the notes that no page teaches

Status: Open

Found 2026-09-20 in a survey Ian asked for. Three readers went through his notes of the last three days, every captured input about the decider model, and this repo's own `ten-use-cases.md`, how-to list, and `specification/roadmap.md`. They found 52 use cases. Ian's question was whether every one is possible with the verbs, with Unix pipes allowed, and whether each is as simple as it can be.

## The finding

No use case needs a new verb. Every miss is a missing recipe, a missing small flag, or a job the tool refuses on purpose. About a third of the list waits on `annotate` or `find`.

## Missing pages

Each of these works today or at version one, and no how-to teaches it.

1. **Judge pairs.** Semantic diff, join, dedupe, set comparison, and claim verification all reduce to one shape. Code builds candidate pairs. One `decide` or `choose` with two `--field` pointers judges each pair. Code groups the result. One page covers all five.
2. **Semantic diff.** `diff` or `git diff` finds the changed hunks, and `filter` keeps the hunks that change what the text requires. How-to 43 lints a change. No page compares two versions of a document.
3. **Grep a directory by meaning.** A shell user expects `grep -rn`. Today it takes `jq -Rc '{file: input_filename, n: input_line_number, text: .}'`, then `filter --jsonl --field /text`, then a `jq -r` line to print `file:line: text`. A page should show it. Linting a whole tree of code or documents, file by file, is the same page.
4. **Fill a form by selection.** A closed field is a `choose`. An open field is a `find` over candidates that code lists: lines, regular-expression hits, a roster. The value is copied and no character comes from a model. This is the tool's answer to every "extract" request, and it needs `find`.
5. **Diagnose a failed agent trace.** `find --none` picks the step where the run went wrong, and `choose` names the kind of error. It is the one software-delivery use case in Ian's notes with no page at all.
6. **Split a document where the topic changes.** `awk` builds rows that hold the previous line and the current line. `decide` marks the cuts. `csplit` cuts. `segment` stays held until this recipe proves too clumsy.
7. **Filter database rows in plain English.** `psql -Atc "select row_to_json(t) from tickets t" | thinkthen filter 'Q' --jsonl`. It needs no database extension, and it demonstrates well.
8. **A linter for prose.** `awk -v RS=` makes paragraph rows, and `annotate` adds several flags to each. It needs `annotate`.
9. **Many labels at once.** A multi-label classification is `annotate` with one `decide` per label. The `choose` help should say so, because a user looks there first.

## Small flags worth reconsidering

1. **`filter --invert`.** The roadmap holds it and says to word the question the other way. A reworded question is a different measurement with a different threshold. Keeping the records that did not reach the mark is not the same as keeping the records that reach the mark of the opposite question. Grep users reach for `-v` on the first day. Today the detour is `decide --details` and `jq 'select(.answer.probability < 0.5)'`.
2. **Line numbers and file names on text records.** This is `grep -n` and `grep -l`. The `jq` recipe above works and is long. A flag could wait until the page shows whether people copy the recipe.
3. **CSV input.** The roadmap's workaround converts to CSV and does not read it. A page should show `mlr --icsv --ojsonl cat` in front of the tool.
4. **A baseline or an examples file.** Outlier ranking and "more like these" both want reference text beside the question. `--context` is held, and concatenating into standard input mixes the reference with the evidence. The `--true` and `--false` texts may already be the right home. A page should settle it.

## Refusals that need a named neighbor

Summarize, rewrite, redact a span, cluster with invented names, extract a free-form graph, pick a diverse sample, and run inside a real-time loop. The tool refuses each on purpose. The documents should name which tool does each job and show the hand-off in a pipe, so a reader who arrives with one of these leaves with an answer.
