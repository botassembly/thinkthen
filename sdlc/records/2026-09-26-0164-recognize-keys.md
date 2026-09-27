# Ticket 0164: the recognize keys in the repository

Build record of 2026-09-26. No test sends a request, and no step made a model call.

## What landed

- `specification/fixtures/recognize/names.jsonl`: local experiment 277's 200-sentence key, 372 names, as `audit` key lines.
- `specification/fixtures/recognize/relations.jsonl`: local experiment 265's 30 sentences, 73 names, 27 stated edges, 5 optional edges and 5 unstated edges.
- `specification/fixtures/recognize/kinds.jq` filters either key to a run's kinds.
- `specification/fixtures/recognize/README.md` gives the sources, license, counts, 29 rules, agreement figure and known limits. Its blocks grade both keys with the built `audit`.
- `sdlc/scripts/recognize-keys` converts the sources and checks both files. `lint` runs its self-test and its check. `spec` runs the README.
- `core/recognize.rs` gains one test that counts reach under today's splitter.

## How the files were made

The sources stay outside the repository, so no rung can rerun the conversion. A reviewer with the experiments reruns these two commands and compares bytes. The first `SOURCE` is local experiment 277's `cases.jsonl`, and the second is local experiment 265's `cases/cases.jsonl`.

```sh
sdlc/scripts/recognize-keys convert names SOURCE > specification/fixtures/recognize/names.jsonl
sdlc/scripts/recognize-keys convert relations SOURCE > specification/fixtures/recognize/relations.jsonl
```

- Local experiment 277's `cases.jsonl`: SHA-256 `c361329c6d5b16f816f9d3059f42f010a15d74c47130f7fff474c597227abdc8`, 62,057 bytes. Its first 100 lines equal local experiment 267's `cases.jsonl` byte for byte, SHA-256 `87b49d13e01d6f641aaf9e5aeb1521fc859210f155b27a7e27ce677c3011ded5`.
- Local experiment 265's `cases/cases.jsonl`: SHA-256 `3b63bb99b5f6c0cf11198e9b4c34c80c01634d57da2e743c6aca0a67e2fc1a6e`, at its commit `2aea0a7`.

A field-by-field comparison against the source found every id, text, category, name, kind and offset unchanged. Only the notes of n001, n017, n019, n024 and n034 differ, by "Local experiment 265" for "Experiment 265".

Stop rule 2: no four-word run of any sentence in either key appears in local experiment 267's 25-group WNUT-17 sample. Local experiment 277's report says every sentence was written for the key.

## Proof

| Test | Result |
| --- | --- |
| 1. `recognize-keys --self-test`, then the check over both files, on `lint` | Plants (a) to (f) each refused with its pinned sentence, and a clean two-line file passes. Both committed files pass: `names.jsonl: 200 lines, 372 names, every offset exact` and `relations.jsonl: 30 lines, 73 names, every offset exact` |
| 2. `mustmatch test specification/fixtures/recognize/README.md`, on `spec` | Every line of the ticket's proof test 2 printed as pinned |
| 3. The reach test in `core/recognize.rs`, on `test` | 322 of 372 names reachable, 1,735 words |

Plants for test 2: each pinned line was changed by one digit, and its block turned red. A pinned line cut short by its last measure also turned red.

Plants for test 3: (m) `split_piece` no longer peels `,`, and the test failed with 310 and 1,709. (n) n001's `Something` given `end` 33, and the test failed with 321, while the check refused the file with `names.jsonl line 1: text[23:33] is 'Something ', not 'Something'`.

## Gaps found in the build

- Proof (j) and (j2) add their extra name to lines that keep names under the filter, n018 and n143. A `kinds.jq` that dropped emptied lines would still pass both blocks. No block adds an extra name to a line whose names all drop.
