# 0543: Pass one 0.2 candidate on Linux, macOS and Windows

Status: OPEN.

Milestone: 0.2

Depends on: 0542

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

Reviews: revision d0428c12fe3a6e13b0295c92a0d8069c1419522e, accept

Reviews: revision 175753e6db48df1d548e952101689be89e986b21, accept

## Outcome

One reviewed commit, tagged `rc/0.2.0-rc.N`, passes the local release suite and the hosted rehearsal on Linux, macOS and Windows. It publishes nothing. This closes the build side of 0.2. Release QA and publication stay with 0425.

## Evidence

- Starts from: [the 2026-10-10 ruling](../decisions/2026-10-10-drive-0-2-to-done.md), which lets the coordinator cut candidates and run nonpublishing workflows, and moves every Windows check to the candidate. This ticket takes the release preparation and candidate stages from 0425.
- Keeps: no registry publication, no approval of the `release` environment, and no paid call. The exact-commit rehearsal guard from 0398.
- Changes: prepare release workflows and qualify one candidate across platforms.
  - Local release suite on the candidate commit: `sdlc/scripts/test-full-cases --run`, `sdlc/scripts/package` and `sdlc/scripts/test-stress --run`, including the bounded MCP timing from 0455 and the 0524 full Go matrix, load and large-input cases.
  - Release preparation, moved from 0425, lands before the final candidate. The checkpoint sweep runs `npm run test-docs` and names the result. The release workflow runs `site/scripts/check-binding-proofs.mjs` in strict mode and fails when `site/examples/bindings-proof.json` does not match the tagged commit. The release workflow builds the R source tarball from the published-crate shape before the GitHub release goes public. The 0425 record notes when the Maven Central token, its signing key and `TAP_DEPLOY_KEY` expire and when each was last rotated. No rehearsal reads a secret; 0398 keeps that rule. `install-check.yml` gains `windows-2025` rows for `install.ps1`, npm, NuGet, Maven Central and every other channel that ships Windows. Report each account prerequisite to Ian: the Maven namespace and token, pub.dev trusted publishing, and any unconfirmed trusted-publisher registration. Claim `.github/workflows/**`, `sdlc/planning/release-process.md`, `sdlc/scripts/surfaces*`, `sdlc/scripts/release-workflow`, `CHANGELOG.md`, `site/scripts/check-binding-proofs.mjs` and `sdlc/records/0543*`.
  - Tag the commit and dispatch `.github/workflows/release.yml` in rehearse mode and `.github/workflows/windows.yml`. Cover Linux x86 and ARM, `macos-15`, `macos-15-intel` and Windows.
  - The Windows run carries the platform proofs for 0383, 0384, 0385 and 0455, and the 0474 and 0480 cache writer, reader, conversion and replay checks.
  - The macOS run executes the installed Foundation consumer from 0518 if 0530 has not already run it on the M5.
  - Windows failures become new bug tickets. Other failures get a fix here, or a bug ticket when they need their own design. Batch fixes and cut the next candidate. 0543 closes when one candidate passes every platform, after the Windows bug tickets land.
- Proof: the run identifiers and the commit, named once in this ticket's landing record.
- Defers: release QA, publication and after-publication checks stay with 0425, 0393 and 0398.

## Progress

- 2026-10-11 started
- 2026-10-11 landed b118ee482; next: Finish code tickets and final review before running the candidate suites and nonpublishing platform workflows.
- 2026-10-11 landed fa49e95b9; next: The checkpoint fixture now proves documentation replay precedes staging and replay failure prevents staging; finish code tickets before candidate qualification.
- 2026-10-11 next: Qualify the reviewed candidate with local full cases, package and stress checks, then run the nonpublishing release rehearsal and Windows workflow.
- 2026-10-11 next: Correct the package graph rule for the shared CSV reader, then qualify a new candidate; rc.7 stopped before full cases or hosted dispatch.
