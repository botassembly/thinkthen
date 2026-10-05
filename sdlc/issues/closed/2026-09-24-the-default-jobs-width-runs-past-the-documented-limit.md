# The default --jobs width runs past the documented request limit on short records

Status: Closed by Quick Fix `qf-jobs-width` on 2026-09-24 (`sdlc/records/qf-jobs-width.md`). The default stays at 4. Owner: Claude, as this repository's queue owner, through a Quick Fix. Split out by ticket 0100 from `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md`, which closed when its `cost.jq` defect was fixed.

## The question

Should the default `--jobs` width drop from 4 to 3?

`--jobs` defaults to 4. Experiment 206 measured a width of 4 at 1,267, 1,319, and 1,272 requests a minute on three separate 200-record checks from one machine. A width of 3 measured between 972 and 1,017 on five checks. The vendor documents 1,200 requests a minute. A user who never sets `--jobs` runs about 6 to 10 percent over that limit on short records. Longer records answer more slowly and stay under it. The service refused nothing in those runs, or at 4,300 a minute in the first spike.

The accuracy issue recommended no pacer, a test that replays a 429 in the middle of a wide run, and keeping 4 until the vendor refuses a request.

## What 0077 decides

Ticket 0077 landed on main at `15fb302a`. It keeps the fallback width at 4 and changes no command behavior. It moves the fallback into one constant, `Width::FALLBACK`, so a change to the default is now one line. Its code review (`sdlc/records/0077-code-review.md`, section 5) found no new defect. It named this a pre-existing default question plus a documentation gap, owned by Claude through a Quick Fix.

## The missing numbers

The accuracy issue asked the `--jobs` reference page to give the measured numbers. On main at `ac69ec5f`, `specification/records.md` line 129 and `site/src/pages/reference.astro` line 145 still carry none of them.

## Fix

A Quick Fix ticket takes both parts:

1. Decide the default under the configuration rule: keep 4 with the reason, or drop to 3. Change `Width::FALLBACK` if it drops, and record the reason in the ticket.
2. Put the measured numbers on the `--jobs` reference pages, `specification/records.md` and `site/src/pages/reference.astro`, naming experiment 206 and the accuracy issue as the measuring record.

Ian can overturn the default width the Quick Fix picks.

## Decision, 2026-09-24: the default stays at 4

Claude decided this under the configuration rule. Ian can overturn it. A change is one line, `Width::FALLBACK` in `crates/thinkthen/src/engine/mod.rs`, plus the `[default: 4]` help pin and the pages that name 4.

Why 4 holds:

- Three accepted ADRs set 4. ADR 0009 read the vendor's own example code, which uses 4 to 12 workers and says the public endpoint limits concurrency above about eight. ADR 0010 made `--jobs` an advanced option with a default of 4. ADR 0017 section 5 lists 4 as the width on every surface. Dropping to 3 would need an amendment to all three against evidence that has not changed their premise.
- The overage is small and narrow. Experiment 206 measured about 6 to 10 percent over the documented 1,200 a minute, from one machine, on short lines only. The accuracy record reports that longer records stay under the limit at 4, but it gives no rate for them.
- No refusal has been seen. The service refused nothing at 4, and nothing at about 4,300 a minute in the first spike (`2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md`).
- An enforced limit slows a run and does not break it. `specification/backends.md` retries a 429 with a doubling wait and honors `Retry-After-Ms` and `Retry-After`. `crates/thinkthen/tests/backend/exchange.rs` covers those waits.
- On short lines, a width of 3 measured 972 to 1,017 a minute against 1,267 to 1,319 at 4, about a fifth to a quarter slower. The width bounds the requests in flight, so long records would likely lose a like share. No record measured that. The cost falls on every run to fix an overage only short records reach.
- The author of experiment 206 recommended keeping 4 until the vendor refuses a request.

What would change it: a 429 or a published enforcement change from the vendor at width 4. Then the default drops to 3.

The two `--jobs` pages now give the measured numbers and name the measuring records: `specification/records.md` and `site/src/pages/reference.astro`. Both tell a user who must stay inside the documented limit on short records to set `--jobs 3`.

Ticket [0400 slice C](../../tickets/0400-provider-setups-and-concurrency.md) raises the current default to 8 using experiment 0017. The throttle-4 rates above remain historical measurements; optional configured rates and 429 retries retain their existing behavior.
