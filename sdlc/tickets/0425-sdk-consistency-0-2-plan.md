# 0425: Pass 0.2 release QA and publish on Ian's go

Status: OPEN.

Milestone: 0.2

Depends on: 0543

Depends on: 0461

Depends on: 0468

Depends on: 0487

Depends on: 0510

Owner: builder.

Reviews: revision 1f8bc0179286c6b3ec6a87b11618e4fca5057be9, accept

Reviews: revision 37e6828a03d0cdfe1e168b4b7afbc0bffc1dbd7c, accept

Reviews: revision 045dfb5b1f0f99b6477aa5446345e9ffb81ef6ef, reject

Reviews: revision 5d9860455, accept

## Outcome

The commit that passed the 0543 candidate also passes release QA by a fresh agent with only the public docs. On Ian's explicit go, that same commit is released as 0.2.0. Every published package then installs and runs from its public channel on a clean machine. This ticket owns release preparation, release QA, publication and the after-publication checks. 0543 owns the candidate.

## Evidence

- Starts from: the [0.1 release record](../records/0128-release-0-1.md), the six-candidate retrospective in the [0425 record](../records/0425-sdk-consistency-0-2-plan.md), and three mailroom rulings in `repos/agents/inbox/thinkthen/`: the 2026-10-06 completion criteria (`2026-10-06-pm-what-0-2-done-means-beyond-the-tickets-and-two-cleanups.md`), the 2026-10-07 two-stage release (`2026-10-07-thinkthen-m5-use-and-the-two-stage-github-release.md`) and Ian's 2026-10-08 release hold. It also absorbs the release issues this rewrite closes: release and install for 0.1, run facts, the RubyGems placeholder, public install checks, R before release, rehearsal publish steps, two release secrets, two approvals, patch-release hand passes, and doc tests gating releases.
- Keeps: every release safety gate already built.
  - Both GitHub approvals stay as safety gates: the `release` environment and the `publish` step. 0393 keeps the npm staged approval.
  - 0398 keeps the exact-commit rehearsal guard and the after-publication install-check workflow.
  - The M5 runs bounded Mac checks only and never stands in for hosted macOS. Yellow stays excluded. No paid diagnostic runs.
  - A candidate or archive check never stands in for a published-package check.
- Changes: five stages.
  - Release preparation, in any free lane. The checkpoint sweep runs `npm run test-docs` and names the result. The release workflow runs `site/scripts/check-binding-proofs.mjs` in strict mode and fails when `site/examples/bindings-proof.json` does not match the tagged commit. The release workflow builds the R source tarball from the published-crate shape before the GitHub release goes public. The 0425 record notes when the Maven Central token, its signing key and `TAP_DEPLOY_KEY` expire and when each was last rotated. No rehearsal reads a secret; 0398 keeps that rule. `install-check.yml` gains `windows-2025` rows for `install.ps1`, npm, NuGet, Maven Central and every other channel that ships Windows. Report each account prerequisite to Ian: the Maven namespace and token, pub.dev trusted publishing, and any unconfirmed trusted-publisher registration. Claim `.github/workflows/**`, `sdlc/planning/release-process.md`, `sdlc/scripts/surfaces*`, `sdlc/scripts/release-workflow`, `CHANGELOG.md`, `site/scripts/check-binding-proofs.mjs` and `sdlc/records/0425*`.
  - Land the workflow changes above before 0543 tags its final candidate, so the candidate runs them. Ian confirms or rotates the secrets before publication.
  - Release QA. A fresh docs-only agent installs the candidate artifacts, then asks questions including an admitted image question and a real MCP tool call. 0.1 question files load unchanged. 0.1 caches read through the compatibility path or refuse with one clear message before sending. Record docs failures with their owning tickets. `CHANGELOG.md`, the release notes and a written known-gaps list describe the final behavior, including images, MCP, the additive question format and 0.1 compatibility.
  - Publication, on Ian's explicit go. Tag the same commit `v0.2.0` and dispatch release mode. Move the public install text to 0.2.0 once and re-prove the site pages. With Ian's authorization, replace the RubyGems `ruby` platform 0.0.1 placeholder with the diagnostic gem.
  - After publication. Run 0398's install check on every channel, including R-universe once it syncs. Each channel installs 0.2.0 and replays the first-run sample. On Ruby 3.3 and on macOS's Python 3.9, install fails with the diagnostic message and never installs a 0.0.1 placeholder. The experiments team runs its selective 0035 follow-up against public 0.2 for the affected steps.
- Proof: The 0425 record cites 0543's candidate and names the QA result, the publication run and the install-check run.
- Defers: An approval notice, reuse of the rehearsal's built files, version-only proof hashing, install-text automation, a switch to silence the spend warning, Maven trusted publishing and an app token for the tap. They are ideas for after 0.2. The transcript-search recipe stays with the later issue `2026-10-05-recipe-search-transcripts.md`.

## Progress

- 2026-10-10 started
- 2026-10-10 next: Prepare the current reviewed candidate, run final local package and release qualification, then nonpublishing GitHub platform checks under the 2026-10-10 ruling; publishing remains on Ian’s go.
- 2026-10-10 landed 194218207; next: Current candidate docs landed; repair NuGet native-asset assembly and add the Windows SDK consumers before final qualification.
- 2026-10-10 landed 700532b98f9544f53397c7a31de94fa8436065fb; next: Current docs replays and installed dataframe checks pass after reviewed repairs; finish installed-package qualification, then run the nonpublishing candidate on GitHub.
- 2026-10-10 next: Await the 0530 fixture repair and complete installed qualification; Windows, Apple and fresh release QA remain unverified. No candidate or hosted workflow starts during Ian’s requested wrap-up; publication requires his go.
- 2026-10-10 next: Add the missing Windows public-channel install checks in lane 1 while 0530 finishes; then run the authorized nonpublishing candidate and fresh QA, with publication awaiting Ian.
- 2026-10-10 landed 973edbc28; next: Windows public-channel checks are reviewed and landed; await 0530 installed qualification, then run the authorized candidate and platform QA before asking Ian to publish.
- 2026-10-10 next: Stopped at Ian’s wrap-up request; after 0530 finishes, candidate/platform runs and fresh QA remain authorized but unstarted, and publication still requires Ian’s explicit go.
