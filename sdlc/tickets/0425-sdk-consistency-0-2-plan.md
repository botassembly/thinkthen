# 0425: Put the complete SDK outcome into the 0.2 plan

Status: landed. The complete reviewed 0.2 plan assigns every PM ask, dependencies and acceptance checks. Product implementation remains open in its owning tickets.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Every PM ask has a ticket owner, dependencies, retained behavior and an acceptance check. The current plan replaces superseded scope and lane orders. This planning ticket changes no product behavior.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 1–12.
- Keeps: Landed product behavior and existing qualified-run history. Specific publication approval remains required.
- Changes: Create 0426–0445; promote and amend 0296, 0300, 0406–0414, 0417 and 0418; retain 0393 as the npm owner. Update the milestone, team note and run-facts issue. Answer all twelve mail asks with ticket numbers after fresh ticket reviews.
- Proof: Review each new or materially amended ticket with a fresh read-only reviewer. Check the twelve-ask map, nonduplicated ownership, dependencies, milestone lines and links. Run the ticket/documentation checks; no runtime or release experiment is needed for this planning-only change.
- Defers: Implementation, migrations, paid calls, registry/settings changes and further release runs. A later final-commit qualification remains an acceptance criterion, not current work.


## Required completion amendment, 2026-10-06

Authority: PM message `2026-10-06-pm-what-0-2-done-means-beyond-the-tickets-and-two-cleanups.md`, ask 1. This amendment awaits fresh review with 0456; the original planning landing does not accept this amendment or qualify product completion. Keep all previous seventeen asks, core/host scope, release rehearsal and Ian's publication go.

Before publication, the final reviewed release candidate must pass the complete executed parity table including MCP 0455 (0432), full Linux gates and full hosted macOS gates on the existing `macos-15` and `macos-15-intel` release runners, the real Windows behavior qualification, and release QA/rehearsal. Required gates are install/lint/test/spec and affected complete surfaces/package checkpoints, with no required cells skipped. Use existing gates/runners; this plan adds no qualification workflow or runner feature. M5 performs bounded checks only; Yellow remains excluded from ThinkThen under the standing workspace README. Neither substitutes for hosted macOS qualification.

A fresh docs-only agent, given installed candidate artifacts and public docs but no repository context, must successfully install/start and ask, including an admitted image question and a real MCP tool call. Record concrete docs failures through their existing owners. Confirm 0.1 question files load unchanged; 0.1 caches read through the validated native compatibility path or fail with one clear actionable message before sending. Existing shared corpus, offline replay and public consumers provide these checks; no new proof framework.

After Ian's explicit publication go and the actual release, every published package must install and run on a clean machine from its public channel (0128/0398 and existing packaging owners), and the experiments team's selective 0035 follow-up must run against public 0.2, only for affected steps. A candidate/archive check cannot stand for a published-package check. Existing earlier selective notifications remain useful, but do not complete this public 0.2 criterion. Do not dispatch these publication-dependent checks prematurely. Docs, changelog, release notes and explicitly written known gaps must describe the final behavior, including images, MCP, the additive question format and compatibility. Release go and completion after public checks remain separate facts.

Current account prerequisite checklist is pending owner confirmation, not live-verified: Maven namespace/token setup; pub.dev publish/transfer/trusted-publishing setup; any unconfirmed trusted-publisher registrations. NuGet prefix reservation is optional. 0128/0389/0398 and each owning packaging ticket handle these account steps before release day; 0393 retains staged npm ownership and its maintainer approval. Report concrete login prerequisites to Ian through those owners; change no credential, registry or trust setting in this planning slice. Historical 0.1 account success is not a current account check.
