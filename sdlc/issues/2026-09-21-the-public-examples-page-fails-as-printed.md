# The public examples page fails as printed

Status: Closed on 2026-09-22. The three blocks are corrected in the marketing repository at `6354fb4`, the page is marked planning only, and the marketing README rules that a public example lives only in the deck's recorded `examples/` and `usecases/` folders.

The marketing repository's `products/thinkthen/examples.md` page promises "Before any example goes public it runs against the shipped binary." Two of its blocks fail as printed, and its header names the built state wrong. The fixes land in the marketing repository. This issue is the record.

## 1. The score transform line omits its argument

The page prints `jq -n -f transforms/score/score.jq run.jsonl` with a comment promising accuracy, precision, recall, and F1. Run as printed, 2026-09-21:

    $ thinkthen decide @refund.json --jsonl --field /body --details < labeled.jsonl > run.jsonl
    $ jq -n -f transforms/score/score.jq run.jsonl
    jq: error: $cut is not defined at <top-level>, line 67:
        cut: $cut,
    jq: 1 compile error

The transform's own header requires `jq -n --argjson cut 0.5 -f score.jq ROWS`, and every in-repo demo passes it that way (demos 13, 25, 41). The orchestrator reproduced the failure independently. The sweep line beside it runs as printed.

## 2. The annotate question set is refused

The page's block is a bare object of questions. The grammar needs `{"version": 1, "questions": {...}}`. Run as printed:

    $ thinkthen annotate triage.json --jsonl --field /body < inbox.jsonl
    thinkthen: the question set holds no key `unresolved`
    exit 5

With the wrapper and the version the same command prints its annotated object and exits 0. The vocabulary page itself says the annotate line needs a question set with `"version": 1`, so the marketing side knows the rule. The refusal message is its own issue, `2026-09-21-question-file-refusals-name-the-wrong-thing.md`.

## 3. The header says annotate and find are unbuilt

The page's line 4 says `annotate` and `find` are specified and unbuilt. Both are built: both help pages render, demos 15 and 16 are green, and the deck's own examples replay both verbs byte-identically. The page undersells the binary on the eve of a release whose slides show both working.

## How bad it is for a user

Major. A new user copying the page hits a jq compile error and an exit 5, on a page that promises every example ran.

Found by experiment 218, wave 1, area 1.
