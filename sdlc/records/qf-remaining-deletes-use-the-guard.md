# Quick Fix qf-remaining-deletes-use-the-guard: the remaining recursive deletes use scratch.sh

Status: built in lane 1 on 2026-09-26, reviewed, not landed. It finishes rule 11 of `sdlc/planning/worktrees.md` and closes the issue `sdlc/issues/closed/2026-09-26-scripts-can-delete-paths-they-did-not-create.md`. The coordinator lands it. Ian can overturn the lint scope, the seven exceptions, and the choice to route demo 16 and the Ruby setup below.

## Result

- `databases/duckdb/check.sh` sources `scratch.sh` once, near the top. `stock_cli` makes its `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME` folders through `scratch_dir`, so the EXIT trap removes them. Before this fix, each surfaces run left six `/tmp/tmp.*` folders. The deny plant file now sits in its own scratch folder, because the first `scratch_dir` call takes over the EXIT trap that used to remove it.
- Twenty scripts replace `x=$(mktemp -d)` and their own `rm -rf` trap with a source line and `scratch_dir x`:
  - `databases/duckdb/tools/selftests.sh`, `databases/sqlite/setup.sh`, and `databases/sqlite/tests/host_sqlite.sh`.
  - `libraries/python/build-wheel.sh` and `libraries/r/tools/make-tarball.sh`.
  - The eight `transforms/*` tests.
  - `probes/replay-check.sh`, and the `run` and `self-test` of `find-0040` and `probability-total-0038`.
  - Demo 16's `run` and `record.sh`.
- Four scripts keep their own trap and call `scratch_clean` in it, after the first `scratch_dir`. `libraries/r/tests/with-backend.sh` still closes fd 3 first. Demo 16's `self-test` still stops its child first. `sdlc/live-test` still runs `chmod -R u+w` first. Its folder now comes from `mktemp`, where it used to be `thinkthen-live-test.$$`.
- `libraries/ruby/setup-ruby.sh` makes its install stage with `scratch_dir` beside the build folder and removes it with `scratch_remove`. The stage used to be the fixed path `$prefix.partial`. It keeps the build folder with its logs, as before.
- `databases/postgresql/runtime.sh` extracts the server package into a `scratch_dir` folder beside the toolchain and moves it into place. It used to remove and reuse `$EXTRACTED.partial`. An old `.partial` folder from an earlier run stays where it is.
- `scratch_dir` now exits on a hangup as well as an interrupt, so the EXIT trap runs. The traps it replaced listed HUP.
- `scratch_lint` reads `sdlc/scripts/*`, `sdlc/live-test`, `install.sh`, and every tracked `*.sh` or `#!/` file under `libraries/`, `databases/`, `transforms/`, `probes/`, and `demos/`, leaving out Markdown. That is 108 files beside `sdlc/scripts/`. It fails outside a Git checkout, so the scan cannot shrink quietly.
- `sdlc/scripts/README.md` describes the wider scan. The issue is closed and moved to `closed/`. The two records that linked to it point to the new path.

## Exceptions

`scratch_lint` names seven lines. None of the paths comes from `mktemp`.

| Line | Reason |
|---|---|
| `demos-self-test`: `rm -rf -- "$REPO/$ROOT"` | The demos check's literal build folder, from the last Quick Fix |
| `libraries/python/check.sh`: `rm -rf -- "$venv"` | A failed toolchain venv at a fixed cache prefix and a hash, from the last Quick Fix |
| `runtime.sh`: the killed run's folder | It comes from `.runtime/last-run` and passes a name guard first, from the last Quick Fix |
| `runtime.sh`: `rm -rf .runtime/tree` | A literal build path under the surface's own folder, after `cd`. Moving it into the run folder would copy the server tree into TMPDIR on every run |
| `make-tarball.sh`: `.cargo` and `target` under `$PKG` | Both sit inside the script's own scratch folder, after `cp -R` |
| `install.sh`: `rm -rf -- "$work"` | The installer runs from `curl … \| sh`, so it cannot source a repo helper. `sdlc/scripts/versions` requires it to equal `site/public/install.sh`, which the marketing lead owns. Its `work` comes from `mktemp -d` under `set -e`, and the cleanup removes only that path |
| Demo 16 `triage`: `rm -rf -- "$output"` | The page shows it as the reader's complete safe script, so it stands alone. It removes only the output folder its own `mkdir` made, and `mkdir` refuses an existing folder |

The R package's `Makevars.in` clean rules are make rules that ship in its tarball. The scan leaves them out, and the lint comment says so. Demo 19's `rm -rf build/` is fixture text for the tool to judge.

The change adds 80 nonblank lines and removes 63, a net of 17 in scripts and the README, before this record and the issue update. It touches no Rust.

## Plants

