# 0622: Split the Intel macOS smoke into parallel jobs and tighten job time limits

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0613

Reviews: revision cc87266, accept

Reviews: revision cc87266, accept

## Outcome

- The smoke for each target is split by family into separate jobs that run in parallel. Each job downloads the shared build outputs it needs.
- Each long job's time limit is set at about twice its measured passing time on its hosted runner (for example, the Intel build measured 3h28m22s), with the figure in a comment.
- A job that times out keeps its logs as a build output, so the cause can be read.

### Proof

- `sdlc/scripts/lint` passes on the M5 through pm job.
- The workflow tickets 0613, 0620 and 0622 land together. One dispatched nonpublishing rehearsal at the next checkpoint (0691) proves them: every Intel smoke job passes under its limit, and the run's total time is lower than the last single-job Intel smoke.

## Evidence

The `smoke` matrix in `.github/workflows/release.yml` runs every family on one runner per target: the Go and C++, Swift and Zig, PHP and Dart, Ada, Objective-C and COBOL, and managed gates, then the smoke bundle, installer and crate smoke. The Intel macOS runner is the slowest, and this one job ran into the six-hour hosted cap.

Reviewed source 14e97f9e7, now on main, bounds all 23 executable gate and release jobs. Short jobs got small caps, but long jobs kept the six-hour ceiling, so a hung long job still costs up to six hours. Hosted runs can also return 404 for live child logs, so a long job shows only broad step states.

## Progress

- 2026-10-19 started
- 2026-10-19 next: Fresh review accepts cc872667a after all14 existing workflow-runtime-installer groups and full lint passed in M5 job1792427137514-26784; real-checkout fixtures now validate all bundles before execution and selected SQLite/PostgreSQL families require the shipped CLI, while provisional hosted timings and transfer proof remain0691 and this change batches with0620 after0619.
