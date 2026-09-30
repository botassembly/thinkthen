---
flow: build
priority: 142
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/mod.rs crates/thinkthen/tests/backend/parallel.rs sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0142: The pool keeps up to `--jobs` connections

Status: landed 2026-09-26. A fresh code review accepted it with one minor finding, recorded as a deferred gap. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A run at `--jobs 16` opens about 16 connections and reuses them for every later request. Today it opens a new connection for 40 to 65 of every 100 requests, and each secure handshake costs 60 to 90 ms.

This is ticket B2 in `sdlc/issues/closed/2026-09-26-batching-design.md`. Its row reads: "The pool keeps up to `--jobs` connections. Per `2026-09-26-connection-pool-reopens-connections-above-three-jobs.md`. Proof: new connections at `--jobs 16` stay near the job count. Depends on nothing. Needs no ADR." Ian's ruling 9 in that design orders B2 after B0, C1 and B1. The issue says the design belongs to this repository.

## What happens today

`Client::new` in `crates/thinkthen/src/engine/http.rs:81` builds the ureq 3.4.2 agent. It sets the timeout, the status handling, the redirects and the proxy. It does not set the idle pool size. ureq keeps at most 10 idle connections in all, and at most 3 for one host, by default (`ureq-3.4.2/src/config.rs:443` and `:452`). When a finished request returns its connection and the pool already holds 3 for that host, `Pool::purge` drops the oldest (`ureq-3.4.2/src/pool.rs:234`). The next request finds no idle connection and opens a new one.

`specification/records.md`, section `jobs`, says one pool serves every worker, "so a run over many records pays for one handshake rather than one for each record", and that "`--jobs N` opens up to N connections". The code keeps that promise at 3 jobs or fewer and breaks it above.

`crates/thinkthen/tests/backend/parallel.rs:302`, `one_process_reuses_the_connections_it_opens`, runs 6 records at `--jobs 1` and `--jobs 4` and asserts between 1 and the job count connections. At 4 jobs, the fourth idle connection is dropped only if all four sit idle at once, and 6 records rarely make that happen. So the test passes against today's code.

## Design

`Client::new` sets both idle limits to the widest throttle, 32:

    .max_idle_connections(pool)
    .max_idle_connections_per_host(pool)

`pool` is `Width::MOST.get()`. `Width` in `engine/mod.rs:235` gains `pub(crate) const MOST: Self = Self(32)`, and `Width::new` checks its range against `MOST` in place of the literal 32. The idle age keeps ureq's 15 s default.

A cap of 32 keeps up to `--jobs` connections at every `--jobs`. ureq takes an idle connection before it opens one, and `send` returns its connection to the pool before the permit drops (`http.rs:133`). So the connections open at once never outnumber the attempts in flight, which the throttle bounds. The pool never holds more than the run opened.

The `Client` doc comment says the pool keeps an idle connection for each request the throttle allows. `http.rs` holds 499 nonblank lines of its 500, so the rewrite of that comment pays for the two new builder lines.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The cap is the widest throttle, not the run's own width.** The pool cannot hold more connections than were in flight at once, so a cap of 32 behaves as a cap of `--jobs`. Passing the selected width into `Client::new` would change its signature and every caller, most of them tests, for no change in behavior.
2. **Both limits are set.** The per-host limit alone leaves ureq's total limit of 10, which reopens connections above 10 jobs. Every client posts to one address, so the two limits are equal.
3. **The idle age stays at ureq's 15 s default.** A run that pauses longer than 15 s between requests opens new connections. The issue does not ask for more, and a longer age keeps sockets open that a backend may already have closed.
4. **No specification page changes.** `records.md` already states the behavior this ticket delivers. The code comes into line with it.
5. **One test pins the contract at 1, 4, 16 and 32 jobs.** The new table replaces `one_process_reuses_the_connections_it_opens`, whose bound was loose and whose runs were too small to fail today's code. One contract lives in one test.

## Edge cases

| Run | Today | After |
| --- | --- | --- |
| `--jobs 1` | 1 connection | 1 connection. Kept |
| `--jobs 3` or fewer | Up to that many connections | Kept |
| `--jobs` omitted, throttle 4 | 4, and a new one whenever all 4 sit idle at once | Up to 4 connections. Changed |
| `--jobs 16` | 121 to 198 new secure connections over 306 requests, live | Up to 16 connections. Changed |
| `--jobs 32` | Up to 231 new secure connections over 306 requests, live | Up to 32 connections. Changed |
| A reply with an error status | The connection closes unread, and a retry opens a new one | Kept. `send` returns before it reads that body |
| A pause over 15 s between requests | The idle connection is dropped, and the next request opens one | Kept. Decision 3 |
| `https://` address | Same pool code as `http://` | Same fix. The loopback test proves the plain form. Deferred gap 1 |
| `annotate` and `relate` | Post through the same `Client` | Same fix |
| Two library engines in one process | Each holds its own pool | Kept. Each pool stays under the process throttle |

## Proof

The test runs the compiled command against the loopback listener `Listener::answering`, which keeps each connection open until the client closes it and counts each accepted connection (`conformance/backend/src/listener.rs:292`).

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `a_run_opens_one_connection_for_each_job_and_reuses_it` in `tests/backend/parallel.rs`, replacing `one_process_reuses_the_connections_it_opens` | For each of `--jobs` 1, 4, 16 and 32, `decide --jsonl --field /body --no-cache` runs over two rounds of `jobs` records each. The listener holds the first round with the harness's `Gathering` until all `jobs` requests are in flight, so the run opens `jobs` connections. The test writes the first round to standard input and waits for `jobs` answers on a `Canned::notifying` channel, with a 10 s failsafe. It sleeps 200 ms, so every connection sits idle in the pool. It then writes the second round, which a second `Gathering` holds the same way, and closes standard input. It asserts exit 0, `2 × jobs` rows, `2 × jobs` requests read, a peak of `jobs` in flight, and exactly `jobs` connections accepted | (a) Remove both pool lines, restoring ureq's defaults: the 16 row opens at least 29 connections. (b) Remove only `max_idle_connections`, leaving the total at 10: the 16 row opens at least 22. (c) Cap the pool at 16 in place of `Width::MOST`: the 32 row opens at least 48 |

