# 0623: Set a time limit on every workflow job

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- Each long job's limit is set from its measured passing time in the capped container (for example, Intel build measured 3h28m22s), at about twice that time, with the figure in a comment.
- A job that times out keeps its logs as a build output, so the cause can be read.

### Proof

- A hosted rehearsal run on the candidate SHA passes with every job under its limit.
- One dispatched run with a step forced to sleep past its job's limit stops at that limit and keeps its logs.

## Evidence

Reviewed source 14e97f9e7, now on main, bounds all 23 executable gate and release jobs. Short resolution and packaging jobs got small caps, but long jobs kept the six-hour ceiling, so a hung long job still costs up to six hours. Hosted runs can also return 404 for live child logs, so a long job shows only broad step states.
