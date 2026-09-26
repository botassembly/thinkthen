# Quick Fix qf-scripts-delete-only-what-they-made: one guarded helper removes every scratch folder

Status: built in lane 1 on 2026-09-26, reviewed, not landed. It carries out rule 11 of `sdlc/planning/worktrees.md` and the issue `sdlc/issues/2026-09-26-scripts-can-delete-paths-they-did-not-create.md`. The coordinator lands it. Ian can overturn the helper's name, the two exceptions, and the stale-folder choice below.

## Result

- `sdlc/scripts/scratch.sh` is new. It sits beside `verdict.sh` and holds the guard that ticket 0128 Phase 2 wrote into `installed.sh`. `scratch_dir NAME [TEMPLATE]` makes a folder with `mktemp -d`, stops the script on a failed `mktemp`, and refuses a path with whitespace or glob characters, the current folder, or the root. It records the folder one path per line and sets NAME to it. The first call takes over the EXIT trap. `scratch_remove PATH` removes PATH only when it matches a recorded line exactly, and otherwise exits 1. `scratch_clean` reads the list one line at a time with `read -r`, so it does not depend on the shell's word separators.
- The name `scratch.sh` follows the issue's `scratch_dir` and `scratch_clean`. It names what the file holds: folders a run makes and removes. `installed.sh` keeps only the installed-file steps. Its `installed_scratch` and `backend_start` now call `scratch_dir`, and each caller sources `scratch.sh` first.
- Every other recursive `rm` on a variable in `sdlc/scripts/`, `libraries/*/check.sh`, and `databases/*/check.sh` now goes through the helper. The count on main was 24 lines, where the issue counted 20. Ticket 0128 added the new ones. `time-limit` and `lint` drop their own removals and let the EXIT trap clean. `package` makes its unpacked source and its build folder as separate scratch folders, so it removes each by exact path. `release-pack`'s `fresh` makes a new `mktemp` folder in the stage, set to mode 755 so each archive root keeps its old mode. `databases/postgresql/runtime.sh` makes its run folder through `scratch_dir`, and `check.sh`'s cleanup calls `scratch_clean`.
- Two lines stay as named exceptions in `scratch_lint`. `demos-self-test` removes its literal build folder `target/demos-standard`. The Python check removes a toolchain venv it just failed to make. That venv sits at a fixed cache prefix plus a hash, and the line above names it. Neither folder comes from `mktemp`.
- `runtime_open` still removes a killed run's folder, which `.runtime/last-run` names. That path comes from a file, so it now passes a guard first. Its name must be this TMPDIR's canonical path, then `tt-pg.`, then six letters or digits. Anything else prints a refusal and is skipped. The check does not exit there, so a TMPDIR change between runs cannot fail it.
- `lint` calls `scratch_lint` over the scanned scripts. It then plants a file holding `rm -f -r -- "$x"` in a scratch folder and requires `scratch_lint` to fail on it. It also seeds the list with `$REPO/x` and requires `scratch_remove "$REPO"` to refuse, with `rm` replaced by an exit. The pattern catches `-r`, `-rf`, `-fr`, `-Rf`, `-f -r`, and `--recursive`.
- `sdlc/scripts/README.md` lists `scratch.sh`. The issue stays open for the scripts outside the scan and lists them.

The change adds 107 nonblank lines and removes 87, a net of 20 in scripts and the README, before this record and the issue update. It touches no Rust, so the ratchet holds at 67758/67758.

## Plants

Each plant ran before any real run of the changed cleanup. A recorder script named `rm` came first on PATH, so nothing could be deleted. The plants lived in a `mktemp` folder under the session scratchpad's `tqf-rm/`.

| Plant | Exit | Removals recorded |
|---|---|---|
| `TMPDIR=/nonexistent`, then `scratch_dir x` | 1, from `mktemp` | none |
| TMPDIR holding a space, then `scratch_dir x` | 1, `scratch.sh: refusing .../a b/tmp.…` | none |
| `scratch_remove` on the lane path | 1, `refused to remove /home/…/thinkthen-lane-1` | none |
| `scratch_remove` on `libraries/typescript` under the lane, absolute | 1, refused | none |
| The same path, relative, from the lane | 1, refused | none |
| `scratch_dir x`, then `scratch_remove` on the lane path | 1, refused | only the folder `x` made, at exit |
| Control: `scratch_dir x`, then `scratch_remove "$x"` | 0 | only `x` |
| Control: two `scratch_dir` calls, then exit | 0 | only the two made folders |
| Stale run named the lane path, through `runtime_open` | refused | only the new run folder, at exit |
| Stale run named a path under the lane | refused | only the new run folder |
| Stale run named `$TMPDIR/tt-pg.ab/c12` | refused | only the new run folder |
| Stale run named the relative `abc123` | refused | only the new run folder |
| Control: stale run named a real `$TMPDIR/tt-pg.abc123` | removed | that folder and the new run folder |
| `scratch_lint` on a planted tree holding `rm -rf "$x"` | 1, names the planted line | none |
| `scratch_lint` on the lane | 0 | none |

The pattern plants each held one line. `rm -rf "$x"`, `rm -r`, `rm -fr`, `rm -Rf`, `rm -f -r --`, `rm --recursive`, and `x=1; rm -rf "$x"` each exited 1. `rm -f "$x"`, `rm "$x"`, `trim -r x`, and `form -rf x` each exited 0.

## Review

A fresh read-only Opus reviewer read the diff and the rule 11 record. It found five items to fix. The postgresql release part still packed the old fixed stage folder. `mktemp` folders gave each archive root mode 0700. `runtime.sh` removed a stale path it read from disk with no guard. `scratch_lint` had no in-rung plant and missed `rm -f -r` and `--recursive`. The lint plant ran on an empty list, so a prefix match bug would pass it. All five were fixed in `1e210bd9`, and the same reviewer returned ACCEPT on that commit. Its sixth note, the scripts outside the scan, went to the issue.

## Checks

The branch merged `origin/main` at `34995fef`. `node sdlc/scripts/ratchet.mjs` printed `ratchet: crates + conformance 67758/67758`.

Each rung ran once on `1e210bd9` with `THINKTHEN_API_KEY` unset. The rungs took the heavy lock themselves.

| Rung | Exit | Result |
|---|---|---|
| `lint` | 0 | `scratch_lint` and both plants held, `inventory: 353 declared items checked, 4 plants refused` |
| `test` | 0 | `live-test: all cases passed` |
| `spec` | 0 | `demos: 21 green, 0 red` |
| `surfaces` | 0 | all 10 surfaces pass, 8 release files pass their installed checks, `surfaces: pass release smoke`, 499 s |

The surfaces run exercised the changed cleanups in every library check, `release-pack`, `release-smoke`, `time-limit`, and the PostgreSQL runtime. The first `lint` and `test` also ran on `5aa39c56` before the review fixes, and both exited 0.

`sdlc/scripts/live` did not run.

## Found on the way

`databases/duckdb/check.sh` line 38 makes three `mktemp -d` folders for `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME` and never removes them. Each surfaces run leaves six in `/tmp`, from the full and installed modes. It deletes nothing, so it is a leak and not a rule 11 hazard. It is on main before this fix. This fix left the six folders from its run in place, because no script here made them.

## Deferred

- The scripts outside the scan keep their own `rm -rf`. The issue lists them.
- `scratch_dir` leaves the folder a refused `mktemp` made, such as one under a TMPDIR holding a space. It removes nothing it has not recorded.
