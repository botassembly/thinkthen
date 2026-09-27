---
flow: build
priority: 171
opens: crates/thinkthen/src/core/result.rs crates/thinkthen/src/core/result/batch_warning.rs crates/thinkthen/src/core/mod.rs crates/thinkthen/src/result_json.rs crates/thinkthen/src/cli/profile.rs crates/thinkthen/src/cli/asked.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/batched.rs crates/thinkthen/src/cli/audit.rs crates/thinkthen/src/cli/audit/write.rs crates/thinkthen/src/cli/diff.rs crates/thinkthen/tests/backend/batching.rs crates/thinkthen/tests/backend/batching crates/thinkthen/tests/audit_write.rs crates/thinkthen/tests/diff.rs crates/thinkthen/tests/fixtures/measure specification/result.md specification/question-file.md specification/audit.md specification/diff.md specification/records.md specification/settings.md spec/audit.md sdlc/issues sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0171: A tuned threshold warns at another batch setting

Status: ready for review. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and later the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user tuned `refund.json` with `audit --write` when every record went in its own request. After ticket 0146, `decide @refund.json --lines` fills each request to the limit, and the tuned cut no longer fits as well. Today nothing says so. After this ticket the run prints once on standard error:

```text
thinkthen: warning: threshold tuned at batch 1 is running at batch max
```

Each `--details` row carries `meta.batch_warning`. `audit` and `diff` warn when the results they read ran at different batch settings. `audit --write` records the setting the bar was tuned at, so the file says what it was tuned for.

This is batching row B16 of `sdlc/issues/2026-09-26-batching-design.md`, proof test 13. ADR 0048 item 8 gives the rule. ADR 0053 item 3 makes a file with a `threshold` and no `batch` count as tuned at batch 1. It is Batch D item 5 of `sdlc/planning/work-plan-2026-09-27.md`.

Ian's rulings set the frame. Ian can overturn each.

- Speed wins over accuracy. The default stays `max`, and a warning, not a refusal, carries the cost.
- The batch part of calibration identity stays out of the question digest (batching design, ruling 5 of "Open items").
- Simple beats clever.

## Prior experiment evidence

- Local experiment 284, file 14: a threshold tuned at one record a request runs batched with no warning, under an unchanged `meta.question_sha256`. It asks for one warning naming both settings, and for the digest question to be settled in writing.
- `sdlc/records/2026-09-26-batching-and-recognize-evidence.md` section 14: over the 306 titles, one full request in the tool's form gave 32 to 34 false yeses where one title a request gave 16 or 17. ADR 0055's quoted form narrowed that gap (283 to 287 right against 286), but a tuned cut can still move.
- `cli/profile.rs::Mismatch` already prints a profile mismatch once and copies it into every row. This ticket reuses that path.
- This ticket makes no paid call. The warning is a comparison of settings. B6 measures how far cuts move.

## Which findings hold

Checked on `origin/main` `40783433`.

| Finding | Holds? | Evidence | This ticket |
| --- | --- | --- | --- |
| File 14, a tuned cut runs batched with no warning | Yes, from ticket 0146 on | `core/digest.rs:55-68` hashes the question, threshold and profile, and nothing else. Main does not batch. Ticket 0146 makes `max` the default and excludes the warning. `result.md` line 109 marks `batch_warning` as not built | The warning, the row member, and the `audit` and `diff` lines |
| File 14's digest question | Settled already | ADR 0048 item 8 and Ian's ruling keep the batch setting out of the digest. Putting it in would re-key every cache entry | No digest change. The ticket cites the ruling |

## What happens today

Read from `origin/main` `40783433` and ticket 0146's branch.

- `cli/asking.rs:150` builds `Mismatch::new(settled.profile(), profile)`. `Mismatch::print_once` prints the profile line at the first successful logical result. `result_json.rs::Run` carries the warning into `meta.profile_warning`.
- Ticket 0146 adds `QuestionFile::parse_top`, which takes `batch` off the top of a `decide` file and returns its raw value. `cli/asked.rs` feeds it to the file tier of the batch setting. The resolved setting is `max` or a whole number.
- `cli/audit/write.rs::bars` writes the threshold and then the one model every results line names. When the lines name more than one model, it keeps the model and says why.
- `cli/diff.rs::warnings` prints at most two warning lines.
- After ticket 0170, a batched row carries `meta.batch.setting`. A row asked alone carries no `meta.batch`.

