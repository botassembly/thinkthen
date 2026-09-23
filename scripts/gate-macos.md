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

Three of these no longer depend on a guard (surfaces-review-7 R3-32).
The scripts spell in-place edits with `perl -pi`, read sub-second time
through perl's Time::HiRes because BSD `date` has no `%N`, and resolve
`timeout` or `gtimeout` once into `$TIMEOUT`.
`scripts/check_portable_shell.sh` runs in the gate and fails on a bare
`sed -i`, a `date` format with `%N`, or a bare `timeout`. A line that
names one of them as data ends with `# portable-shell: data`, and the
check skips that line only. The
`libthinkthen_native.so` name in `libraries/ruby/build.sh` stays,
because that copy runs inside the Linux builder container (step 2).

Recipe, one surface at a time. Steps 2, 4, and 5 cannot run on macOS
today (surfaces-review-5). Each says why. Run the others, and record the
three as not runnable with the reason below.

1. `cd libraries/typescript && ./build-addon.sh` — the addon has
   `build.rs`; `npx napi build` produces the `.node` directly.
2. Not runnable on macOS. `libraries/ruby/build.sh` builds inside a Linux
   builder container and copies `libthinkthen_native.so`, so it never
   produces a darwin `thinkthen.bundle`. A native macOS gem build needs
   its own script first.
3. `cd libraries/c && ./check.sh` — the release build remaps the home
   prefix; verify `strings` on the `.dylib` shows no builder path.
4. Not runnable on macOS. `databases/sqlite/package.sh` exits 1 on
   Darwin: "this rehearsal cross-builds for Linux; on macOS build
   natively". Run `databases/sqlite/check.sh` instead, which builds the
   `.dylib`.
5. Not runnable on macOS. `databases/duckdb/package.sh` exits 1 on
   Darwin with the same sentence, and it has no `--dry-run`. Run
   `databases/duckdb/check.sh` instead.
6. PostgreSQL on a Mac needs a local server; skip it and record why.

Each step's log lands in the records folder with the tip's commit hash
in its first line. Nothing here installs into the user's Home; the
scratch rule from the original visits still holds.
