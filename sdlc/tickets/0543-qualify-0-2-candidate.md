# 0543: Pass one 0.2 candidate on Linux, macOS and Windows

Status: OPEN.

Milestone: 0.2

Depends on: 0542, 0549, 0550, 0551, 0552, 0553, 0554, 0555, 0556, 0558

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

Reviews: revision d0428c12fe3a6e13b0295c92a0d8069c1419522e, accept

Reviews: revision 175753e6db48df1d548e952101689be89e986b21, accept

Reviews: revision b17f9693d10918b04638aaa0b71578f0ca51bbad, accept

Reviews: revision 1d1686487, accept

Reviews: revision fe3906cfd, accept

Reviews: revision d1368f621, accept

Reviews: revision 2d7ca433a, reject

Reviews: revision 412ba61a1, accept

Reviews: revision 7326dcf5b09ac6811c0afbc0062f8639e4c6c077, accept

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 597ef8ae60b452689aa5f3813d6aea3967a5fe7b, accept

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

PM amendment, 2026-10-11. The [adversarial review](../records/2026-10-11-0-2-adversarial-review.md) found defects and gaps after 0542 closed. Stop qualifying rc.11 when its current run exits. Land 0549 first, then 0550 to 0554, and cut the next candidate after they land. Before cutting it, a fresh reviewer searches the code for every item in the Changes lines of 0549 to 0554 and confirms each one is gone or done. The candidate's local campaign uses the 0549 runner. Ian approved 0555 and 0556 the same day, and the same rules cover them: they land before the candidate, and the pre-candidate search covers 0549 to 0556.

PM amendment, 2026-10-11, second. One revision gets tested, accepted and landed.
- Combine 0549, 0555 and the 0550 to 0558 stack onto current `origin/main` as one revision. The campaign running on `ad4b49b4a` contains none of the 0550 to 0558 product code, so its result only checks the runner.
- On that combined revision, run routine `test` and `lint`, then the Polars nextest and clippy lane and the C library checks. Record a review accept only after those pass on the same SHA. A source-only review is evidence, not acceptance.
- Run the full campaign once on that SHA. A candidate run fails on exit 77 and on platform-pending rows. Linux may skip only Objective-C and Swift, and the M5 run covers them.
- Install the Apple packages that the Swift and Objective-C `check.sh` scripts need on the M5, then rerun both. If they need full Xcode or an Apple account, tell the PM, who will raise it with Ian. 0556 does not close on exit 77.
- Land from the tested SHA.

## Progress

- 2026-10-11 started
- 2026-10-11 landed b118ee482; next: Finish code tickets and final review before running the candidate suites and nonpublishing platform workflows.
- 2026-10-11 landed fa49e95b9; next: The checkpoint fixture now proves documentation replay precedes staging and replay failure prevents staging; finish code tickets before candidate qualification.
- 2026-10-11 next: Qualify the reviewed candidate with local full cases, package and stress checks, then run the nonpublishing release rehearsal and Windows workflow.
- 2026-10-11 next: Correct the package graph rule for the shared CSV reader, then qualify a new candidate; rc.7 stopped before full cases or hosted dispatch.
- 2026-10-11 landed 09b4ab1ee7e61fcdf90498385cb71984db527c3e; next: The package graph retains shared CSV parsing and rejects CLI dependencies; qualify the next candidate locally before hosted rehearsal.
- 2026-10-11 next: Make the generated request schema part of the Rust source package, then qualify a new candidate; rc.8 passed library-only tests but failed default package verification.
- 2026-10-11 landed 483faa4e2; next: Qualify a new candidate with the reviewed portable MCP schema; rc.8 retained the failing source-package evidence.
- 2026-10-11 landed 4be5631e0; next: Qualify the corrected full checkpoint on a new candidate, reusing unchanged native artifacts and rebuilding packages that record the candidate commit.
- 2026-10-11 next: Repair the common smoke isolation and native package helper plus the affected language smoke paths, then qualify their reviewed batch on a new candidate.
- 2026-10-11 landed 933be5367; next: Finish and review the common native installer and native smoke repairs, then qualify their batch on a new candidate.
- 2026-10-11 landed 2c139537a; next: Finish the reviewed native smoke batch and qualify one candidate through full installed parity, stress and hosted platform checks.
- 2026-10-11 landed 1e6e4b562; next: Qualify the reviewed candidate through full installed parity, stress and hosted platform checks.
- 2026-10-11 next: Stopped rc.11 and retained its logs; land 0549 first, then 0550 through 0554 before a fresh search review and new candidate.
- 2026-10-11 next: Retain stopped rc.11 logs, land 0549 through 0556, search every reviewed change, then qualify a fresh candidate with background runs.
- 2026-10-11 next: Read amendments 22b005e1b and consolidate current main, 0549, 0555 and the product stack into one revision; runner-only ad4b49b4a exited 1 on a missing retained TypeScript archive, three developers now own consolidation and runner fixes, decode and direct ABI proof, and Engine regressions, with compiler checks released and no source-only acceptance permitted.
- 2026-10-11 next: Combined ba41be5a7 passes unchanged installed C 42/42, Polars 46/46 and owning Rust checks; whole routine test and lint now run on that frozen revision before any acceptance, while exhaustive ratchets confirm JVM overages and a new C fixture overage, with a separate safe fixture repair lane and Mac package prerequisites unresolved.
