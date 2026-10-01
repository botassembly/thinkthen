# 0145: Build the speed test

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0145-speed-test`, in lane `thinkthen-lane-2`. The ticket is `sdlc/tickets/0145-speed-test.md`. Two fresh read-only design reviews accepted it after the coordinator's rulings of 2026-09-26. The change raises the ceiling, so a second agent reviews the code and names what it checked. Ian can overturn every decision the ticket lists.

No live call ran. The builder never ran `sdlc/scripts/live`, the live mode or `job.sh`. It ran `plan` with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset.

## Result

- `probes/speed/functions.jsonl` holds the ten function rows of the ticket's table. Eight carry a `list` entry: `decide`, `filter` and `rank` for B4, `choose` for B8, `tag` and `score` for B9, `annotate` for B10, and `recognize` for R7. No entry has a number yet.
- `probes/speed/workloads/` holds 12 made-up titles, the same 12 as JSON Lines records with an album field, 12 made-up sentences, three names, the two-group `annotate` question set, and one one-group set for each field. None of the 36 records is a content cut. The README lists each value mod 4,096.
- `probes/speed/measure.py` is the runner, with modes `gate`, `plan` and `live`. Each command runs in a private home that `tempfile.mkdtemp` made, with `--no-cache`, and `status --json` in that home gives its counts. `remove()` deletes only a folder this run made. Every child gets an explicit environment, as the `children` lint check requires.
- `probes/speed/job.sh` is the live job: `#!/bin/sh`, then `exec python3 probes/speed/measure.py live "$@"`.
- `crates/thinkthen/tests/speed.rs` runs the gate against the in-process conformance backend's generic arm. It checks the counts `status` reports against the socket count, then exit 0 and empty standard error, then three planted tables with their exact sentences.
- `probes/README.md` gains a row for `speed/`.

On today's build, the gate reads 12 requests for each of `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `recognize`, 24 for `annotate`, 1 for `find` and 3 for `relate`: 112 in all, each equal to the ticket's `items`. So stop rule 3 did not fire. The socket also read 112, so stop rule 4 did not fire.

## Plan

`env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL python3 probes/speed/measure.py plan BENCH`, with Beatles Bench at `7d246844`, exited 0. It printed 191 commands and an estimate of 1,192,341 input tokens:

| Measurement | Commands |
| --- | --- |
| target, 306 titles, default | 3 |
| throttle, `--jobs 16` | 1 |
| function rows | 10 |
| recognize by step, with a relation rule | 1 |
| annotate by field | 2 |
| bench jobs | 19 |
| bench questions, grouped by function and question | 140 |
| evidence size: 19, 7,965, 23,904, 39,942 and 55,975 bytes | 15 |

No `--batch 1`, `--boundary run` or `--context` arm appears, because today's help names none of them. As a free check, the builder ran every planned command once against the loopback generic arm with the made-up key. All exited 0.

## Plants

Plants (a) to (e) each edited `probes/speed/measure.py`, ran `cargo test --test speed` under the heavy lock, and restored and touched the file. The script and logs sit in the session scratchpad under `t0145/`. A grep of the diff for plant text found none.

| Plant | Result |
| --- | --- |
| None | Green |
| (a) Count printed lines in place of `status` | Red at the socket check: 103 reported, 112 read |
| (b) One shared home for every row | Red at the socket check: 677 reported, 112 read |
| (c) Rule 3 compares with `items` | Red: the unlisted `filter` table exits 0, not 1 |
| (d) Rule 2 fails only above `items` | Red: the stale `find` table exits 0, not 1 |
| (e) Read the ticket's first line in place of its `Status:` line | Red: the landed `decide` table exits 0, not 1 |
| (f) `remove()` given the lane path | Refused at exit 2: `speed: refused to delete a folder this run did not make`. The lane is intact |

Plant (a) reads 103, not the ticket's "short by 12". The printed lines differ from the requests in three rows: `filter` prints 0 of 12, `annotate` 12 of 24, and `relate` 18 edges for 3 requests. The test checks the socket count before the verdict, so (a) and (b) turn red for their stated reason.

## Deviations

- **The stale-binary check.** The ticket compares the binary with the commit time of `HEAD`. A commit that changes no source leaves the binary current, and cargo does not relink it. So the check dates the binary by the last commit to `crates/thinkthen/src`, `crates/thinkthen/Cargo.toml`, `Cargo.toml` and `Cargo.lock`.
- **The clean-tree checks read tracked files only.** Each child's environment holds only `PATH`, so git reads no global ignore file, and `__pycache__` folders showed as untracked.
- **The README gives the value mod 4,096 for each record.** It does not list the first 16 hex digits, which would have passed its 70-line budget.
- **`audit/rows-context.jsonl` is skipped.** It matches `*-context.jsonl`, but it holds saved rows, not cases. The runner skips any file whose first row has no `args`.

## Budgets

Nonblank lines.

| File | Budget | Measured |
| --- | --- | --- |
| `tests/speed.rs` | at most 90 | 95, within the tenth stop rule 1 allows |
| `probes/speed/measure.py` | at most 260 | 260 |
| `probes/speed/job.sh` | at most 6 | 4 |
| `probes/speed/functions.jsonl` | 10 rows | 10 |
| `probes/speed/workloads/` | at most 50 | 42 |
| `probes/speed/README.md` | at most 70 | 31 |
| `probes/README.md` | one row | one row |

`sdlc/ratchet.json` moves from 69,097 to 69,192, up 95, for `tests/speed.rs`. The test reuses the shared `test_deadline` child and run helpers and the conformance backend, so it copies no harness code. The five extra lines come from returning `io::Result` from its two helpers, which clippy's `expect_used` rule requires.

## Ladder

Run on the merge of `origin/main` at `b78b6515`, except `lint`, which reran on the later merge at `678d2d71`.

| Rung | Result |
| --- | --- |
| `lint` | exit 0 on the merge at `678d2d71`; ratchet 69,192 of 69,192. On the first merge, main's `AGENTS.md` stood at 5,641 characters of its 5,000 cap. The rest of lint, run from a copy with that one check removed, passed. Main then trimmed `AGENTS.md` |
| `test` | exit 0; 956 passed, 0 failed across 37 test binaries |
| `spec` | exit 0; demos 21 green, 0 red |

`install` and `surfaces` did not run. No dependency, public type, method or message changed.

## Left for later

- "S1 live run 1" runs on a main commit after S1 lands and before B4 builds, and only when Ian authorizes it by name. The coordinator brings him the `plan` output and the charge of 2,000,000 tokens.
- The ticket's deferred gaps stand as written.
