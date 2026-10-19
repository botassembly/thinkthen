# 0608 — campaign-reuses-passes-and-stops-on-failure

Status: WITHDRAWN.

Opened as: 2026-10-19, 0.2.1 retired the candidate campaign: candidates are main revisions checked by the 0691 checkpoint run, and hosted runs happen only for releases.

Milestone: 0.3

## Outcome

A release candidate run cancels its hosted runs when a local gate fails. Each gate declares its inputs, and a pass is reused while those inputs are unchanged.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- Hosted runs used 903 runner minutes over 20 revisions. 280 of them (31%) went to the 7 revisions that failed locally; those hosted runs started 6 to 23 minutes into a local run that later failed.
- Every revision reran all 49 local gates and all three hosted workflows. b25c45401 changed only `release.yml`, release-script self-tests and records, and cost 14.8 local minutes and 33.6 hosted. 97f46705d changed one test file and cost 16.2 and 28.1.
- Ian ruled during 0.2 that the campaign runners stay out of Git while he evaluates the build system, and that every gate runs on the frozen commit before a tag. This ticket waits on that evaluation and must keep that rule: reuse applies only to passes on the same frozen inputs.
- Cost before: 280 hosted minutes on failed revisions, and a full local and hosted pass on records-only changes. Expected after: hosted spend stops within a minute of a local failure, and a records-only commit reuses every pass.

## Scope

- Changes: the candidate gate list, `sdlc/scripts/hosted-candidate.py`, `sdlc/scripts/hosted-workflow.py`, and the campaign runner once Ian's evaluation places it.
- Keeps: every gate on the tagged commit's inputs; the tag only after gates, campaign and rehearsal pass on that commit.

## Proof

A planted local failure cancels the hosted runs within one minute. A records-only commit reuses every pass, and a C-only change reruns only the gates that read C. After landing, measure again and record the result here.
