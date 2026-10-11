# 0308: A requests-per-minute pacer

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-4. Branch `ticket/0308-request-pacer`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Design: ADR 0111 section 8. Issue: `sdlc/issues/closed/2026-09-26-no-requests-per-minute-pacer.md`.

## Outcome

A run at the defaults stays under the vendor's published limit. One process spaces its HTTP attempts to each posting address. The default is 1,000 requests a minute for an `https://` address. The vendor published 1,200 a minute for the hosted service on 2026-09-19 (`specification/records.md`, "The default backend's published limits"). The default leaves a 200-a-minute margin for replies that arrive unevenly.

## Setting

- Name: `THINKTHEN_REQUESTS_PER_MINUTE`. A whole number from 1 to 60,000.
- Surfaces: the command reads it at its edge. `EngineBuilder::from_env()` reads it, and every port and SQL extension builds from that, so no port adds code. No flag, setter, or question-file key: AGENTS.md adds options only for demos.
- Default: 1,000 for an `https://` address. A plain `http://` address is a loopback one by the address rule, so it is not the vendor and is unpaced unless the variable is set.
- Scope: one process, per posting address, shared by every engine and thread in the process. Separate processes on one account still add their rates together. Each PostgreSQL backend process paces alone, so N connections can send up to N times the rate. A limit across processes needs shared state on disk, and this ticket defers it.

## Evidence

- Starts from: main `4de0f8116`. The issue measured 1,519 requests a minute at the default `--jobs 4` live, and 1,285 on loopback at `--jobs 3` with 100 ms replies. The vendor limit is the 2026-09-19 row in `specification/records.md`. ADR 0111 section 8 places the pacer in `engine/backoff.rs`.
- Keeps: the throttle (`--jobs`, `throttle`) still caps requests in flight. The per-address 429 gate, the retry count, and the doubling wait are unchanged. Stored and replayed answers never wait.
- Changes: `Gates` in `engine/backoff.rs` gains one next-slot time per address. Every attempt, retries included, takes the next slot after any 429 gate wait and before it asks for a throttle place, so a paced wait holds no place and a gate wait holds no stale slot. The wait is `Cancel::wait`, so a cancel, a stop token, or a deadline ends it. The command edge and `from_env` parse the variable once. The facade hands the rate to the one `Client`.
- Proof: a unit test takes slots from three threads and shows each start is at least one interval after the one before. A unit test shows a cancel ends a paced wait. A command test against a loopback backend counts request start times at a set rate. The settings page gains a row.
- Defers: a limit shared across processes; sizing the retries for a sustained 429; a token-per-second limit; the `site/` reference sentence that says no setting limits requests a minute, which marketing owns.

## Interactions

- Throttle: the two limits stack. The pacer bounds how often an attempt starts. The throttle bounds how many run at once.
- 429 backoff: an attempt waits at a closed gate first and takes its slot after the gate opens, so attempts leave a reopened gate one interval apart.
- Deadline: a slot past the call's deadline fails at once with the deadline error and reserves nothing, so short-deadline calls do not push later slots out.

## Build result

- `engine/backoff.rs`: `Gates` holds one next-slot time per address. `Gates::pace` reserves a slot under the lock and waits with `Cancel::wait`. `per_minute` parses the variable and `interval` picks the set rate or the `https://` default.
- `engine/http.rs`: `Client::paced` stores the spacing. `acquire_open` paces after the gate wait and before the throttle place, for first sends and retries alike. A gate that closes again during the wait costs one more slot.
- `engine/facade.rs`: the facade setting `per_minute` reaches the one `Client`. `cli/edge.rs` and `from_env` read the variable. `cli/asking.rs` and `cli/check.rs` pass it on. Five test builders of the facade settings gain `per_minute: None`.
- Tests: `engine::backoff` checks spacing from three threads, a cancel during a paced wait, a quiet second address, a deadline refusal that reserves nothing, and the parser's edge table. `tests/backend/backoff.rs` counts loopback start times under the command at 600 a minute and pins the malformed-variable sentence with no send. `tests/public_env/batch.rs` does the same through `EngineBuilder::from_env`. Each end-to-end test failed with the pacer or its wiring removed.
- Pages: a settings row, the `records.md` rate paragraph, and a changelog line.
- Growth: 240 nonblank Rust lines, 105,012 to 105,252; about 165 are tests. `cli/edge.rs` stays at 498 after a stale doc comment shrank.
- Checks: `cargo nextest run -p thinkthen` and `sdlc/scripts/test` passed after merging main at 0307 (1,267 tests). One earlier rung run hit a race in an unrelated process-cap test, filed as `sdlc/issues/2026-09-30-process-cap-test-races-two-records.md`; clippy; `policy.py`; `sdlc/scripts/settings` and its self-test.

## Code review

ACCEPT with four non-blocking findings, all fixed before landing: fail a slot past the deadline without reserving it; reserve after the gate wait; loosen the timing bounds by one interval; name PostgreSQL's per-backend pacing.

## What the build taught us

- `policy.py` requires the facade to call `Client::new` by name. A tidy `with_roots(.., None)` call failed it, so the match stays.
- Every port starts from `EngineBuilder::from_env()`. A variable read there reaches all surfaces with no port code. That is the thin path for an engine-wide setting.
- A pacer that reserves a slot under a lock and then waits outside it needs no condition variable. The cancel wait already exists as `Cancel::wait`.
- A lower-bound-only timing assertion keeps a rate test deterministic on a loaded machine. Each start is at least its slot, so the sorted starts cannot come early.
