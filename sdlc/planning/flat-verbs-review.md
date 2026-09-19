# Review of the flat-verb redesign

- Status: proposal. Nothing here is settled until Ian rules.
- Date: 2026-09-19

Ian rewrote the command line in four design notes on 2026-09-19: a simplified core, a critique of that core, a grading design, and a saved-question design. This page reviews them against the twelve demos in `demos/`, the measurements in `design-study.md` section 5, and the code that has landed. It ends with one recommended surface and the list of rulings it needs.

## What the notes propose

- Flat verbs replace the `decide` family. `thinkthen decide QUESTION` answers yes or no. `choose`, `score`, `filter`, `rank`, `match`, `segment`, and `run` stand beside it. Seven more verbs cover reduction, patching, state machines, and three constraint solvers.
- Output is a bare JSON value: `true`, `"bug"`, `1.6`. `--details` prints the full object.
- `decide` is a shell test by default. Exit 0 is yes, 1 is no, 3 is uncertain.
- One option sets the rule. `--threshold 0.9` divides no from yes. `--threshold 0.1:0.9` adds an uncertain band. The default is 0.5.
- The question is a positional argument. Required inputs are never flags.
- Backend settings leave the everyday command line and live in a configuration file. A named profile selects one.
- A saved file of questions builds one JSON object per record. Grading is that same operation plus a local report.

## What I accept

1. **Flat verbs.** `decide where` and `decide how` never read as English. `filter` and `score` do. This replaces the grammar in ADR 0003.
2. **Bare JSON by default, `--details` for the object.** The full result object stays the single internal model. The bare value is one view of it.
3. **`decide` is always a shell test.** `grep`, `test`, and `cmp` behave the same way. `--status` goes away and `--quiet` arrives.
4. **`--threshold` with both forms and a default of 0.5.** The present `--min-prob 0.9` equals `--threshold 0.1:0.9`. The new form also allows an uneven band. The `unassessed` status disappears, because a rule always exists.
5. **The question is positional, options sit on either side, and `--` ends option parsing.**
6. **A configuration file and `--profile NAME`.** The URL, adapter, model, and key variable flags stay as advanced options. ADR 0004 holds apart from the rename of `--backend` to `--profile`.
7. **JSON for every file the tool reads or writes.** Configuration, saved questions, and results share one format. `jq` edits all three. No second parser enters the build.
8. **JSON Pointer for `--field`.** The rest of the record stays on the machine.
9. **Grading is the saved-question verb plus a local report.** No second way of asking questions exists.
10. **Recording stays opt-in and out of the short help.** It already works that way, and the gates depend on it.

## Where I differ

