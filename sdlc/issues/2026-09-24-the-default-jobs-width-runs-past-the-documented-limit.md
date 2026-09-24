# The default --jobs width runs past the documented request limit on short records

Status: Open. No owner yet. Split out by ticket 0100 from `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun.md`, which closed when its `cost.jq` defect was fixed.

## The question

Should the default `--jobs` width drop from 4 to 3?

`--jobs` defaults to 4. Experiment 206 measured a width of 4 at 1,267, 1,319, and 1,272 requests a minute on three separate 200-record checks from one machine. A width of 3 measured between 972 and 1,017 on five checks. The vendor documents 1,200 requests a minute. A user who never sets `--jobs` runs about 6 to 10 percent over that limit on short records. Longer records answer more slowly and stay under it. The service refused nothing in those runs, or at 4,300 a minute in the first spike.

The accuracy issue recommended no pacer, a test that replays a 429 in the middle of a wide run, and keeping 4 until the vendor refuses a request.

## What 0077 decides

Ticket 0077 (`sdlc/tickets/0077-share-one-process-width-cap.md` on `ticket/0077-process-width-cap`) keeps the fallback width at 4 on purpose. It says the command still schedules an unbound batch at the existing fallback width 4, so command behavior and help remain unchanged. It does not take this question.

## The missing numbers

The accuracy issue asked the `--jobs` reference page to give the measured numbers. On main at `ac69ec5f`, `specification/records.md` line 129 and `site/src/pages/reference.astro` line 145 still carry none of them.

## Fix

1. Decide the default: keep 4 with the reason, or drop to 3.
2. Put the measured numbers on the `--jobs` reference pages, naming experiment 206 and the accuracy issue as the measuring record.
