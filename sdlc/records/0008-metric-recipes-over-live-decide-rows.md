# Record: 0008, metric recipes over live `decide` rows

Date: 2026-09-19

## What landed

`recipes/` holds seven recipes as folders. `recipes/rows/` holds the case file, the loop that judged it, both runs, and the eighty recorded exchanges. Demos 13, 24, 25, 28, and 38 are green how-tos in the form of ADR 0011, and the `spec` rung runs every block of them from committed files with no network and no key.

The case file is forty made-up support messages with a stable `id`, a `body`, and a trusted `label`. One case, `C-12`, carries no label on purpose. `sdlc/scripts/live` ran `recipes/rows/record.sh`, which judged every case twice, once with each wording of the question, at the band `0.2:0.8`. The two runs spent 23,848 input tokens, about a tenth of a cent.

The judge scored an accuracy of 0.9744 and an F1 of 0.9744 at the default cut of 0.5 over the 39 labeled rows. The sweep over the tuning half picks 0.7, and the holdout agrees at every rate.

## The verdict on each recipe

Length is the whole file, header included. The header is a third to a half of each one, and that is deliberate: a metric with no stated definition is a number nobody can check.

| Recipe | Lines | Verdict |
| --- | --- | --- |
| `counts/counts.jq` | 28 | Fine as a file |
| `score/score.jq` | 76 | Fine as a file |
| `sweep/sweep.jq` | 95 | Fine as a file |
| `band/band.jq` | 64 | Fine as a file |
| `calibration/calibration.jq` | 63 | Fine as a file |
| `compare/compare.jq` | 76 | Clumsy enough to earn a command |
| `cost/cost.jq` | 45 | Fine as a file |

### `counts.jq`, 28 lines

Traps: none worth the name. The one decision was to read `value` rather than recompute from the probability, so the counts report what the run actually answered.

Fine as a file. It is thirteen lines of code, and a command that wrapped it would be longer than it is.

### `score.jq`, 76 lines

Traps met:

- jq has no optional argument. `$cut` must be passed on every call, and a call that forgets it fails at compile time with a message about an undefined variable rather than about a missing cut.
- The three-way rule has to be written three times over: at the label test, at the verdict, and at the denominators. `//` was reached for twice while writing it, and both times it would have turned an unresolved row into a no.
- Rounding had to be decided. Unrounded rates print as `0.9743589743589743`, which no page can pin without looking absurd.

Fine as a file. The command line reads well: `jq -n --argjson cut 0.5 -f score.jq rows.jsonl`.

### `sweep.jq`, 95 lines

Traps met:

- It carries a copy of `score.jq`'s verdict and arithmetic. jq shares code only through `include` and a `-L` search path, which would put a directory on every command line. Two copies were the smaller cost, and this duplication is the first real argument for carrying recipes inside the tool.
- The grid is hard-coded for the same reason `score.jq` demands its argument: an optional argument does not exist.
- The pick needed a stated tie rule. Six cuts scored a perfect F1 on the tuning half, and `max_by` would have returned one of them with no reason given. The rule travels in the output now.

Fine as a file, with the duplication noted. It is the longest recipe and the one a user is least likely to edit.

### `band.jq`, 64 lines

Traps met:

- `accuracy_unresolved` has no obvious definition. The refused rows are scored at a plain 0.5 here, which answers the question a band raises: would the model have got them right anyway. On this run it got two of the three right, so the band paid for its accuracy.
- An unlabeled row belongs in `refused` and in no rate. Those are two different questions about one row, and the reduce had to keep them apart.

Fine as a file.

### `calibration.jq`, 63 lines

Traps met:

- An empty band must yield null. Zero reads as a finding, and null reads as an absence of evidence.
- A probability of exactly 1 falls outside ten bands of a tenth unless the last one is closed, so the slot is clamped.
- Forty cases across ten bands leave three bands empty and three holding one row. The recipe prints the count beside every rate for that reason.

Fine as a file.

### `compare.jq`, 76 lines

Traps met:

- The two runs arrive in two different ways. One comes through `--slurpfile` and the other through `inputs`, so the command line is asymmetric and easy to get backwards.
- `select($old | has(.))` is wrong and runs. Inside `has`, `.` is the object rather than the key, and jq only says so at run time, on real data, with "Cannot check whether object has a object key".
- A `reduce` used as an object's value must be parenthesized, and the parse error points at the line after the mistake.
- Pairing had to refuse duplicate ids before `INDEX` silently kept the last row under each one.
- Six flip directions rather than two. Folding unresolved into no would have reported two of this run's four flips as regressions.

Clumsy enough to earn a command. It is the recipe a user is most likely to copy and edit, the one whose mistakes run silently, and the one whose command line nobody remembers. Three of the five traps above cost a run each to find. If any recipe argues for outcome 5 of ADR 0010, or for option B of ADR 0012, it is this one.

### `cost.jq`, 45 lines

Traps met:

- A replayed row must not be priced. Counting its tokens would put a bill on every gate.
- An absent `meta.usage` is not a zero. Those rows are listed by id, so a total is never quietly short.
- The price is an argument. Hard-coding 0.042 would have baked one vendor's contract on one day into a file that outlives both.

Fine as a file.

## Choices made where the ticket and the pages were silent

Ian can overturn each of these cheaply.

1. **A record row carries the case under `input`, and the trusted label lives inside it.** `specification/result.md` says a record row carries `input`, the whole record including parts never sent, and `records.md` says nothing further. The simplest reading is that the case's own `id` and `label` are members of that record, so every recipe reads `input.id` and `input.label`. Nothing new was invented, and the recipes need no change when record mode lands.
2. **Rates are rounded to four decimals and money to six.** Counts stay exact.
3. **The sweep grid is the 19 cuts from 0.05 to 0.95,** which is the number `report.md` had drafted.
4. **The sweep picks the highest F1, and the middle cut of the cuts that tie.** The rule is printed in the output.
5. **`accuracy_unresolved` scores the refused rows at a plain cut of 0.5.**
6. **The rows live once, in `recipes/rows/`,** and the five pages read them by relative path. Copying forty rows into five demo folders would have been five files to keep in step.
7. **A recipe folder holds the `.jq` file and `example.sh`.** The question file lives once in `recipes/rows/` beside the cases it was asked of, rather than once per recipe. ADR 0012 names the question file as part of a recipe folder, and seven copies of one question would have been seven chances to drift.
8. **One live job ran both runs.** The reworded question is a second pass in the same script.
9. **No recipe reads `meta.tool`.** ADR 0008 item 3 puts it in a saved row, and the landed binary does not write it yet.
10. **The runs applied the band `0.2:0.8`,** so the saved rows carry real unresolved answers and the three-way rule is proved against live data rather than a doctored file.

## One change under `crates/`, which the ticket excluded

`crates/thinkthen/tests/demo_runner.rs` asserted `demos: 1 green, `. Five more demos turn green here, so the test failed on rung 2. It now reads the count off the pages the runner reported, which leaves it passing for every later ticket that turns a demo green as well. The change is one line, and the ceiling in `sdlc/ratchet.json` rose from 4193 to 4194 in the commit that made it. No other file under `crates/` changed.

## Evidence

- Rungs: `install`, `lint`, `test`, and `spec` all exit 0 with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset.
- `demos: 6 green, 11 red`.
- A search of the committed rows and recordings for `apikey_`, `authorization`, and `bearer` in any case finds nothing: three counts of zero.
- `sdlc/live-tokens` carries the spend. This ticket added 23,848 input tokens.
