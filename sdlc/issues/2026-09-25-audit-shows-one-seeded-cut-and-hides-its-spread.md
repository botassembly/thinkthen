# audit shows one seeded cut and hides its spread

Status: Open

Filed on 2026-09-25 from experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

Related: `2026-09-25-audit-picks-its-cut-only-by-most-right-answers.md` asks which score the cut maximizes. This issue asks how steady the cut is under any score. A fix to one does not fix the other.

## What happens

With no `part` in the key, audit splits the labeled ids in half with a generator seeded by `--seed` (default 0). It tunes the cut on one half. On small data, the suggested cut depends heavily on the seed. The user sees one draw. The table does not print the seed number, and nothing shows how far other seeds would move the cut.

On the Beatles Bench audit example (70 songs, 7 of them on Abbey Road), seeds 0 to 199 suggest cuts from 0.55 to 0.95. The most common cut is 0.85, in 50 of 200 seeds. 0.73 follows in 47, then 0.83 in 27. With seed 0, the tuning half holds only 2 of the 7 positives.

## Reproduction

Offline, from committed Beatles Bench files.

    $ cd path/to/beatles-bench/examples/11-audit
    $ for s in 0 1 7; do
        thinkthen audit rows.jsonl key.jsonl --id /input --seed $s |
          jq -c '.suggested | {cut, seed, split, held: .held.at_cut.agreement}'; done
    {"cut":0.85,"seed":0,"split":"seeded","held":0.942857}
    {"cut":0.73,"seed":1,"split":"seeded","held":0.914286}
    {"cut":0.73,"seed":7,"split":"seeded","held":0.942857}

    $ for s in $(seq 0 199); do
        thinkthen audit rows.jsonl key.jsonl --id /input --seed $s | jq -r .suggested.cut; done > cuts.txt
    $ sort -n cuts.txt | uniq -c | sort -rn | head -3
         50 0.85
         47 0.73
         27 0.83
    $ sort -n cuts.txt | sed -n '1p;$p'
    0.55
    0.95

    $ thinkthen audit rows.jsonl key.jsonl --id /input --table | grep suggested
      suggested cut 0.85 (most agreement on the tuning part; seeded split, tuned on 35, checked on 35 held out): held agreement 0.800 as run -> 0.943 at the cut, yes recall 1.000 -> 0.800

The JSON names the seed. The table says "seeded split" and does not name it.

## What the spec says

- `specification/audit.md`: `--seed N` "seeds the split and the bootstrap", default 0.
- `specification/audit.md`, "The split": when no id has a `part`, a generator seeded with `--seed` shuffles the ids, the first half tunes, and the rest are held out. When every id has a `part`, the split is `"key"` and no seed applies.
- `specification/audit.md`, "The suggested cut": the most right answers on the tuning part wins, with the tie rule. The page says nothing about the cut's spread across seeds.
- The calibration note already warns that its interval "does not cover rerun noise". The suggested cut carries no such note.

## Why it matters to a user

A user runs audit once, reads "suggested cut 0.85", and puts 0.85 in a pipeline. Another seed on the same rows would have said 0.73 or 0.55. The one number looks like a finding about the data. On a small key it is one sample from a wide spread. The held-out figure makes the pick look checked, but the held half is as small as the tuning half. The user has no sign to rerun with other seeds or to label more rows.

## Options

1. Print the seed number in the table line, so two runs that differ can be traced.
2. Add a note to the suggested cut when the tuning half is small, for example under a fixed count of labeled positives. The note says the cut can move with `--seed` and suggests more labels or a `part` column in the key.
3. Run the split over a fixed set of seeds and print the spread: the most common cut and its range. The single-seed pick stays available with `--seed`.
4. Pick the cut on all labeled rows when the data is small, and report no held-out figure. This drops the check the split exists for.
5. Teach the `part` column in the key on the audit page and in the help. A key split needs no seed and gives the same cut every run.

Options 1, 2, and 5 change wording or add a note. Options 3 and 4 change what audit computes and need a spec change and a ruling. Any of them can sit beside the `--optimize` asks in the related issue.