Each plant ran before any real run of a changed cleanup. Recorder scripts named `rm` and `chmod` came first on PATH, so nothing could be deleted. The plants lived in a `mktemp` folder under the session scratchpad's `tqf-rm2/`. The recorder left the `/tmp` folders the harness made. I removed those eight empty folders with `rmdir` by the names the recorder logged.

| Plant | Scripts | Exit | Removals recorded |
|---|---|---|---|
| `TMPDIR=/nonexistent` | The 8 transforms tests, sqlite `setup.sh` and `host_sqlite.sh`, duckdb `selftests.sh`, demo 16 `run` and `self-test`, `build-wheel.sh`, `with-backend.sh`, both probe self-tests, the `live-test` head, and the duckdb and `make-tarball.sh` `scratch_dir` lines | 1, from `mktemp` | none |
| TMPDIR holding a space | The same 20 | 1, `scratch.sh: refusing …/a b/tmp.…` | none |
| `runtime.sh` partial, toolchain `/nonexistent` | the `scratch_dir partial` line | 1, from `mktemp` | none |
| `runtime.sh` partial, toolchain holding a space | the same line | 1, refused | none |
| `setup-ruby.sh` stage, tools folder holding a space | the `scratch_dir stage` line | 1, refused | none |
| `scratch_remove` on the lane path | the `setup-ruby.sh` and demo 16 `self-test` shape | 1, `refused to remove LANE` | none |
| `scratch_remove` on `LANE/libraries/typescript`, absolute and relative | the same | 1, refused | none |
| `scratch_dir w`, then `scratch_remove` on the lane, and on the path under it | the trap shape | 1, refused | only `w`, at exit |
| The `live-test` head with `TMP` reset to the lane, and to the path under it | `live-test` lines 1 to 9 | 0 | a `chmod` of that path, and only the made folder removed |
| Control: `scratch_dir w`, then `scratch_remove "$w"` | | 0 | only `w` |
| Control: the `live-test` head | | 0 | `chmod` and removal of its made folder |

The plants ran the real scripts where the guard comes before any other work. The `live-test` plant ran a copy of its first nine lines with `ROOT` set to the lane, then `exit 0`. The `live-test` rows exit 0 because `scratch_clean` never sees the reset path. Its `chmod` still follows `TMP`, and `live-test` never resets `TMP`.

Lint plants: I appended `rm -rf "$x"` to one newly scanned file at a time, ran `scratch_lint`, and restored the file with `git checkout`. The files were `transforms/cost/test.sh`, `probes/find-0040/self-test`, `sdlc/live-test`, `install.sh`, and `databases/postgresql/runtime.sh`. Each run exited 1 and named the planted line. The clean tree exited 0. Outside a Git checkout, `scratch_lint` exited 1.

## Review

A fresh read-only Opus reviewer returned ACCEPT on `8d4990ef`. It checked each source path against the script's working folder, subshells, trap takeover in each script with its own trap, the behavior changes, and the scan list, which it rebuilt by hand. It noted three items: this record was missing; `Makevars.in` was unnamed; and the scan would shrink quietly outside a Git checkout. `af7c25f4` fixed the second and third. The same reviewer returned ACCEPT on that commit. This record fixes the first.

## Checks

The branch merged `origin/main`, which had no new commits. `node sdlc/scripts/ratchet.mjs` printed `ratchet: crates + conformance 69097/69097`.

`lint` ran on `7a31bc52`. `af7c25f4` then changed only `scratch_lint`, which exited 0 on the lane when run directly. `test`, `spec`, and `surfaces` ran on `af7c25f4`. Each rung ran once with `THINKTHEN_API_KEY` unset. The rungs took the heavy lock themselves.

| Rung | Exit | Result |
|---|---|---|
| `lint` | 1 | `agents file: CLAUDE.md 5641/5000 characters`. `AGENTS.md` is 5641 characters on `origin/main` too, so the failure is already on main. `scratch_lint` and its two plants passed before that check. A `mktemp` copy of `lint` with only that exit turned off exited 0: `inventory: 353 declared items checked, 4 plants refused` |
| `test` | 0 | `live-test: all cases passed`, 100 s |
| `spec` | 0 | `demos: 21 green, 0 red`, 25 s |
| `surfaces` | 0 | all 10 surfaces pass, 8 release files pass their installed checks, `surfaces: pass release smoke`, 770 s |

`sdlc/scripts/live`, the probe `run` scripts, and demo 16's `record.sh` did not run. The `test` rung runs `sdlc/live-test` itself, with dummy keys and local jobs.

I listed `/tmp/tmp.*` before and after `surfaces` and deleted nothing. The before list held 954 folders, and the after list held 953. No folder appeared, and one that another process owned went away. The DuckDB check left none of its six folders.

## Deferred

- `AGENTS.md` is over lint's 5,000-character cap on main. Its owner is the queue owner, and this fix does not touch it.
- An old `$EXTRACTED.partial` or `$prefix.partial` folder from before this fix stays in the toolchain cache. No script removes it now.
