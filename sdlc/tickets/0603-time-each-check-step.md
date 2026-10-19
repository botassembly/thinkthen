# 0603 — time-each-check-step

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision c6529de, accept

## Outcome

`sdlc/scripts/test` and `sdlc/scripts/lint` print each step's elapsed seconds, so their cost can be split without guessing.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- `lint` took a median 247 seconds per revision (0.3 to 595). Only clippy (0.5 s) and rustdoc (4.2 s) are timed; about 240 seconds goes to roughly 30 Python, Node and shell self-tests with no per-step timing.
- `test` took about 407 seconds: nextest 171 s, the library-surface test 196 s, conformance unit tests 23 s and the build-admission test 12 s. Only some of these print their own time.
- Cost before: 247 s of lint with 4.7 s attributed. Expected after: every step attributed, at no added cost.

## Scope

- Changes: `sdlc/scripts/test`, `sdlc/scripts/lint`.
- Keeps: every check's coverage.

## Proof

One run of each prints step times that add up to within 5% of the total. After landing, measure again and record the result here.

## Amendment, 2026-10-19

Moved to 0.2.1. Every landing runs the full M5 gate, about 15 minutes per pass and 32 runs in the last day, and nobody can say which steps cost the time. Step timings let the next ticket cut or move the slowest self-tests to the checkpoint suite. It is small, touches only the two entrypoints, and lands on its own.
