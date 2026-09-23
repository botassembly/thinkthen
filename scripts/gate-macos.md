# The Mac re-proof recipe

This file exists because the committed Mac evidence was captured at
branch commit `c3616dc` and no Mac has run since; the record's README
says so. When a Mac is next available, run this recipe and replace the
evidence under `sdlc/records/2026-09-22-macos-artifacts/` with fresh
logs at the then-current tip.

The check scripts carry experimental darwin spellings behind guards, but
the guards have not run on darwin since the wave that added them; treat
every `uname`-guarded path here as unproven until this recipe is
executed. Known Linux-isms the guards must cover (fourth review): `.so`
versus `.dylib` names, `LD_LIBRARY_PATH` versus `DYLD_LIBRARY_PATH`,
GNU `timeout` versus `gtimeout`, `ss` (absent on macOS — the stub probe
falls back to `curl`), GNU `sed -i` versus `sed -i ''`,
`/usr/bin/pg_config` (Homebrew lives elsewhere), and the
`x86_64-unknown-linux-gnu` targets in the package scripts, which are
Linux-only by design and skip on darwin.

Recipe, one surface at a time:

1. `cd libraries/typescript && ./build-addon.sh` — the addon has
   `build.rs`; `npx napi build` produces the `.node` directly.
2. `cd libraries/ruby && ./build.sh synthetic` — the gem platform comes
   from `Gem::Platform::CURRENT`; verify the spec names `darwin-arm64`
   and the built file is `thinkthen.bundle`, not `.so`.
3. `cd libraries/c && ./check.sh` — the release build remaps the home
   prefix; verify `strings` on the `.dylib` shows no builder path.
4. `cd databases/sqlite && ./package.sh --dry-run` — darwin builds a
   plain `.dylib` (zig targets are Linux-only).
5. `cd databases/duckdb && ./package.sh --dry-run` — the extension
   suffix and architecture follow the host.
6. PostgreSQL on a Mac needs a local server; skip it and record why.

Each step's log lands in the records folder with the tip's commit hash
in its first line. Nothing here installs into the user's Home; the
scratch rule from the original visits still holds.