1. **The question comes first on every verb.** The notes give `choose` a hidden default question and make its first argument a label. I recommend `thinkthen choose QUESTION OPTION OPTION…` and `thinkthen score QUESTION LEVEL LEVEL…`. One grammar covers every verb: verb, question, then what the verb needs. The user always sees the question the model receives. A saved file already requires the question, and the command line should match it.
2. **Boundaries are inclusive.** The notes use "strictly above", so `--threshold 1` can never say yes, and the two ends of a band behave differently. I recommend "meets the mark": yes when p ≥ HIGH, no when p ≤ LOW. The band is then symmetric, and it equals the rule the landed code already applies.
3. **`rank` orders by the probability of yes.** The notes rank by a rubric score. The measurements say rubric scoring is the weakest thing a decider does, and no demo wanted a rubric. Demo 06 ranks with a plain question. `rank QUESTION` needs no rubric file.
4. **`score` ships with a warning, and nothing else depends on it.** It costs little once `choose` exists. Its help text carries the measured weakness.
5. **`filter` takes a single cut only.** A band forces a third pile and a flag to steer it. A user who wants three piles runs `decide --jsonl --details` and splits with `jq`. `--invert` waits, because every demo rewrote the question instead.
6. **An exact tie in `choose` returns `null`.** Alphabetical order is no evidence. The three notes disagree on this point, and the critique note has it right.
7. **Exit 3 means unresolved on every single-input verb.** `choose --threshold 0.9` exits 3 when no option clears the mark. One note uses 2 for uncertain in an example. Code 2 stays usage.
8. **`--raw` on `choose`.** A shell `case` on `"bug"` with its quotation marks is a trap. `--raw` prints the bare label, as `jq -r` does.
9. **The saved-question verb is `annotate`, and it adds to the record.** Ian dislikes `run`, and the word also misleads, because this tool never runs anything. Its default output is the input record plus one new top-level field per question. A name collision is an error. This settles the three-way disagreement in the notes over appending, `jq` merging, and copying. It also fixes a strong demo finding from demo 07: chained judgments nested as `.input.input.sku`. With records passing through `filter` unchanged and `annotate` adding flat fields, a chain stays flat.
10. **The saved file holds questions only.** The run note adds `copy`, `const`, `object`, `array`, and `default`. That is a small programming language, and the grading note warns against exactly that. Once the record passes through, `copy` has no job. `jq` does the rest. I keep `on`, because a table asks different questions of different columns and no question should see a column it does not need.
11. **Seven verbs leave this tool.** `assign`, `cover`, and `select` never call a model, so they belong in a solver. `state` keeps a durable store. `patch` writes files. `reduce` runs an adaptive search. Each breaks the rule that this tool judges and never acts. `match` is `decide --jsonl` over records that hold two things, and demo 11 declined it. All seven stay on a roadmap page and out of version one.
12. **No `config set`, no `spec` group, no `eval compare`.** An editor changes the configuration. `annotate --dry-run` checks a saved file. Comparison across runs belongs to a history tool outside this one.
13. **`--field` implies JSON.** The notes carry `--input-format json` beside `--jsonl` and `--lines`. With `--field` and no record flag, the input is one JSON value. The extra option goes away.
14. **`--plan` becomes `--dry-run`.** It is the name people already know.
15. **The local adapter moves up the plan.** No live judgment has come back, because the vendor account has no credits. The `chat-logprobs` adapter against a local server gives a live path that costs nothing, and it proves the backend seam with a second implementation.

## The recommended surface

| Command | Reads | Prints | Exit |
| --- | --- | --- | --- |
| `decide QUESTION` | one document | `true`, `false`, or `null` | 0, 1, 3 |
| `choose QUESTION OPTION…` | one document | a label or `null` | 0, 3 |
| `score QUESTION LEVEL…` | one document | a number | 0 |
| `filter QUESTION` | records | the records that clear the mark | 0 |
| `rank QUESTION` | records | the records, most likely yes first | 0 |
| `segment QUESTION` | lines or paragraphs | segments with line numbers | 0 |
| `annotate FILE` | records | each record plus the named answers | 0 |
| `report` | detailed results | counts and a threshold sweep, with no model call | 0 |
| `config path`, `show`, `check` | the configuration file | local facts | 0 |

Codes 2, 4, 5, and 70 keep their meanings on every command. `--jsonl` or `--lines` turns `decide`, `choose`, and `score` into a map over records. The exit code then reports the run, and no record's answer sets it.

Everyday options: `--threshold`, `--details`, `--quiet`, `--raw`, `--lines`, `--jsonl`, `--field`, `--context`, `--input`, `--top`, `--dry-run`, `--profile`.

Advanced options, shown only in the long help: `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, `--max-retries`, `--record`, `--replay`, `--config`.

Held for a later release: `--from FILE`, `--invert`, `--output FILE` with publish on success, CSV reading, and `--context` if no demo asks for it.

## What changes in the landed code

One rework ticket covers all of it. `decide if CONDITION` becomes `decide QUESTION`. The default output becomes the bare value, and `--details` prints the object with a new top-level `value`. `--min-prob` becomes `--threshold`, the default mark becomes 0.5, and `unassessed` goes. `--status` goes and `--quiet` arrives. `--plan` becomes `--dry-run`. `--backend` becomes `--profile`. The adapter, the HTTP edge, the failure table, recording, and replay stay as they are. The twelve demos are rewritten first, as ADR 0005 requires.

## What Ian rules on

Every item under "Where I differ" departs from his notes, so each is his to accept or overturn. Items 1, 9, 10, and 11 change the shape of the tool the most.
