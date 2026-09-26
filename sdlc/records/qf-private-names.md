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
- `sdlc/scripts/lint` gains a first check. When `THINKTHEN_PRIVATE_NAMES` names a file, lint reads one fixed string per line and fails on any tracked file that contains one, ignoring case. It prints only the file and line of each hit. It refuses a list inside the repository and a list with no name. When the variable is unset, it prints that the check was skipped.
- `sdlc/scripts/allow-list` lets `THINKTHEN_PRIVATE_NAMES` through, since lint sources it first. The variable holds a path and no secret. Lint's pinned allow-list row and its planted child both gain the name, and `sdlc/scripts/README.md` says so.

## Checks

- The guard, run alone against the real list, fails with the four `site/` lines and exits 1. With a list of absent strings, it passes. With a blank list, a list inside the repository, or a planted upper-case string that exists in lower case, it fails as intended. Unset, it prints the skip line and passes.
- After the fix, the real list finds zero hits outside `site/`.
- `sdlc/scripts/lint` with the variable unset exited 0 and printed the skip line. The pinned allow-list row passed with the new name.
- `sdlc/scripts/lint` with the variable set to the real list stopped at the guard, exit 1, on the four `site/` lines. It turns green when they go.
- `sdlc/scripts/tickets` exited 0.

## Deferred

- Four `site/` lines, named in the issue's status line. The website agent owns them.
- Git history still holds the names. The launch step decides whether that matters.
- Generic-word repository names (for example a name that is also an ordinary English noun) are not in the guard list, because they would fail on ordinary prose. None of them names a private repository in the current text.