## Retained behavior

- `meta.question_sha256` and every cache key stay the same.
- A run whose setting matches its file's prints exactly what it prints today.
- A file with neither `threshold` nor `batch` warns nobody.
- `choose`, `tag`, `score`, `annotate`, `find`, `recognize` and `relate` behave as today. They do not batch yet.
- The profile warning keeps its line and its member.

## The change

### The run warns

On a record run of `decide`, `filter` or `rank`:

- **Tuned for.** The file's `batch` when it has one. Otherwise 1 when the file has a `threshold`. Otherwise none, and no warning.
- **Running.** The resolved batch setting from ticket 0146's four tiers.
- **Mismatch.** Tuned for and running differ. `10` and `max` differ. `max` and `max` do not.

On a mismatch the run prints `thinkthen: warning: threshold tuned at batch T is running at batch R` once, where the profile line prints and by the same rule: at the first successful logical result, and also when `filter` keeps nothing. Each `--details` row carries `meta.batch_warning` as `{"tuned_for":T,"running":R}`, a number or the string `"max"` on each side. It sits after `batch` in `meta`, in the order the `result.md` table gives. A run on one document never warns, because it does not batch.

`Mismatch` gains the batch pair beside the profile pair. Each line prints once. `core/result/batch_warning.rs` holds the pure `BatchWarning` value, next to `profile_warning.rs`.

### `audit` warns and writes

**The setting of a results set.** It is the set of `meta.batch.setting` values the lines carry. When no line carries one, it is `{1}`. A line with no `meta.batch` was asked alone, so it adds nothing to a set that already holds a batched setting.

- `audit` prints one line on standard error when the graded lines hold more than one setting: `thinkthen: audit: warning: the results ran at more than one batch setting (1 and max); a bar tuned over both may fit neither`. The values are listed in ascending order, numbers before `max`, joined by `and` or commas.
- `audit --write` writes `batch` beside the threshold, as it writes `model`, when it writes a threshold into a single `decide` file and the lines hold one setting. The value is the number or `"max"`. When they hold more than one, it keeps the file's `batch` and adds `thinkthen: audit: kept the batch setting for the question; the results ran at more than one batch setting` to its report.
- It never writes `batch` into a `choose`, `tag` or `score` file, a question set, a `recognize` file or a `relate` file. Their parsers refuse the key until B8, B9 and B10 add it.

### `diff` warns

`diff` reads the setting of each side by the same rule. When the union of the two sides holds more than one setting, it prints `thinkthen: diff: warning: the runs used different batch settings (1 and max); batching moves answers, so some changes may come from it`. `diff` then prints at most three warning lines.

### Pages

- `result.md`: the `batch_warning` row and the `meta` paragraph lose their item 8 markers.
- `question-file.md`, "Precedence": the item 8 half of the marker becomes the rule. It says that `audit --write` writes `batch` and that the file's `batch` then runs by the file tier.
- `audit.md`: the warning line, the `batch` write and the kept line.
- `diff.md`, "Warnings": the third line, and "at most three".
- `records.md`: one sentence that a tuned file warns at another batch setting.

After this ticket, `grep -rn "Not built yet, by ADR 0048 item 8" specification` returns nothing.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **A warning, not a refusal.** Ian ruled speed ahead of accuracy. A refusal would stop every tuned file at the new default.
2. **No digest change.** ADR 0048 item 8 and Ian's ruling decide it. File 14's question is answered by citation, not by a new rule.
3. **A row asked alone counts as batch 1 only when no line was batched.** Its request is byte for byte the batch-1 request, by ADR 0055 item 3. In a batched run a single-record batch is common, at a content cut or a pause, and must not make every batched run look mixed.
4. **`audit --write` writes `batch` silently when it writes a threshold, as it writes `model`.** The spec page and demo 41 then change nothing they pin.
5. **A written `batch` also sets the run's batch through the file tier.** ADR 0048 item 4 gives the file one `batch` key for both. A file tuned one record a request then runs one record a request unless a typed value or `THINKTHEN_BATCH` overrides it, and those print the warning.
6. **A typed `--threshold` beside the file still warns.** The profile warning behaves the same way. One rule for both is simpler.
7. **Only `decide`, `filter` and `rank` warn.** They alone batch. B8, B9 and B10 extend the rule with their verbs.

