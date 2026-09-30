# 0308: A requests-per-minute pacer

Status: open. Lane claude-4. Branch `ticket/0308-request-pacer`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Design: ADR 0111 section 8. Issue: `sdlc/issues/2026-09-26-no-requests-per-minute-pacer.md`.

## Outcome

A run at the defaults stays under the vendor's published limit. One process spaces its HTTP attempts to each posting address. The default is 1,000 requests a minute for an `https://` address. The vendor published 1,200 a minute for the hosted service on 2026-09-19 (`specification/records.md`, "The default backend's published limits"). The default leaves a 200-a-minute margin, which covers up to 32 requests that a gate wait can release together.

## Setting

- Name: `THINKTHEN_REQUESTS_PER_MINUTE`. A whole number from 1 to 60,000.
- Surfaces: the command reads it at its edge. `EngineBuilder::from_env()` reads it, and every port and SQL extension builds from that, so no port adds code. No flag, setter, or question-file key: AGENTS.md adds options only for demos.
- Default: 1,000 for an `https://` address. A plain `http://` address is a loopback one by the address rule, so it is not the vendor and is unpaced unless the variable is set.
- Scope: one process, per posting address, shared by every engine and thread in the process. Separate processes on one account still add their rates together. A limit across processes needs shared state on disk, and this ticket defers it.

## Evidence

- Starts from: main `4de0f8116`. The issue measured 1,519 requests a minute at the default `--jobs 4` live, and 1,285 on loopback at `--jobs 3` with 100 ms replies. The vendor limit is the 2026-09-19 row in `specification/records.md`. ADR 0111 section 8 places the pacer in `engine/backoff.rs`.
- Keeps: the throttle (`--jobs`, `throttle`) still caps requests in flight. The per-address 429 gate, the retry count, and the doubling wait are unchanged. Stored and replayed answers never wait.
- Changes: `Gates` in `engine/backoff.rs` gains one next-slot time per address. Every attempt, retries included, takes the next slot before it asks for a throttle place, so a paced wait holds no place. The wait is `Cancel::wait`, so a cancel, a stop token, or a deadline ends it. The command edge and `from_env` parse the variable once. The facade hands the rate to the one `Client`.
- Proof: a unit test takes slots from three threads and shows each start is at least one interval after the one before. A unit test shows a cancel ends a paced wait. A command test against a loopback backend counts request start times at a set rate. The settings page gains a row.
- Defers: a limit shared across processes; sizing the retries for a sustained 429; a token-per-second limit.

## Interactions

- Throttle: the two limits stack. The pacer bounds how often an attempt starts. The throttle bounds how many run at once.
- 429 backoff: a paced attempt may then wait at a closed gate. When the gate opens, up to the throttle width of attempts can start together. That burst is at most 32, inside the margin.
- Deadline: a paced wait counts against the call's deadline. A slot past the deadline ends with the deadline error.
