# Release process

Status: accepted 2026-10-01 on Ian's direction. Ian can overturn any step. This page is the one home of the coordinator's release process. Other pages link here and do not restate it.

## 1. Milestones

[milestones.md](milestones.md) holds the milestones, the `Milestone:` line rule and each milestone's open items.

## 2. Checkpoints

1. The coordinator runs one surface sweep on one main commit per round of landings, and fixes any red at once.
2. When every check passes, the sweep publishes: `sdlc/scripts/surfaces --publish checkpoint/surfaces/YYYY-MM-DD-N` with `THINKTHEN_PUBLISH_ROOT` set to the coordinator's builds folder. The [scripts README](../scripts/README.md) describes what the publish packs and checks.
3. The publish runs on Linux x86_64 only. A macOS publish stops by design on the Linux-only Ada, COBOL and Objective-C checks.
4. `publish-builds` keeps the newest three checkpoint folders and removes no other folder.
5. The coordinator tags the commit under [worktrees.md](worktrees.md), "Landing commits and tags".

## 3. Release QA rounds

1. Each checkpoint goes to the release QA team as one round. The coordinator sends it as a message through the mailroom (`pm send`).
2. QA findings come back as messages (`pm inbox`).
3. Each finding becomes a ticket, and the ticket lands through a lane like any other.
4. The next checkpoint carries the fixes into the next round.

## 4. Rehearsals

1. A rehearsal dispatches the release workflow in rehearse mode by hand, under Ian's approval: `gh workflow run release.yml --ref main -f mode=rehearse`. After the cut, rehearsals run from the release branch once ADR 0116 item 7 lands.
2. Rehearse mode builds, smokes and collects a draft. It publishes nothing.
3. Rehearsals repeat until every job passes through `draft` on all four targets with zero "not run".
4. Ticket 0128 records each attempt and its result.

## 5. The release candidate and the release branch

[ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md) defines the release candidate, the cut and the cherry-pick rule.

1. At the release candidate, the coordinator tags `rc/VERSION-rc.N` under worktrees.md.
2. The coordinator cuts `release/VERSION` from main.
3. The coordinator cherry-picks each fix from main to the release branch.

## 6. The release

1. The real release is dispatched only on Ian's go.
2. Release mode runs from a `v*` tag. Each publish job waits for Ian's approval in the GitHub `release` environment (ticket 0128).

## 7. Registries and the site

The docs team owns registry accounts and setup steps (ruling 15 of [cleanup-2026-09-30.md](cleanup-2026-09-30.md)) and the site (`site/`). The registries are PyPI, npm, crates.io, RubyGems, NuGet, Maven, pub.dev and the Homebrew tap. The queue owner owns the code and the release workflow. [ownership.md](ownership.md) names the folders.