## Edge cases

| Input | Expected |
| --- | --- |
| File with `threshold`, no `batch`, default setting | Warns `tuned at batch 1 is running at batch max`. Rows carry `{"tuned_for":1,"running":"max"}` |
| The same file at `--batch 1` | No warning. Today's bytes |
| File `"batch": 10`, `--batch 1`, from a recording | Warns once `tuned at batch 10 is running at batch 1` (design test 13) |
| File `"batch": 10`, `THINKTHEN_BATCH=max` | Warns `tuned at batch 10 is running at batch max` |
| File `"batch": "max"`, default | No warning |
| File `"batch": 1`, nothing else | No warning. One record a request |
| File with neither key | No warning |
| File with `threshold`, one document | No warning |
| `filter` over a tuned file that keeps nothing | The line prints once |
| A failure at the first record | No warning, as the profile line |
| `choose @FILE` with a `threshold` | No warning |
| `audit` over lines all without `meta.batch` | No warning. `--write` writes `"batch": 1` |
| `audit` over lines at `max` and some single-record batches | No warning. `--write` writes `"batch": "max"` |
| `audit` over lines at `max` and lines at 10 | The audit warning. `--write` keeps `batch` and reports why |
| `audit --write` into a `choose` file | No `batch` written. The file still parses |
| `audit --write` into a question set | No `batch` written |
| `diff` of a `--batch 1` run and a default run | The diff warning. Standard output and exit code unchanged |
| `diff` of two default runs | No batch warning |

## Tests and proof

Every command test drives the compiled binary. Run tests use the loopback or a recording. `audit` and `diff` tests read fixture lines and send nothing.

| Test | What it proves | Deliberate break that turns it red |
| --- | --- | --- |
| `a_tuned_file_warns_at_another_batch`, design test 13, in `tests/backend/batching` | The run rows of the edge table. Each pins the whole standard error and, under `--details`, `meta.batch_warning` on every row. The `--batch 1` row pins today's standard error | (a) Treat a file with a threshold and no `batch` as untuned: the default row is silent. (b) Compare against the file tier after the typed value replaced it: the `--batch 1` row is silent. (c) Print per row: the line repeats. (d) Warn on one document |
| `audit_reads_and_writes_the_batch_setting`, in `tests/audit_write.rs` | The `audit` rows, over fixture lines with and without `meta.batch`. It pins standard error and the written file's bytes | (a) Count a line without `meta.batch` as 1 in a batched set: the `max` row warns. (b) Write `batch` into a `choose` file: the file stops parsing. (c) Write the first setting when there are two |
| `diff_warns_at_different_batch_settings`, in `tests/diff.rs` | The two `diff` rows. Standard output and exit code match today's | (a) Compare only the first line of each side. (b) Print the warning on standard output |

The four questions:

- **What behavior does each protect?** That a tuned cut never runs in another batch regime without a word, and that a tuned file records its regime.
- **What credible regression fails each?** The breaks above: a lost default of 1, a wrong tier, a mixed-run rule that fires on every batched run, or a write that breaks a file.
- **Why does no existing test catch them?** Nothing compares batch settings today. The profile warning tests cover the profile pair only.
- **Does any need a test-only hook?** No.

## Budgets and ratchet estimate

Nonblank lines, measured with `grep -c .`, net against main after ticket 0170 lands.

