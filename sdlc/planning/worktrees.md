# Worktrees and lanes

Status: accepted 2026-09-26 on Ian's direction, as a trial in this repository. Ian can overturn it. If the trial holds, other Rust repositories can adopt it.

## Why

Every ticket used to get its own worktree. Each worktree built its own Rust and surface build folders of 4 to 5 GB, and landed worktrees stayed on disk for `workspace sweep`. On 2026-09-26 they filled the Beelink disk and stopped every build. Removing 64 landed worktrees raised free space from 43 GB to 220 GB. A worktree without its build folders is about 30 MB, so the build output is the whole cost.

## Rules

1. All work happens in a worktree off `origin/main`, never in the main checkout.
2. The coordinator keeps three lanes, `claude-0` to `claude-2`, since Ian's ruling of 2026-10-04. Each warm lane holds 15 to 40 GB of build output across the repository's Rust workspaces and language builds. Five lanes filled the disk. Ian authorizes more when independent ready work and host capacity justify them. Reuse the long-lived worktrees listed in the Lanes table of the current team note, `team-0-2-2026-10-04.md`. Independent builds may overlap under the work plan's Build capacity policy, using isolated outputs and lane-specific locks. Retain exclusion for shared mutable installations or the same output, and check disk space before adding a lane.
3. The coordinator assigns each ticket, Quick Fix, or experiment a free lane and names the lane in the ticket or brief. One agent holds a lane at a time.
4. To start, the agent checks that `git status --porcelain` is empty in the lane, then runs `git switch -c ticket/NNNN-slug origin/main`. The lane keeps its build folders, so the ladder rebuilds only what changed since the lane's last ticket.
5. After landing, the agent that landed the branch runs `git switch --detach origin/main` in the lane and deletes the local ticket branch. The lane stays, with its build folders, for the next ticket. Remove that ticket’s owned scratch folders and logs under `target/`, keeping warm build folders. The total lane cap is 40 GB, including `libraries/` and `databases/`; these remain per lane because a shared copy would mix branches. Fully clean an idle lane only when free space falls below 50 GB. Check for 50 GB free before the full parity campaign.
6. A worktree outside the lanes, such as one made while every lane was busy, is removed with `git worktree remove` by the agent that lands it. A branch kept unmerged on purpose is pushed first, then its worktree is removed.
7. `workspace sweep` is a backup for people. Agents never leave a worktree for it.
8. No build runs as root inside a worktree. Root-owned build files cannot be removed without root, so a root container test gets its own copy of the checkout.
9. A builder commits and pushes work in progress to its own ticket branch after each fix, so a lost lane loses nothing. The landing merge brings the whole change to main at once.
10. No worktree or lane is removed while `git status --porcelain` shows anything, or while its branch has commits that are neither pushed nor merged into main. `--force` is used only after both checks pass, to clear ignored build output.
11. A script deletes only a path it created with `mktemp` in the same run. A cleanup checks each path against the folders it made and refuses anything else. A new or changed cleanup gets a plant that passes the lane path, and the guard must refuse it before the cleanup's first real run. See `sdlc/records/2026-09-26-lane-1-deleted-by-a-cleanup-trap.md`.


## Landing commits and tags

Ian approved this convention on 2026-09-30. Tools read these lines from Git, so the records need not repeat them.

1. A landing merge's subject reads `Land NNNN: outcome` or `Land NNNN slice X: outcome`. Its body ends with trailers: `Ticket: NNNN`, `Slice: X` when the ticket lands in slices, and `Review: accept`. A Quick Fix uses `Ticket: quick-fix`. Trailers never name an agent.
2. The coordinator tags a main commit `checkpoint/surfaces/YYYY-MM-DD-N` only after every surface check passed on that exact commit. The annotated message lists each check and its result.
3. A release candidate gets `rc/VERSION-rc.N` before the release suite runs. A phase start gets `phase/NAME`.
4. Landings get no tags; the trailers cover them. The coordinator batches its plan records into one commit per round of landings.

## Trial

The first two tickets built in lanes record in their build record the ladder time on a warm lane and the lane's disk size. The last cold ladder in a new worktree is the comparison. If a warm lane is not clearly faster, the repository returns to one worktree per ticket, removed at landing.

Worktrees made before this rule finish where they are and are removed when they land.
