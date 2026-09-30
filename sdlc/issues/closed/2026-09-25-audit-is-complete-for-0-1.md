# audit is complete for 0.1

Status: Closed on 2026-09-25 by ticket 0125. `recognize` and `relate` grading moved to `sdlc/issues/closed/2026-09-25-audit-grades-recognize-and-relate.md`, which stays open.

Ian wants `audit` right for 0.1. It grades every function type, scores by the measure the user picks, shows how steady its suggested bar is, and hands that bar back to the question file. This issue merges four issues filed on 2026-09-25, and their evidence is kept below:

- `audit-cannot-grade-score-or-tune-its-level-cuts`
- `audit-picks-its-cut-only-by-most-right-answers`
- `audit-shows-one-seeded-cut-and-hides-its-spread`
- `audit-suggests-a-bar-and-the-user-copies-it-by-hand`

This overturns two earlier limits:

- Ticket 0113 defers verbs other than `decide` and `choose` "until after 0.1".
- `closed/2026-09-24-audit-and-diff-needs-for-graded-agent-runs.md` holds graded scores for after 0.1.

Its other three needs stay after 0.1: cost per row, repeated samples, and run identity.

The comparison with DSPy 3.4's ReAnchor optimizer is Ian's inbox report `notes/reports/2026-09-25-dspy-jev-support-vs-audit-and-diff.md` in the workspace. ReAnchor tunes only the bars: a yes/no cut, the cuts between `score` levels, and a weight for each `choose` option. It maximizes a measure the user writes. It keeps the current bar unless another bar scores strictly better on up to five splits of the data.

## The loop Ian expects

1. audit grades saved answers against a key.
2. It reports accuracy, precision, recall, and F1.
3. The user picks one of the four measures.
4. audit gives the bar that does best on that measure.
5. The bar goes into the question file, so every later run uses it (question-file rule 8).

Today steps 1 and 4 work with accuracy only, and step 5 is a copy by hand. audit writes only standard output ([audit.md](../../../specification/audit.md), "audit routes before any setup").

## 1. Every function type

Today audit grades `decide` and `choose` and refuses any other verb at exit 2 ([audit.md](../../../specification/audit.md), "The verb"). It checks each member of an `annotate` row. A question set with one `score` or `tag` question is refused whole, and the questions audit could grade go unchecked too. The `annotate` example in the spec mixes `decide`, `choose`, and `score`, so auditing it fails today.

| Function | What a key holds | What audit grades | Bar it tunes |
| --- | --- | --- | --- |
| `decide` | yes or no | Done today | The yes/no cut |
| `filter` | yes or no | As `decide`, from `decide --details` or a filter run's detailed rows. Kept rows alone cannot show misses | The cut |
| `choose` | the right option | Done today | The cut to reach a target. Option weights are open, see below |
| `find` | the right candidate, or `none` | As `choose`, `none` included. Blocked until `2026-09-25-docs-how-tos-and-spec-claims-owed.md` is fixed | The cut |
| `tag` | the labels that apply | One group per label, graded as yes/no. The labels ride in one request, so their errors move together and are never pooled | One cut per label, or one shared cut |
| `score` | the right level | Exact-level agreement and the mean distance in levels. A miss by one level is closer than a miss by two | The cuts between levels |
| `rank` | the relevant records, or an order | Measures of order, such as how many relevant records reach the top N | None. `rank` takes no cut |
| `annotate` | an object with one member per question | Each member by its own verb | Each member's bar |
| `recognize` | the names, with their kinds and places | Precision, recall, and F1 over names | The strength cut |
| `relate` | the edges | Precision, recall, and F1 over edges | The probability cut |

A `score` level cut is the point where one level ends and the next begins. It works like the yes/no bar, with one bar between each pair of levels. On a three-level email task in the DSPy post, moving the level cuts from 0.5/1.5 to 0.1/1.1 raised right answers from 239 to 294 of 336.

