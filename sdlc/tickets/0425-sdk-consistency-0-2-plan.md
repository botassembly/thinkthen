# 0425: Put the complete SDK outcome into the 0.2 plan

Status: OPEN. Installed parity is complete; final qualification remains incomplete. Ian held release work on 2026-10-08 pending 0462 and the TCGA recognize decision; new release management requires his permission.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Every PM ask has a ticket owner, dependencies, retained behavior and an acceptance check. The current plan replaces superseded scope and lane orders. This planning ticket changes no product behavior.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 1–12.
- Keeps: Landed product behavior and existing qualified-run history. Specific publication approval remains required.
- Changes: Create 0426–0445; promote and amend 0296, 0300, 0406–0414, 0417 and 0418; retain 0393 as the npm owner. Update the milestone and run-facts issue. Answer all twelve mail asks with ticket numbers after fresh ticket reviews.
  Claim `sdlc/planning/**`, `sdlc/records/0425*`, `sdlc/scripts/surfaces*` and `.github/workflows/**`. The final installed run follows the new shared contract and migrations; release management remains held.
- Proof: Review each new or materially amended ticket with a fresh read-only reviewer. Check the twelve-ask map, nonduplicated ownership, dependencies, milestone lines and links. Run the ticket/documentation checks; no runtime or release experiment is needed for this planning-only change.
- Defers: Implementation, migrations, paid calls, registry/settings changes and further release runs. A later final-commit qualification remains an acceptance criterion, not current work.


## Required completion amendment, 2026-10-06

Authority: PM message `2026-10-06-pm-what-0-2-done-means-beyond-the-tickets-and-two-cleanups.md`, ask 1. Fresh High review accepted this amendment with 0456 after correcting its typed-document admission rule. The planning amendment does not qualify product completion. Keep all previous seventeen asks, core/host scope, release rehearsal and Ian's publication go.

Before publication, the final reviewed release candidate must pass the complete executed parity table including MCP 0455 (0432), full Linux gates and full hosted macOS gates on the existing `macos-15` and `macos-15-intel` release runners, the real Windows behavior qualification, and release QA/rehearsal. Required gates are install/lint/test/spec and affected complete surfaces/package checkpoints, with no required cells skipped. Use existing gates/runners; this plan adds no qualification workflow or runner feature. M5 performs bounded checks only; Yellow remains excluded from ThinkThen under the standing workspace README. Neither substitutes for hosted macOS qualification.

A fresh docs-only agent, given installed candidate artifacts and public docs but no repository context, must successfully install/start and ask, including an admitted image question and a real MCP tool call. Record concrete docs failures through their existing owners. Confirm 0.1 question files load unchanged; 0.1 caches read through the validated native compatibility path or fail with one clear actionable message before sending. Existing shared corpus, offline replay and public consumers provide these checks; no new proof framework.

After Ian's explicit publication go and the actual release, every published package must install and run on a clean machine from its public channel (0128/0398 and existing packaging owners), and the experiments team's selective 0035 follow-up must run against public 0.2, only for affected steps. A candidate/archive check cannot stand for a published-package check. Existing earlier selective notifications remain useful, but do not complete this public 0.2 criterion. Do not dispatch these publication-dependent checks prematurely. Docs, changelog, release notes and explicitly written known gaps must describe the final behavior, including images, MCP, the additive question format and compatibility. Release go and completion after public checks remain separate facts.

Current account prerequisite checklist is pending owner confirmation, not live-verified: Maven namespace/token setup; pub.dev publish/transfer/trusted-publishing setup; any unconfirmed trusted-publisher registrations. NuGet prefix reservation is optional. 0128/0389/0398 and each owning packaging ticket handle these account steps before release day; 0393 retains staged npm ownership and its maintainer approval. Report concrete login prerequisites to Ian through those owners; change no credential, registry or trust setting in this planning slice. Historical 0.1 account success is not a current account check.


## Two-stage release, 2026-10-07

Authority: Ian’s ruling in mailroom `2026-10-07-thinkthen-m5-use-and-the-two-stage-github-release.md`. Beelink remains primary. M5 may run experiments and Mac-specific checks, including before a candidate run, and does not run after every ticket.

1. Stage 1: after the completed installed parity table, remaining MCP timing, documentation and local checks, tag the final reviewed commit `rc/0.2.0-rc.N`. Dispatch `.github/workflows/release.yml` with mode `rehearse` and `.github/workflows/windows.yml` using that tag as their ref. Rehearsal covers Linux x86 and ARM, macOS 15 ARM and Intel, and Windows. Fix failures, review and check the fixes, then cut the next candidate. The resolver admits matching candidate tags with positive N; the release check requires a successful manual rehearsal from such a tag on the exact commit. 0425 owns this correction without a new ticket. No candidate has been cut by this amendment.
2. Stage 2: after successful candidate checks, release QA and Ian’s explicit publication go, tag the same qualified commit `v0.2.0` and dispatch release.yml in mode `release`. Keep the existing publication environment and run-specific approvals. Published-package checks follow actual publication.

Dispatch hosted workflows only for a release candidate or a Windows-only fix that cannot be checked locally. Batch Windows fixes into one dispatch. Do not run hosted checks after each ticket or treat Linux evidence as Windows/macOS qualification.


## Release hold, 2026-10-08

Ian's current instruction supersedes automatic stage-one progression. Candidate six's two workflows are canceled. Cut no new candidate tag, run no GitHub workflow and advance no release branch without permission. Complete the after-sprint review and bug sweep in 0462, retain 0461 pending the PM's TCGA ruling, and report the confirmed fixes and remaining work. The six-candidate retrospective is in the existing 0425 record. Publication still requires Ian's explicit go.
