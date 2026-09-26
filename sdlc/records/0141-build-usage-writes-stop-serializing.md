# 0141: Build usage writes stop serializing requests

Status: built 2026-09-26 in lane `worktrees/thinkthen-lane-2`. The code review's fixes are in. Owner: Claude.

Branch `ticket/0141-usage-writes-stop-serializing`. The ticket is `sdlc/tickets/0141-usage-writes-stop-serializing.md`. A fresh read-only design review accepted it on 2026-09-26. The change raises the ceiling and moves a Settled guarantee, so a second agent reviews the code and names what it checked. Ian can overturn every decision the ticket lists.

## Result

- `Counters` in `engine/usage.rs` holds one `Shared` queue: the process totals, the pending deltas by month, and the writer's state, under one mutex and a condition variable. `add` counts in memory, queues its delta, and starts the writer thread on first use when a usage path exists. It never touches a file.
- `write_behind` takes everything pending, writes each month with the unchanged `update()`, and sleeps until more arrives. The first failure stops all later writes.
- `finish()` replaces `warning()`. It waits for the writer, then says whether a write failed. `cli/mod.rs` calls it where it called `warning()`. `Drop` writes what is pending and joins the writer.
- The process totals moved from four atomics into the queue. This keeps the file under its 500-line cap. The ticket expected the atomics to stay. A request now takes one short uncontended mutex for its count, with no file access under it.
- The two `Stage` enums became one. The `FAILURE` test hook follows the writer thread through `carried()`, under `cfg(test)` alone. The `dead_code` allowance on `snapshot` went, because `facade.rs` already calls it outside tests.
- `specification/recording.md` says counting never holds back a request, and gives the new crash sentence. ADR 0049 records the change and amends ADR 0034's crash sentence by name.
- The comments at `engine/http.rs:128` and on `accounting_that_outlasts_the_budget_sends_nothing` no longer name the usage lock. `http.rs` stays at 500 nonblank lines.
- `sdlc/issues/2026-09-26-site-states-the-retired-usage-crash-guarantee.md` asks the website agent to fix `site/src/pages/backends.astro:48`.
- The harness `spawn` split into `start`, which returns the running child, and `spawn`.
- No setting was added or changed.

## Code review fixes

The code review at `83272ab8` found the design sound and asked for these fixes.

- The writer starts through `thread::Builder::spawn`. If the thread cannot start, persistence fails with the one warning, and nothing is queued.
- `snapshot` reads through a poisoned lock, so it never reports zeros.
- A write that unwinds counts as a failed write. `catch_unwind` around the write does what the asked-for drop guard would do: it clears `writing` and sets `failed`, so `finish()` never waits forever. It fits in two lines, and the file stays under its 500-line cap. The crate already uses `catch_unwind` in `public/options.rs` and `engine/mod.rs`.
- ADR 0034's crash sentence ends "(Amended by ADR 0049.)".

## Plants

`requests_go_out_while_another_process_holds_the_usage_lock` in `tests/backend/default_cache/usage.rs`. Each plant edited `engine/usage.rs`, ran the test, restored the file from a saved copy, and touched it. The plants ran on the final code after the last merge. The script and logs sit in the session scratchpad under `t0141/`, outside the repository. A grep of the diff for plant text found none.

| Plant | Result |
| --- | --- |
| None, the fix as built | Green |
| (a) `add` calls `update()` itself and queues nothing, as before the change | Red: `(count, peak)` was `(0, 0)` while the lock was held, after the 10 s failsafe |
| (b) the month's pending sum keeps only the newest delta | Red: `requests_sent` was 1, not 16 |
| (c) `finish()` does not wait and `Drop` does not join | Red: the command exited while the lock was held |

## Timing

This measurement gates nothing. It used 306 lines at `--jobs 16` against a local Python server that answers each request after 140 ms. Both binaries were debug builds of this branch at `befd5b85`, one with the old `usage.rs` and `cli/mod.rs`. The usage folder sat on the machine's ext4 NVMe disk.

| Build | Round 1 | Round 2 | Round 3 |
| --- | --- | --- | --- |
| Before | 4.79 s | 4.92 s | 6.45 s |
| After | 4.61 s | 4.46 s | 4.48 s |

Every run exited 0 with 306 rows and 306 requests counted. The gain here is smaller than experiment 268's, where each flush took a median of 11.4 ms. This disk flushes faster, and both runs sit above the 2.7 s that 16 jobs at 140 ms allow. The test above is the proof: a held lock no longer stops any request.

## Budgets

Nonblank lines against `origin/main` at `a14d959e`.

| File | Ticket budget | Measured |
| --- | --- | --- |
| `crates/thinkthen/src` production | at most 60 added, 50 net | `usage.rs` 106 added, 56 removed, 448 to 498, +50 net, after the review fixes. It was 103 added and +47 net at review. `cli/mod.rs` and `http.rs` 1 changed each. **Crosses the added budget** |
| New test and harness split | at most 80 added | 65 in `default_cache/usage.rs`, 12 added and 2 removed in the harness: 77 |
| Existing unit tests | at most 15 changed | 11 |
| Pages and ADR | one sentence, one comment, ADR at most 30 lines, the issue | one sentence, two comments, ADR 13 nonblank lines, the issue |

The net budget holds, and the added budget does not. Replacing the atomics and the `persistent` flag rewrote the lines around them. The coordinator accepted the overrun: 103 added and 56 removed, net +47, within the 50 net budget. Most added lines replace removed ones, and merging the two Stage enums removes duplication. Ian can overturn this.

`sdlc/ratchet.json` moves from 67,627 to 67,758, up 131: 50 in `usage.rs`, 75 in the new test and harness, and 6 in the unit tests. The builder looked for duplication in `usage.rs` first and merged the two `Stage` enums.

## Ladder

After the review fixes and the merge of `origin/main` at `b43fcd8a`, each rung ran once. The three plants ran again on this code, and each turned the test red as in the table above.

| Rung | Result |
| --- | --- |
| `install` | exit 0, run before the review fixes |
| `lint` | exit 0; ratchet 67,758 of 67,758 |
| `test` | exit 0; 943 passed, 0 failed across 36 test binaries |
| `spec` | exit 0; demos 21 green, 0 red; settings 0 failures |

`surfaces` did not run. No public library type, method, or message changed.

## Deferred gaps

The ticket's gaps stand: no time bound on `finish()`, no early end to its wait on the first Ctrl-C, one sync per write, the lock inherited across a fork, and the batching speed target. The website sentence waits on the filed issue.
