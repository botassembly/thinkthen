# Two-function flows lose the record between stages

Status: Open

The marketing side drafted ten use-case examples on 2026-09-21, each joining two or three functions with a pipe. Every stage ran under `--dry-run` at exit 0. The drafts are in the marketing repository under `decks/2026-09-21-thinkthen-semantic-commands/usecases/`. Composition is the product's promise, and these are the places where it cost the user something.

## The main finding: the answer and its record part ways

`filter`, `rank`, and `find` print records, so the next function can read them. `decide`, `score`, `choose`, `tag`, and `annotate` over `--lines` print the answer alone. Three costs follow.

1. **Keeping the records where a `decide` band says "not sure", or where a `score` is low, costs a `jq` line:** `--details ... | jq -r 'select(.value == null) | .input'`. It is the widest line on two of the ten slides, and a new user has to know the details shape to write it.
2. **`annotate --lines` prints the answers with no message beside them.** The record survives only as a JSON object: `jq -R '{body: .}'` and then `annotate --jsonl --field /body`.
3. **A three-function chain ends with bare values.** `filter | rank | choose --lines` prints two team names and no lead beside either.

A user expects to see the answer next to the thing it is about. The builder owns the design. Two shapes worth weighing: a flag on the value-printing functions that prints the record with its answer in the input's own framing, and a `filter` that accepts a band and a choice of which pile to keep (yes, no, or not sure). The second would turn candidate 3 into one plain line.

## Smaller findings

4. **`find` takes one text.** Matching an alert to a list of incidents, or an expense to a policy, puts the alert inside the question string through a shell variable. It works. The specification forbids treating free text as an instruction, and a question built from untrusted text deserves a sentence in the manual.
5. **`choose` in a pipe needs `--raw`,** or the next stage reads the quotation marks. The `choose` help should say so in its pipe example.
6. **`decide` on a pipe reads the whole stream as one document unless `--lines` is given.** This is right and easy to forget. A how-to on joining functions says it once.
7. **`--dry-run` in a pipeline shows only the last stage's plan.** A user who wants to see what a whole pipeline would send has to run each stage alone. The request count before a big run is a trust item already on the trust list.

## What Ian can overturn

All of it. None of this blocks the current task list.