- `core/result/batch_warning.rs`: at most 30, new. `core/result.rs`, `core/mod.rs` and `result_json.rs`: at most 12 net together.
- `cli/profile.rs`: at most 25 net.
- `cli/asked.rs`, `cli/asking.rs` and `cli/asking/batched.rs`: at most 20 net together.
- `cli/audit.rs` and `cli/audit/write.rs`: at most 45 net together.
- `cli/diff.rs`: at most 20 net.
- Product code: at most 152 net.
- Tests: at most 170 net, and at most 8 small fixture files.
- `sdlc/ratchet.json` moves to the measured total, at most 322 above main after 0170 lands. The commit says what grew.
- Pages: at most 20 net lines.
- No dependency. No paid call.

## Stop rules

1. Stop before crossing any budget by more than a tenth, or before adding a dependency.
2. Stop if `meta.question_sha256` or any cache key changes.
3. Stop if a run whose setting matches its file's changes one byte.
4. Stop if demo 41 or `spec/audit.md` needs a changed expectation. Decision 4 says they should not.
5. Stop if any break stays green.
6. Stop if ticket 0170 or ticket 0152 Part B has not landed on main.
7. Stop if the build needs a live call. None is authorized here.

## Build order

1. Ticket 0170 (B5) lands first. It adds `meta.batch.setting`, which `audit` and `diff` read.
2. Ticket 0152 Part B lands first. It opens `core/measure`, `cli/audit/table.rs`, `audit.md` and `diff.md`.
3. Ticket 0172 (B7) builds after this one. Both edit `meta`, `result_json.rs` and `cli/asking/batched.rs`.

## Scope and exclusions

Excluded: the warning for `choose`, `tag`, `score` and `annotate` (B8, B9, B10), `meta.batch_warning` in the libraries (B12a), measuring how far a cut moves (B6), and `site/`.

## Routing

Builder: Claude (Opus subagent) in lane 1. Reviewer: a fresh read-only Claude session for the design and the code. The change raises the ceiling and adds a row member, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 1; total 6. Final level: 2. The risk is a mixed-run rule that fires on every batched run, which the audit test's first break guards.

## Deferred gaps

1. A results file that joins a `--batch 1` run and a batched run reads as the batched setting, because rows asked alone carry no `meta.batch`. `audit` then writes the batched setting. Carrying the setting on every row would change `--batch 1` bytes, which ADR 0048 item 9 forbids.
2. The verbs that batch later extend the warning in B8, B9 and B10.
3. Library runs carry `meta.batch_warning` from B12a, as the design's edge table asks.

## What Ian can overturn

- Decision 1: a warning, not a refusal.
- Decision 3: a row asked alone counts only when no line was batched.
- Decision 4: a silent `batch` write beside the threshold.
- Decision 5: the written `batch` also sets the run by the file tier. This follows ADR 0048 item 4.
- Decision 6: a typed threshold still warns.

## Closes

No issue file holds local experiment 284 file 14 alone. The build records file 14 as settled in its landing commit. The batching design issue stays open.

## Evidence

- Starts from: Local experiment 284 file 14, checked on `origin/main` `40783433`: `core/digest.rs:55-68`, `cli/profile.rs`, `cli/asking.rs:150`, `cli/audit/write.rs::bars` and `model`, `cli/diff.rs::warnings`, and `result.md` line 109. ADR 0048 items 4, 8 and 9, ADR 0053 item 3 and ADR 0055 item 3. Ticket 0146's `parse_top` and tiers, and ticket 0170's `meta.batch`. Evidence section 14 of the batching record.
- Keeps: The question digest and every cache key. Every run whose setting matches its file's. Files with neither key. The profile warning. Verbs that do not batch.
- Changes: A `decide`, `filter` or `rank` record run over a file tuned at another batch setting prints one warning and carries `meta.batch_warning`. `audit` and `diff` warn over mixed settings. `audit --write` writes `batch` into a single `decide` file when the lines hold one setting.
- Proof: Three outside-in tests, each with deliberate breaks, including design test 13. The `install`, `lint`, `test`, `spec` and `surfaces` rungs.
- Defers: Mixed results files that include rows asked alone, the later verbs, and library warnings.
