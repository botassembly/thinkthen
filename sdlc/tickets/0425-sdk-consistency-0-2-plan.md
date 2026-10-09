# 0425: Qualify the 0.2 release candidate and publish on Ian's go

Status: OPEN. Ian's 2026-10-08 release hold governs this ticket. Cut no candidate tag, run no GitHub workflow, advance no release branch and publish nothing without his permission.

Milestone: 0.2

Depends on: 0508

Depends on: 0461

Depends on: 0468

Depends on: 0487

Depends on: 0510

Owner: builder.

Reviews: revision 1f8bc0179286c6b3ec6a87b11618e4fca5057be9, accept

## Outcome

After the 0508 final review, one reviewed commit passes a hosted release candidate on Linux, macOS and Windows, plus release QA by a fresh agent with only the public docs. On Ian's explicit go, that same commit is released as 0.2.0. Every published package then installs and runs from its public channel on a clean machine. This ticket is the single owner of candidate qualification, publication and the after-publication checks.

## Evidence

- Starts from: the [0.1 release record](../records/0128-release-0-1.md), the six-candidate retrospective in the [0425 record](../records/0425-sdk-consistency-0-2-plan.md), and three mailroom rulings in `repos/agents/inbox/thinkthen/`: the 2026-10-06 completion criteria (`2026-10-06-pm-what-0-2-done-means-beyond-the-tickets-and-two-cleanups.md`), the 2026-10-07 two-stage release (`2026-10-07-thinkthen-m5-use-and-the-two-stage-github-release.md`) and Ian's 2026-10-08 release hold. It also absorbs the release issues this rewrite closes: release and install for 0.1, run facts, the RubyGems placeholder, public install checks, R before release, rehearsal publish steps, two release secrets, two approvals, patch-release hand passes, and doc tests gating releases.
- Keeps: every release safety gate already built.
  - Both GitHub approvals stay as safety gates: the `release` environment and the `publish` step. 0393 keeps the npm staged approval.
  - 0398 keeps the exact-commit rehearsal guard and the after-publication install-check workflow.
  - The M5 runs bounded Mac checks only and never stands in for hosted macOS. Yellow stays excluded. No paid diagnostic runs.
  - A candidate or archive check never stands in for a published-package check.
- Changes: five stages, in order.
  - Before the first candidate. Every other open 0.2 ticket has landed, except 0383, 0384, 0385, 0455, 0474 and 0480, which the candidate finishes. Every fix ticket that 0508 names has landed. The executed parity table passes, including MCP 0455. Full Linux install, lint, test and spec gates pass. The checkpoint sweep runs `npm run test-docs` and names the result. The release workflow runs `site/scripts/check-binding-proofs.mjs` in strict mode and fails when `site/examples/bindings-proof.json` does not match the tagged commit. The release workflow builds the R source tarball from the published-crate shape before the GitHub release goes public. The 0425 record notes when the Maven Central token, its signing key and `TAP_DEPLOY_KEY` expire and when each was last rotated. Ian confirms or rotates each one before the candidate. No rehearsal reads a secret; 0398 keeps that rule. `install-check.yml` gains `windows-2025` rows for `install.ps1`, npm, NuGet, Maven Central and every other channel that ships Windows. Report each account prerequisite to Ian: the Maven namespace and token, pub.dev trusted publishing, and any unconfirmed trusted-publisher registration. Claim `.github/workflows/**`, `sdlc/planning/release-process.md`, `sdlc/scripts/surfaces*`, `sdlc/scripts/release-workflow`, `CHANGELOG.md`, `site/scripts/check-binding-proofs.mjs` and `sdlc/records/0425*`.
  - Candidate, with Ian's permission. Tag the commit `rc/0.2.0-rc.N`. Dispatch `.github/workflows/release.yml` in rehearse mode and `.github/workflows/windows.yml` on that tag. Cover Linux x86 and ARM, the `macos-15` and `macos-15-intel` runners, and Windows. The Windows run carries the remaining proofs of 0383, 0384, 0385, 0455, 0474 and 0480. Fix failures, review the fixes, batch Windows fixes, and cut the next candidate.
  - Release QA. A fresh docs-only agent installs the candidate artifacts, then asks questions including an admitted image question and a real MCP tool call. 0.1 question files load unchanged. 0.1 caches read through the compatibility path or refuse with one clear message before sending. Record docs failures with their owning tickets. `CHANGELOG.md`, the release notes and a written known-gaps list describe the final behavior, including images, MCP, the additive question format and 0.1 compatibility.
  - Publication, on Ian's explicit go. Tag the same commit `v0.2.0` and dispatch release mode. Move the public install text to 0.2.0 once and re-prove the site pages. With Ian's authorization, replace the RubyGems `ruby` platform 0.0.1 placeholder with the diagnostic gem.
  - After publication. Run 0398's install check on every channel, including R-universe once it syncs. Each channel installs 0.2.0 and replays the first-run sample. On Ruby 3.3 and on macOS's Python 3.9, install fails with the diagnostic message and never installs a 0.0.1 placeholder. The experiments team runs its selective 0035 follow-up against public 0.2 for the affected steps.
- Proof: The 0425 record names each candidate run, the QA result, the publication run and the install-check run.
- Defers: An approval notice, reuse of the rehearsal's built files, version-only proof hashing, install-text automation, a switch to silence the spend warning, Maven trusted publishing and an app token for the tap. They are ideas for after 0.2. The transcript-search recipe stays with the later issue `2026-10-05-recipe-search-transcripts.md`.
