# 0604 — speed-library-surface-test

Status: OPEN.

Milestone: 0.3

## Outcome

The library-surface conformance test's 19 cases finish in under 30 seconds.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- `conformance/library-surface-test.py` took a median 196 seconds (189 to 308) over 13 runs, the costliest single step in the routine `test`.
- It runs the checker as a subprocess 18 times, each in a freshly copied tree with its own `git init`.
- Cost before: 196 s per routine run. Expected after: under 30 s, by running the checker in-process over one shared prepared tree.

## Scope

- Changes: `conformance/library-surface-test.py`, `conformance/library-surface.py`, `conformance/library-surface-declarations.py`.
- Keeps: every check's coverage.

## Proof

All cases pass, a planted rule break still fails, and the new time is recorded in this ticket. After landing, measure again and record the result here.
