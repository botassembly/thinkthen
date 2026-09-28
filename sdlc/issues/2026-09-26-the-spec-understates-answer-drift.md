# The spec understates how far a repeated request moves

Status: closed on 2026-09-27. Ticket 0163 records the three-set offline measurement and corrects the drift pages and ADR. Reviewed source: `0292cba2`; proof: `sdlc/records/0163-code-review.md`.

## What happens

`specification/recording.md` line 78 says borderline answers moved "by up to 0.08", and that experiment 259 "found gaps up to 0.09 in 178 of 681 repeated digests". The ADR 0010 amendment uses the same numbers.

The report counted repeated digests in the Beatles Bench recordings, all from `jev-1.13.0`. 4,075 of 5,290 repeated digests differed. 263 differed by more than 0.1, and the largest gap was 0.45. One digest replays as `john` 0.21, 0.25 and 0.66, so it crosses the default cut of 0.5. Live, six back-to-back sends of that request gave 0.22 to 0.49.

A team that sizes a tolerance band or a "rerun until stable" budget from 0.08 will see decisions flip.

## Checked on main

Partly verified. `recording.md:78` gives 0.08 and 0.09 as quoted. The bench counts come from the report and were not recounted here.

## What would fix it

Measure drift over the current recording corpus. The recordings are on disk, so the measurement needs no paid run. Replace the numbers with the measurement and name the record that holds it. Say plainly that a borderline answer can cross the cut from one call to the next. Point readers to the not-sure band in `threshold.md`.

## Done when

`recording.md` and the ADR 0010 amendment cite a measurement over the corpus, and the page says a borderline answer can flip.
