# 0543: Pass one 0.2 candidate on Linux, macOS and Windows

Status: OPEN.

Milestone: 0.2

Depends on: 0542

## Outcome

One reviewed commit, tagged `rc/0.2.0-rc.N`, passes the local release suite and the hosted rehearsal on Linux, macOS and Windows. It publishes nothing. This closes the build side of 0.2. Release QA and publication stay with 0425.

## Evidence

- Starts from: [the 2026-10-10 ruling](../decisions/2026-10-10-drive-0-2-to-done.md), which lets the coordinator cut candidates and run nonpublishing workflows, and moves every Windows check to the candidate. This ticket takes the candidate stage from 0425.
- Keeps: no registry publication, no approval of the `release` environment, and no paid call. The exact-commit rehearsal guard from 0398.
- Changes:
  - Local release suite on the candidate commit: `sdlc/scripts/test-full-cases --run`, `sdlc/scripts/package` and `sdlc/scripts/test-stress --run`, including the bounded MCP timing from 0455.
  - The final candidate includes 0425's landed release-preparation workflow changes.
  - Tag the commit and dispatch `.github/workflows/release.yml` in rehearse mode and `.github/workflows/windows.yml`. Cover Linux x86 and ARM, `macos-15`, `macos-15-intel` and Windows.
  - The Windows run carries the platform proofs for 0383, 0384, 0385 and 0455, and the 0474 and 0480 cache writer, reader, conversion and replay checks.
  - The macOS run executes the installed Foundation consumer from 0518 if 0530 has not already run it on the M5.
  - Windows failures become new bug tickets. Other failures get a fix here, or a bug ticket when they need their own design. Batch fixes and cut the next candidate.
- Proof: the run identifiers and the commit, named once in this ticket's landing record.
- Defers: release QA, publication and after-publication checks stay with 0425, 0393 and 0398.
