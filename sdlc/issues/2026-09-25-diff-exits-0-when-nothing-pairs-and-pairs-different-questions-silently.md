# diff exits 0 when nothing pairs, and pairs different questions silently

Status: Open

Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

## What happens

Two related gaps.

1. `thinkthen diff A B` exits 0 when no answer pairs. It prints `0 of 0 changed` and McNemar p 1.0. This holds when the ids do not match, when B is empty, and when both files are empty. A script sees the same result as two runs that agree.
2. diff pairs answers by answer name and record id. It pairs two runs of different questions without a word. Every detailed result line carries `meta.question_sha256`, so diff holds what it needs to notice.

## Reproduction

Offline, from committed Beatles Bench results. The `11-audit` rows ask the Abbey Road question. The `01-decide` outputs hold four rows that ask a love-song question about four of the same titles. `--id /input` pairs by title.

    $ BB=path/to/beatles-bench
    $ jq -c 'select(.input.input == "She Loves You" or .input.input == "Michelle"
              or .input.input == "Yesterday" or .input.input == "Taxman")' \
        $BB/examples/11-audit/rows.jsonl > abbey.jsonl
    $ jq -c '.rows[] | select(.question.text | test("love song"))' \
        $BB/examples/01-decide/outputs.jsonl > love.jsonl
    $ jq -r '.meta.question_sha256[0:12]' abbey.jsonl love.jsonl | uniq -c
          4 c86e44e03108
          4 34c67483d67b

Different questions pair with no warning:

    $ thinkthen diff abbey.jsonl love.jsonl --id /input --table; echo "exit $?"
    Taxman  yes -> no  p 0.56 -> 0.06
    Michelle  no -> yes  p 0.12 -> 0.75
    Yesterday  no -> yes  p 0.10 -> 0.61
    She Loves You  no -> yes  p 0.07 -> 0.95
    A -> B: 4 of 4 changed; no -> yes 3; yes -> no 1; McNemar p 0.625 on yes answers
    exit 0

Nothing pairs, and diff still exits 0:

    $ : > empty.jsonl
    $ thinkthen diff abbey.jsonl empty.jsonl --id /input --table; echo "exit $?"
    A -> B: 0 of 0 changed; McNemar p 1.000 on yes answers; only in A 4, only in B 0
    exit 0

    $ thinkthen diff empty.jsonl empty.jsonl; echo "exit $?"
    {"summary":{"records":0,"changed":0,"only_a":0,"only_b":0,"moves":[],...,"mcnemar_p":1.0,...}}
    exit 0

    $ thinkthen diff $BB/examples/11-audit/rows.jsonl $BB/examples/11-audit/rows-context.jsonl --table; echo "exit $?"
    A -> B: 0 of 0 changed; McNemar p 1.000 on yes answers; only in A 70, only in B 70
    exit 0

The last case is the bench's own cold and context runs. Their ids differ (`audit-01` against `audit-context-01`), so nothing pairs. `thinkthen diff abbey.jsonl /dev/null` gives the same result as the empty file.

## What the spec says

- `specification/diff.md`, "Output": diff "prints one JSON object per changed pair in A's order, then a summary line, and exits 0."
- `specification/diff.md`, McNemar: "`p` is 1 when `n` is 0."
- `specification/diff.md`: "diff pairs answers by answer name and record id only. It does not check that the two runs saw the same record text or the same question. The `compare` transform checks both."
- Ticket 0114, decision 3, holds the same line and adds that extra members would break the goldens. ADR 0021 gives the question-digest check to `compare`.

The spec decides both behaviors today. A change needs a spec edit.

## Why it matters to a user

diff answers "did anything change between these runs". A pipeline that feeds diff an empty file after a failed step, or two runs keyed by different ids, gets "0 of 0 changed", p 1.0, and exit 0. The pipeline reads that as "no change". A user who compares the wrong two files gets a confident table and a McNemar p for a comparison with no meaning. The data to catch both cases is already in the input.

## Options

For the empty pairing:

1. Exit with a distinct nonzero code when `records` is 0 and either side had answers, or when both sides are empty. The summary line still prints.
2. Keep exit 0, and print one warning on standard error when nothing paired. The table and JSON stay the same, so the goldens hold.
3. Print `mcnemar_p` as null when `n` is 0, so "no test ran" differs from "no evidence of change".

For different questions:

4. Compare `meta.question_sha256` within each pair. Print one warning on standard error with the count of pairs whose digests differ. The goldens hold.
5. Refuse at exit 2 when paired digests differ, with an option to allow it on purpose.
6. Add a `question_mismatch` count to the summary. This adds a member and moves the goldens.

Options 2 and 4 change no stdout byte. The others change the diff contract and need a spec change and a ruling.
