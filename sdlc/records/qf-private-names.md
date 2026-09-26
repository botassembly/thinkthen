# Quick Fix qf-private-names: remove private names from the current text

Status: prepared on `ticket/qf-private-names` from main `5f43013e`. It carries out `sdlc/issues/2026-09-25-sdlc-history-names-private-repositories.md` outside `site/`. The issue stays open for four lines in `site/`, which the website agent owns. Ian can overturn the wording of any replacement and the guard.

## Why

This repository goes public. The workspace rule says a public repository never names a private project. The issue's sweep found 34 files that name a private repository or a path on one machine.

## Evidence

The builder built a list of private names outside the repository. It holds every repository folder name on this machine except the public ones, the name the history uses for a private caller, and the home-folder and workspace path forms. Hyphen, underscore, and joined spellings are all listed. The list has 31 strings. A case-insensitive `git grep` on main found 4 private names and 3 path forms: 93 mentions in 46 files outside `site/`, and 4 lines in `site/`. The files owned by in-flight tickets 0134, 0135, 0137, and 0138 hold none.

## Change

- Each mention now reads as a generic description: "the marketing repository", "a private experiment repository", "a private caller", "the workspace's `experiments/` folder", or "the workspace's decisions folder". A path that matters as evidence keeps its part inside that repository or folder. Worktree links in one review record now point at the file relative to the record.
- One mention was the ordinary word for hidden files in an R packaging comment. It now says "hidden files", so the guard passes.
- No file was deleted. No file exists only to describe a private project.
- `sdlc/scripts/lint` gains a first check. When `THINKTHEN_PRIVATE_NAMES` names a file, lint reads one fixed string per line, ignoring blank lines and case. It fails on any tracked path that holds a name and prints only the count and each path's number in `git ls-files`. It then fails on any tracked file whose text holds a name and prints only the file and line. No shell variable holds a name, so `sh -x lint` prints none. The list goes straight from `grep` into `git grep -f -`. A `grep` or `git grep` exit above 1 fails the check. It refuses a list inside the repository and a list with no name. When the variable is unset, it prints that the check was skipped. On this machine the list sits at `~/.config/thinkthen/private-names.txt`.
- `sdlc/scripts/allow-list` lets `THINKTHEN_PRIVATE_NAMES` through, since lint sources it first. The variable holds a path and no secret. Lint's pinned allow-list row and its planted child both gain the name, and `sdlc/scripts/README.md` says so.

## Ruling: Beatles Bench may be named

The review flagged Beatles Bench, the two-word bench project. Ian ruled on 2026-09-26 that it goes public with ThinkThen, so this repository may name it. It is not in the private-name list. The workspace `repos/README.md` says it stays private until Ian makes it public. That line does not block naming it here, because this ruling is Ian making it public alongside ThinkThen. Ian can overturn this ruling.

## Review fixes

The first code review found five points. Point 1 is the ruling above. The rest are fixed:

2. The guard checked file text only. It now checks tracked paths too, and a path hit prints no path text.
3. `sh -x lint` traced a variable that held every name. No variable holds a name now.
4. When `git grep` exited 128, the pipeline status came from `cut`, and the check passed. The check now reads `git grep`'s own status and fails above 1.
5. `sdlc/records/0088-review-codex.md` linked a file that no longer exists. That reference is now plain text, with a note naming the commit that moved the code and the file that holds it now.

## Checks

- Plants live in the builder's scratch folder, outside the repository. Each ran against the lint guard's own lines:
  - Plant 2: a scratch repository tracks a file whose path holds a planted name. The new guard exits 1 with "1 tracked paths name a private project, numbers in git ls-files: 1", and its output holds no planted text. The old guard passed it.
  - Plant 4: a `git` shim exits 128 on `grep`. The new guard exits 1 with "the private-name file check failed (git grep exit 128)". The old guard passed it.
  - Plant 3: `sh -x` over the guard with the real list. The new trace holds no project name on any line. The old trace held one on 56 lines. The trace still shows the list's own path and the repository path, as every lint trace does.

- The guard, run alone against the real list, fails with the four `site/` lines and exits 1. With a list of absent strings, it passes. With a blank list, a list inside the repository, or a planted upper-case string that exists in lower case, it fails as intended. Unset, it prints the skip line and passes.
- After the fix, the real list finds zero hits outside `site/`.
- `sdlc/scripts/lint` with the variable unset exited 0 and printed the skip line. The pinned allow-list row passed with the new name.
- `sdlc/scripts/lint` with the variable set to the real list stopped at the guard, exit 1, on the four `site/` lines. It turns green when they go.
- `sdlc/scripts/tickets` exited 0.

## Deferred

- Four `site/` lines, named in the issue's status line. The website agent owns them.
- Git history still holds the names. The launch step decides whether that matters.
- Generic-word repository names (for example a name that is also an ordinary English noun) are not in the guard list, because they would fail on ordinary prose. None of them names a private repository in the current text.
