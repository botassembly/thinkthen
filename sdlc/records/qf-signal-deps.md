# Quick Fix qf-signal-deps: reconcile the signal dependencies after 0078

Status: landed. Closes `sdlc/issues/2026-09-24-reconcile-signal-dependencies-after-0078.md`. Ticket 0078 made `nix` (feature `signal`) a Unix library dependency and made `signal-hook` optional under `cli`. Tickets 0084 and 0086 still carried the old split. A fresh read-only Opus review is in `sdlc/records/qf-signal-deps-review.md`.

## Result

- 0084 on main: the package-proof sentence now rejects `clap`, `csv-core`, and `signal-hook`, and allows `nix`. A dated amendment line records the change. `wc -m` reads 29,994, under the 30,000 cap. Commit `4f723af6`.
- 0086 on `ticket/0086-public-rust-api`: line 41 now names `signal-hook` as CLI-only and keeps `nix` with feature `signal` in the Unix library graph. A dated amendment line records the change and says 0086 adds no dependency. Commit `bed6a4e6`, pushed. No other line in 0086 stated the old split.

The issue says this edit does not reopen full review of either ticket.

## Checks

`sdlc/scripts/lint` exited 0 in the Quick Fix worktree. `git diff --check` passed in both worktrees. `sdlc/scripts/live` did not run.
