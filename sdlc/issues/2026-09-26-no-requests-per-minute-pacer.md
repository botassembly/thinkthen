# No requests-per-minute pacer

Status: open for the pacer. Ticket 0162 fixed the records-page arithmetic on 2026-09-27 and labels the reply time as derived. Register 30 remains to do after 0155; the wording change does not implement a rate limit.

## What happens

The throttle counts only requests in flight (`crates/thinkthen/src/engine/mod.rs`). Nothing limits requests per minute. So the rate depends on reply time, record length and network, and the user has no direct control.

- Live, the default `--jobs 4` sent 1,519 requests a minute, and `--jobs 16` sent 4,320. The vendor documents 1,200.
- On loopback with 100 ms replies, `--jobs 3` sent 1,285 a minute. `specification/records.md` line 131 says a run that must stay under the limit "sets `--jobs 3`".
- When 429s persist, each record makes 3 sends over about 3 seconds, and then the whole run stops at exit 4. Twelve 429s after 20 good records gave "stopped at record 21".
- Several processes on one account add their rates together.

A long job at the defaults runs until the vendor starts enforcing the limit, then dies partway through.

## Prior decisions

`sdlc/issues/closed/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` kept the default width at 4. Ticket 0155 and ADR 0052 add one shared backoff per provider address within a process. Neither adds a pacer, and 0155 helps within one process only.

## Checked on main

The original finding read `records.md:131` as `--jobs 3` advice. Ticket 0162 replaces that claim with the reply-time arithmetic and says no setting caps requests per minute. Historical rates were not rerun.

## What would fix it

1. Reword `records.md:131` so the `--jobs 3` advice names the reply time it assumes. This part is a doc fix for 0.1.
2. Later, add a requests-per-minute pacer beside the concurrency cap, with a default taken from the backend's published limit. Size the retries for a sustained limit, not a short blip. This part adds a setting, so it needs a ticket and a settings row.

## Done when

The page states the rate honestly. A later ticket settles whether a pacer ships.
