# Lane 1 deleted by a cleanup trap

Status: recorded 2026-09-26 by the queue owner. The rules it adds are in `sdlc/planning/worktrees.md`. Ian can overturn them.

## What happened

Ticket 0128's builder was fixing code review findings for Phase 2 in lane 1. One fix added a shared helper, `sdlc/scripts/installed.sh`, with two functions:

- `installed_scratch PATH...` removes every path it is given when the script exits.
- `installed_tests REPO SURFACE [PATH...]` copies a surface's tests and calls `installed_scratch`.

`installed_tests` passed all of its arguments to `installed_scratch`, including REPO. The builder ran `release-smoke` to try the helper. The TypeScript installed check exited, and its exit trap ran `rm -rf` on the lane itself and on the relative path `libraries/typescript`.

## What was lost

- The builder's uncommitted work: the eight code review fixes, about 11 minutes of agent work. The builder's hand-back lists each fix, so they can be redone without new design work.
- The lane's build folders, about 9.6 GB. The next ladder rebuilds them cold, which took about 36 minutes for ticket 0136.
- The packed release files from the last smoke run. `release-pack` makes them again.

Nothing committed was lost. The branch `ticket/0128-phase-2-pack-and-smoke` stayed at `ade15093`, local and pushed. The main checkout and lanes 2 to 4 were untouched. The deletion stayed inside lane 1.

Git still lists lane 1 as prunable. Recreating it was blocked by the permission system and went to Ian.

## Root cause

Four things lined up.

1. A cleanup trap deleted whatever paths it was handed. It did not check that it had made them. One wrong argument turned a scratch cleanup into deleting the lane.
2. The new helper ran for the first time against the real lane. No plant or dry run proved it safe first.
3. The fixes sat uncommitted. The branch rule was "commit whole changes", so work in progress lived only in the lane.
4. The same folder held the only copy of the work and the target of a new deletion.

## What changes

These rules go into `sdlc/planning/worktrees.md` and the builder brief.

1. A script deletes only a path it created with `mktemp` in the same run. A cleanup function checks each path against the folders it made and refuses anything else with a nonzero exit.
2. A new or changed cleanup gets a plant before its first real run. The plant passes the lane path, and the guard must refuse it.
3. A builder commits and pushes work in progress to its own ticket branch after each fix. A lost lane then loses nothing. The landing merge keeps the history; nothing is squashed.
4. No worktree or lane is removed while `git status --porcelain` shows anything, or while the branch has commits that are neither on origin nor merged into main. `git worktree remove --force` is used only after both checks pass, to clear ignored build output.

The issue `sdlc/issues/closed/2026-09-26-scripts-can-delete-paths-they-did-not-create.md` proposes a lint check for rule 1.

## Found on the way

Before the deletion, the helper's scan of the old packed files found the builder's home path in the Python wheel's CycloneDX SBOM and in the Ruby gem's `thinkthen.so`. Those are real leaks in files meant to ship. Ticket 0128 fixes them in the build.
