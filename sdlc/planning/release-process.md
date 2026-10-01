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

1. A rehearsal dispatches the release workflow in rehearse mode by hand, under Ian's approval: `gh workflow run release.yml --ref main -f mode=rehearse`. After the cut, rehearsals run from the release branch: `--ref release/0.1` (ADR 0116 item 7).
2. Rehearse mode builds, smokes and collects a draft. It publishes nothing.
3. Rehearsals repeat until every job passes through `draft` on all four targets with zero "not run".
4. Ticket 0128 records each attempt and its result.

## 5. The release candidate and the release branch

[ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md) defines the release candidate, the cut and the cherry-pick rule.

The release branch is named for the major and minor version: `release/0.1`. Ticket 0128 phase 4 holds the details. The cut runs in this order:

1. **The release candidate holds on main.** The rehearsal is clean, release QA's latest round is clean, and only release fixes remain (ADR 0116 item 3).
2. **The release commit lands on main.** On a ticket branch from main, the agent runs `sdlc/scripts/versions --set 0.1.0`. That one command writes every version copy and drops the two publish holds. The same commit writes the text by hand:
   - `CHANGELOG.md`: date the 0.1.0 heading. `libraries/dart/CHANGELOG.md` gains a 0.1.0 heading.
   - `README.md`: the "Install" section of ticket 0128 phase 1 item 12.
   - The binding READMEs that name 0.0.1: `libraries/{ada,cobol,csharp,go,jvm,objective-c}`, and `databases/postgresql/NOTES.md`.
   - The site, through marketing: the held install lines of ticket 0128 phase 1 item 14, and `site/examples/install/rust/files/Cargo.toml`, whose requirement the site smoke patches to the working tree. Marketing then re-runs the bindings proof that pins that file's hash.

   `git grep -n '0\.0\.1'` afterwards finds only the copies ticket 0376 lists as fixtures, plants and history. The coordinator lands the commit.
3. **Checkpoint and QA on the cut.** The coordinator runs the checkpoint sweep of section 2 on that main commit. Release QA runs its round on it (section 3).
4. **The cut.** The coordinator tags `rc/0.1.0-rc.1` on that commit under worktrees.md and pushes `release/0.1` from it.
5. **The rehearsal from the release branch.** Ian dispatches `gh workflow run release.yml --ref release/0.1 -f mode=rehearse`. It passes on all four targets.
6. **Ian's go.** Ian tags `v0.1.0` on the head of `release/0.1` and dispatches release mode from the tag. Section 6 and ticket 0128 phase 4 continue from there.

After the cut, main carries 0.1.0 and takes 0.2 work. The 0.2 cut sets the next version. The coordinator cherry-picks each fix from main to the release branch (ADR 0116 item 5).

## 6. The release

1. The real release is dispatched only on Ian's go.
2. Release mode runs from a `v*` tag. Each publish job waits for Ian's approval in the GitHub `release` environment (ticket 0128).

## 7. Registries and the site

The docs team owns registry accounts and setup steps (ruling 15 of [cleanup-2026-09-30.md](cleanup-2026-09-30.md)) and the site (`site/`). The registries are PyPI, npm, crates.io, RubyGems, NuGet, Maven, pub.dev and the Homebrew tap. The queue owner owns the code and the release workflow. [ownership.md](ownership.md) names the folders.
