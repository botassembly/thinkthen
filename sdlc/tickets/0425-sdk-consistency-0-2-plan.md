# 0425: Pass 0.2 release QA and publish on Ian's go

Status: WITHDRAWN.

Opened as: 2026-10-19, superseded by 0687.

Milestone: 0.2.1

Depends on: 0687

Owner: builder.

Reviews: revision 1f8bc0179286c6b3ec6a87b11618e4fca5057be9, accept

Reviews: revision 37e6828a03d0cdfe1e168b4b7afbc0bffc1dbd7c, accept

Reviews: revision 045dfb5b1f0f99b6477aa5446345e9ffb81ef6ef, reject

Reviews: revision 5d9860455, accept

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

Superseded by: 0687

## Outcome

The commit that passed the 0543 candidate also passes release QA by a fresh agent with only the public docs. On Ian's explicit go, that same commit is released as 0.2.0. Every published package then installs and runs from its public channel on a clean machine. This ticket owns release QA, publication and the after-publication checks. 0543 owns release preparation and the candidate.

## Evidence

- Starts from: the [0.1 release record](../records/0128-release-0-1.md), the six-candidate retrospective in the [0425 record](../records/0425-sdk-consistency-0-2-plan.md), and the 2026-10-06 completion criteria, the 2026-10-07 two-stage release and Ian's 2026-10-08 release hold in `repos/agents/inbox/thinkthen/`. Release preparation and the candidate moved to 0543. This ticket also closes the release issues it absorbed: release and install for 0.1, run facts, the RubyGems placeholder, public install checks, R before release, rehearsal publish steps, two release secrets, two approvals, patch-release hand passes, and doc tests gating releases.
- Keeps: every release safety gate already built.
  - Both GitHub approvals stay: the `release` environment and the `publish` step. 0393 keeps the npm staged approval.
  - 0398 keeps the exact-commit rehearsal guard and the after-publication install-check workflow.
  - The M5 runs bounded Mac checks only and never stands in for hosted macOS. Yellow stays excluded. No paid diagnostic runs.
  - A candidate or archive check never stands in for a published-package check.
- Changes: `CHANGELOG.md`, the release notes, the known-gaps list, the public install text and `sdlc/records/0425-sdk-consistency-0-2-plan.md`.
- Remaining work, all on the candidate SHA after 0543 passes:
  - Release QA. A fresh agent with only the public docs installs the candidate on the Beelink and GitHub runners, asks an admitted image question, makes one real MCP tool call, and loads 0.1 question files unchanged. It uses recorded or local backends only. 0565 removed compatibility, so QA checks that a 0.1 cache is refused with its plain message before any send, not loaded. Docs failures go to their owning tickets.
  - Write `CHANGELOG.md`, the release notes and a known-gaps list that describe the final behavior: images, MCP, the additive question format and the 0.1 cache refusal.
  - The PM brings Ian's go list to Ian.
  - On Ian's go, tag the same commit `v0.2.0` and dispatch `release.yml` in release mode. Ian approves the `release` environment, the `publish` step and the npm staged publish. With Ian's authorization, replace the RubyGems `ruby` platform 0.0.1 placeholder with the diagnostic gem.
  - 0575 lands the docs on main and deploys thinkthen.dev, and 0577 pushes the DuckDB descriptor, under the same go.
  - Move the public install text to 0.2.0 once and re-prove the site pages.
  - Run 0398's `install-check.yml` on every channel, including R-universe after it syncs. Each channel installs 0.2.0 and replays the first-run sample. On Ruby 3.3 and on macOS's Python 3.9, install fails with the diagnostic message and never installs a 0.0.1 placeholder.
  - The experiments team runs its selective 0035 follow-up against public 0.2 for the affected steps.
- Ian's go list:
  - Give the expiry and rotation dates of the Maven Central token, the signing key and the Homebrew tap deploy key (`TAP_DEPLOY_KEY`), or rotate them. The 0425 record keeps the dates only.
  - Switch off direct publishing on the npm trusted publisher for `thinkthen`, so npm publishes through 0393's staged publish.
  - Give one go that covers the registries, the DuckDB pull request and the thinkthen.dev deploy.
- Proof: The 0425 record cites 0543's candidate and names the QA result, the publication run and the install-check run.
- Defers: An approval notice, reuse of the rehearsal's built files, version-only proof hashing, install-text automation, a switch to silence the spend warning, Maven trusted publishing and an app token for the tap are ideas for after 0.2. The transcript-search recipe stays with the later issue `2026-10-05-recipe-search-transcripts.md`.

## Progress

- 2026-10-10 started
- 2026-10-10 next: Prepare the current reviewed candidate, run final local package and release qualification, then nonpublishing GitHub platform checks under the 2026-10-10 ruling; publishing remains on Ian’s go.
- 2026-10-10 landed 194218207; next: Current candidate docs landed; repair NuGet native-asset assembly and add the Windows SDK consumers before final qualification.
- 2026-10-10 landed 700532b98f9544f53397c7a31de94fa8436065fb; next: Current docs replays and installed dataframe checks pass after reviewed repairs; finish installed-package qualification, then run the nonpublishing candidate on GitHub.
- 2026-10-10 next: Await the 0530 fixture repair and complete installed qualification; Windows, Apple and fresh release QA remain unverified. No candidate or hosted workflow starts during Ian’s requested wrap-up; publication requires his go.
- 2026-10-10 next: Add the missing Windows public-channel install checks in lane 1 while 0530 finishes; then run the authorized nonpublishing candidate and fresh QA, with publication awaiting Ian.
- 2026-10-10 landed 973edbc28; next: Windows public-channel checks are reviewed and landed; await 0530 installed qualification, then run the authorized candidate and platform QA before asking Ian to publish.
- 2026-10-10 next: Stopped at Ian’s wrap-up request; after 0530 finishes, candidate/platform runs and fresh QA remain authorized but unstarted, and publication still requires Ian’s explicit go.
