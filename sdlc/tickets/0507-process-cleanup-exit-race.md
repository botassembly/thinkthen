# 0507 — process-cleanup-exit-race

Status: OPEN.

Milestone: 0.2

Reviews: revision 4ffc11e39, accept

## Outcome

The existing process-cleanup test treats an owned child that exits during its Linux process-state read as gone, while still detecting live descendants and retained handles.

## Evidence

- Starts from: The 0488 lint gate at source 9333013f99 failed in `sdlc/scripts/release-process-cleanup-test.py:48`: an exiting descendant made `/proc/PID/stat` raise ProcessLookupError. The preceding run passed the same workflow test. The helper catches FileNotFoundError for this read but omits the other observed disappearance error.
- Keeps: Existing owned-child termination, zombie handling, live-process refusal and file-handle cleanup assertions. Never stop another team's process.
- Changes: Handle the observed disappearance at the existing liveness boundary. Claim `sdlc/scripts/release-process-cleanup-test.py` and `sdlc/records/0507*`. Keep the repair separate from MCP protocol changes.
- Proof: A small regression exercises both disappearance exceptions at the existing boundary and still recognizes a live owned child. Run the existing process-cleanup and workflow tests. No hosted workflow or release action is needed.
- Defers: New process supervision, proof tooling and general release-script redesign.

## Progress

- 2026-10-08 started