The pause makes the plants deterministic. With both rounds on standard input at once, a worker may take back its own connection before the pool trims it, and a plant could pass by luck. Under the fix the pause changes nothing. A worker that has not yet returned its connection still holds its permit, so the second round cannot outnumber the connections the first round opened.

The four questions:

- **What behavior does it protect?** One process reuses its connections up to `--jobs`, as `records.md` promises. At `--jobs 16` the new connections equal the job count.
- **What credible regression fails it?** A pool left at ureq's default, a pool limited on one axis, or a pool capped below the widest throttle. A ureq upgrade that changes a default or its trimming order is caught too.
- **Why does no existing test catch it?** The one connection test runs 6 records at 4 jobs or fewer, with a loose bound, and it passes against today's code. No test runs above 4 jobs and counts connections.
- **Does it need a test-only hook?** No. It drives the compiled binary and counts at the real socket.

No unit test is added.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src/engine/http.rs`: at most 1 net added, so the file stays at or under 500. Doc lines included.
- `crates/thinkthen/src/engine/mod.rs`: at most 3 net added.
- `crates/thinkthen/tests/backend/parallel.rs`: at most 60 added and at most 45 net of the replaced test. The file stays under 500.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. The commit says what grew. The builder looks for duplication to delete first in `parallel.rs`, whose piped-stdin spawn in `a_reader_that_closes_the_pipe_stops_the_reading_and_the_scheduling` the new test may share.
- No dependency. No public library type, method or message changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green in any of three runs.
3. Stop if the fixed code opens more than `jobs` connections in any row, in any of three runs. That would mean a connection outlives its permit, and the design's premise is wrong.
4. Stop if the command does not send the first round before standard input closes. The test's premise needs a streaming reader, and the failsafe turns that into a failure, not a hang.
5. Stop if the change needs a file that 0141 or 0143 owns. `engine/http.rs` overlaps with 0141, the usage writes, which may touch `post_observed`'s accounting. This ticket touches only `Client::new` and the `Client` doc comment there. Landing 0142 leaves `http.rs` at 500 nonblank lines, its ceiling, so 0141 must trim to fit. `engine/mod.rs` overlaps with 0143 if relate's throttle touches `Width` or `client_width`. This ticket touches only `Width`. The coordinator orders each pair.
6. Stop if the build needs a live call. None is authorized here.

## Scope and exclusions

Excluded: the usage-write lock (B1). Relate's throttle (J1, ticket 0143). Batching. The idle age. Any live measurement. `site/`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling by a few lines, so the code review names what it checked.

## Complexity

Contract 1; state and timing 1; reach 0; proof 1; cost of error 0; total 3. Final level: 1. The risk is a flaky connection count, which the gathered rounds and the pause remove.

## Deferred gaps

- The command-line parser in `cli/args.rs:164` keeps its own range of 1 to 32 for `--jobs`. `Width::MOST` sizes only the pool and `Width::new`. Deriving the parser's range from it needs a const accessor and a file other tickets own.

1. No gate test counts secure handshakes. The loopback listener speaks plain HTTP, and ureq pools both schemes through one code path. The S1 speed test's live part, or an authorized run of experiment 268's harness, confirms the count at `--jobs 16` against the service.
2. The 15 s idle age. A slow backend with long gaps between replies still reopens connections. No evidence says it matters.
3. ureq 3.4.2's defaults are read from its source. A ureq upgrade that renames the two setters fails the build, and one that changes trimming fails the test.

## What Ian can overturn

- Decision 1: the cap is the widest throttle, not the run's own width.
- Decision 3: the idle age stays at 15 s.
- Decision 5: the new table replaces the old connection test.

## Closes

- `sdlc/issues/closed/2026-09-26-connection-pool-reopens-connections-above-three-jobs.md`. The lander writes its closing status line and moves it to `closed/` in the landing commit.

## Evidence

- Starts from: The issue above, filed 2026-09-26 from workspace experiment 268, which counted 121 to 198 new secure connections of 306 requests at `--jobs 16`, up to 231 at `--jobs 32`, and 7 to 13 at `--jobs 4`, with 60 to 90 ms a handshake at the median. The B2 row of `sdlc/issues/closed/2026-09-26-batching-design.md`. Experiment 218's descriptor counts in `records.md`. ureq 3.4.2's `config.rs` and `pool.rs`. `http.rs` and `parallel.rs` at `origin/main` `d410ef4a`.
- Keeps: The throttle and its range, 1 to 32 with a default of 4. Output order and bytes at every `--jobs`. The timeout, retries, redirects and proxy handling. The 15 s idle age. Every existing test except the one connection test this ticket replaces.
- Changes: `Client::new` sets ureq's total and per-host idle limits to the widest throttle. `Width` names its widest value as `MOST` for the pool and `Width::new`. The connection test counts exact connections at 1, 4, 16 and 32 jobs over two rounds.
- Proof: The outside-in table test under "Proof", counting accepted connections at a loopback listener at `--jobs 16` and `--jobs 32`. Plants (a), (b) and (c) each turn a row red.
- Defers: The parser's own `--jobs` range. A gate count of secure handshakes, left to S1's live part. The idle age. Tracking ureq's defaults across upgrades.