Open design points:

- For `tag`, whether a label missing from the key means "no" or "not labeled".
- For `rank`, which order measure to use.
- For `recognize`, which name matches count: the same text and kind, or overlapping places.
- For `choose`, weighting options. This changes how `choose` picks and needs an ADR.

## 2. The user picks the measure

The suggested `decide` cut is the bar with the most right answers on the tuning part. The user cannot say which mistake costs more. The output prints `yes_recall` but no precision and no F1.

Take the Beatles Bench audit example (`examples/11-audit`): the question "It appears on the album Abbey Road." over 70 songs, 7 of them on Abbey Road.

| Bar | Caught of 7 | Wrong yes | Right of 70 | Precision | Recall | F1 |
| --- | --- | --- | --- | --- | --- | --- |
| 0.5 | 7 | 14 | 56 | 0.33 | 1.00 | 0.50 |
| 0.75 | 7 | 5 | 65 | 0.58 | 1.00 | 0.74 |
| 0.85 | 5 | 2 | 66 | 0.71 | 0.71 | 0.71 |

audit suggests 0.85. F1 picks 0.75, which catches all seven. The four measures pick three different bars on this data: accuracy 0.85, precision 0.95, recall 0.75, F1 0.73.

Ian's design, 2026-09-25:

1. `audit --optimize accuracy|precision|recall|f1` sets the measure the suggested cut maximizes. `accuracy` stays the default.
2. Tie rules:
   - Recall takes the highest bar with the top recall. A bar of 0 also reaches 100%, but it says yes to everything.
   - Precision takes the lowest bar with the top precision.
   - Accuracy and F1 keep today's rule: nearest to 0.5, then the smaller bar.
3. The output adds `precision` and `f1` beside `yes_recall`, and `--table` prints all four at the suggested cut.
4. An optional `--all` prints the suggested cut for each of the four measures in one run.

The suggested object already names its `objective`, so the output shape holds. Each function type in section 1 names which of the four measures apply to it.

## 3. The suggested bar shows how steady it is

With no `part` in the key, audit splits the labeled ids in half with a generator seeded by `--seed` (default 0). On small data the suggested cut depends heavily on the seed. The table does not print the seed.

On the same 70-song example, seeds 0 to 199 suggest cuts from 0.55 to 0.95. The most common is 0.85 (50 seeds), then 0.73 (47), then 0.83 (27). With seed 0 the tuning half holds only 2 of the 7 positives. Reproduce it offline from `beatles-bench/examples/11-audit`:

    for s in $(seq 0 199); do
      thinkthen audit rows.jsonl key.jsonl --id /input --seed $s | jq -r .suggested.cut; done |
      sort -n | uniq -c | sort -rn | head -3

Options:

1. Print the seed in the table line.
2. Add a note when the tuning half is small, pointing to more labels or a `part` column in the key.
3. Run over a fixed set of splits and print the most common cut and its range. ReAnchor's rule fits here: keep the current bar unless another bar scores strictly better across the splits.
4. Teach the `part` column on the audit page and in the help.

Options 1, 2, and 4 change wording. Option 3 changes what audit computes and needs a spec change.

## 4. The bar goes back into the question file

Options:

1. audit prints a question-file line, ready to paste, beside each suggestion.
2. `audit --write QUESTIONS` writes the suggested bar into a question file the user names. This fits the rule that the tool writes only files the user names. It needs a test that the rest of the file stays byte for byte.

The filed recommendation was 1 first and 2 if users ask.

## Timing

- Tickets 0113 (audit) and 0114 (diff) landed on main with the prototype's `decide` and `choose` grading. A new ticket builds this issue on top of them.
- Everything here ships in 0.1. The Beatles Bench goldens stay the definition of today's behavior. New behavior gets new goldens.
- Ian can overturn any design point above. The ruling that audit is complete in 0.1 is his.
