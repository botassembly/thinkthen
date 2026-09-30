# The Dart check never runs under the surfaces rung

Status: open. Found while landing ticket 0314 slice 4c on main at `88649ec0d`. Owner: none yet.

Kind: debt

Pay when: before the next checkpoint tag, which needs every surface green on one commit.

Debt: 022

Severity: high

Keeping it means `surfaces` reports Dart as not run, so a Dart break can land unseen.

## The problem

Three things stop `libraries/dart/check.sh` under `sdlc/scripts/surfaces`:

1. Dart and Flutter live under `~/.local/opt/dart-3.13.4` and `~/.local/opt/flutter`, not on `PATH`. `check.sh:6-7` falls back to `TT_DART` and `TT_FLUTTER`, and `sdlc/scripts/allow-list` drops both. The check exits 77.
2. With Dart on `PATH`, `check.sh:88` (and `:32`, `:57`) takes `flock -w 180 -E 75` on `THINKTHEN_HEAVY_LOCK`, which the rung already holds through `flock -o`. The check waits 180 seconds and exits 75.
3. Past the lock, `pub get --offline` resolves `ffi` from `$CHECKS/scratch/pub-cache` (`check.sh:84`), which is empty. The allow-list drops `PUB_CACHE`. The check exits 69.

Run by hand with Dart on `PATH`, `PUB_CACHE=$HOME/.pub-cache` and no held lock, the check passes. A fix might add the toolchain paths and `PUB_CACHE` to the allow-list, and skip the inner lock when `THINKTHEN_HEAVY_LOCK_HELD` names it, as `heavy-lock` does.
