# 0142: Build the pool keeps up to `--jobs` connections

Status: built; ladder run once after the merge of `origin/main`; ready for code review. Owner: Claude.

Branch `ticket/0142-pool-keeps-jobs-connections`, in lane `worktrees/thinkthen-lane-2`. The ticket is `sdlc/tickets/0142-pool-keeps-jobs-connections.md`. A fresh read-only design review accepted it on 2026-09-26. The change raises the ceiling, so a second agent reviews the code and names what it checked. Ian can overturn every decision the ticket lists.

## Result

- `Client::new` in `engine/http.rs` sets ureq's total and per-host idle limits to `Width::MOST`, 32. The `Client` doc comment says the pool keeps an idle connection for each request the widest throttle allows. The file holds 500 nonblank lines, its ceiling.
- `Width` in `engine/mod.rs` names its widest value once as `MOST`, and `Width::new` checks its range against it.
- `a_run_opens_one_connection_for_each_job_and_reuses_it` in `tests/backend/parallel.rs` replaces `one_process_reuses_the_connections_it_opens`. At `--jobs` 1, 4, 16 and 32 it sends two held rounds of `jobs` records, with a 200 ms pause between them, and asserts exactly `jobs` connections accepted. A `piped` helper starts the binary with standard input open. The pipe-close test shares it.
- No specification page changes. `records.md` already says `--jobs N` opens up to N connections.

## Plants

Each plant edited `engine/http.rs`, built, ran the new test three times, and restored the file. The restored file was touched. The script and logs sit in the session scratchpad under `t0142/`, outside the repository. A grep of the diff for plant text found none. The final runs used the final test file.

| Plant | Result, three runs each |
| --- | --- |
| None, the fix as built | Green 3 of 3 |
| (a) both pool lines removed, ureq defaults | Red 3 of 3: 4 jobs opened 5 connections |
| (b) only the total limit removed, left at 10 | Red 3 of 3: 16 jobs opened 22 connections |
| (c) both limits at 16 | Red 3 of 3: 32 jobs opened 48 connections |

Plant (a) turns red at the 4-job row, earlier than the ticket's 16-job row predicted. ureq's per-host limit of 3 trims the fourth idle connection once all four sit idle in the pause.

## Budgets

Nonblank lines against `origin/main`.

| File | Ticket budget | Measured |
| --- | --- | --- |
| `engine/http.rs` | at most 1 net | 499 to 500, +1 |
| `engine/mod.rs` | at most 3 net | 389 to 391, +2 |
| `tests/backend/parallel.rs` | at most 60 added, 45 net | 53 added, 417 to 429, +12 |

`sdlc/ratchet.json` moves from 66,679 to 66,694, up 15. The test file grows by the two-round table. The shared `piped` helper removes the pipe-close test's own spawn, and that removal paid for most of the growth.

## Ladder

Run once each after the merge of `origin/main` at `0ec897e1`.

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint` | exit 0; ratchet 66,694 of 66,694 |
| `test` | exit 0; 932 passed, 0 failed across 35 test binaries |
| `spec` | exit 0; demos 21 green, 0 red |

`surfaces` did not run. No public library type, method or message changed.

## Deferred gaps

The ticket's three stand. No gate test counts secure handshakes, the idle age stays at 15 s, and ureq's defaults are read from its 3.4.2 source.
