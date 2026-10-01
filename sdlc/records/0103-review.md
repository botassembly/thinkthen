ACCEPT

# 0103 code review: exact empty-output checks

Reviewer: fresh read-only Claude session (Opus 5.5). Branch `ticket/0103-exact-empty-output-checks` at 7d64bbfc, worktree `thinkthen-0103` in the workspace. All planting ran in scratch copies, since deleted.

## What was checked, by command

1. Exact number checks. The diff removes 41 `mustmatch like "<digits or empty>"` lines and adds 37 `mustmatch "N"` lines (41 = 37 + the 4 empty ones). I ran 13 of the 37 by hand against the built binary and dumped raw bytes with `od -c`: `wc -l` of `--version`, `--help` `grep -c` of `--version` and `--help`, `decide -h` `grep -c` of `--model` and `--jobs`, the `key_env` count, the `THINKTHEN_BASE_URL` trailing-slash count, the `secret` count, exit codes 2 (ftp base), 5 (missing input file), 5 (bad UTF-8), the `--field /body --field /id` count, and the `--lines` count. Each output is one digit and a newline, and each exact check passes. The values are counts and exit codes, with no timestamps. `mustmatch "N"` trims surrounding white space: `printf '       1\n'` and `printf '1\n\n'` both pass `mustmatch "1"`. A BSD `wc -l` pad does not break it. `echo 12 | mustmatch "2"` fails where `like "2"` passed, which is the fix.
2. `test -z` lines. A stand-in `thinkthen` ran the real binary and then printed `planted` on stdout only, or on stderr only. Results (0 = pass):
   - `version.md:16` (stderr): real 0, stdout-plant 0, stderr-plant 1. Right stream.
   - `version.md:31` (stdout): real 0, stdout-plant 1, stderr-plant 0. Right stream.
   - `decide.md:151` (stdout): real 0, stdout-plant 1, stderr-plant 0. Right stream.
   - `decide.md:231` (stdout): real 0, stdout-plant 1, stderr-plant 0. Right stream.
   - The old `like ""` form of each line passed on all three binaries.
   - `mustmatch test` runs a block under `set -e`: a block with `test -z "x"` then a passing check fails. A `test -z` in the middle of a block therefore counts.
3. Runner refusal. In a scratch copy, a `like ""` and a `not like ''` appended to `spec/version.md` gave exit 1 with `demos: spec/version.md:36: ...` and `:37`. A `like ""` appended to `demos/01-refund-gate/README.md` gave exit 1 naming that file and line. `like "x"`, `not like "0"`, and exact `mustmatch ""` were not refused (exit 0, 21 green). The real pages hold many legitimate `like` uses and pass. With `demos` from origin/main, `demos-self-test` printed `loose-empty exited 0 and 1 was wanted`.
4. Self-test count. `cases` increments inside `expect`, which runs in the main shell. The script has 23 `expect` calls and none inside a loop or pipe. origin/main had 22 and printed a fixed 21. The run printed `23 cases pass`.
5. Files. The diff against origin/main touches only the issue, the ticket, the record, `sdlc/scripts/demos`, `sdlc/scripts/demos-self-test`, `spec/decide.md`, and `spec/version.md`. All are in the ticket scope. origin/main is 3 commits ahead (0100: workflow, issues, records). None of them touches `spec/` or `sdlc/scripts/`. `git merge-tree --write-tree origin/main HEAD` succeeds with no conflict.
6. Gates, run with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset: `sdlc/scripts/lint` exit 0. `sdlc/scripts/spec` exit 0: `demos-self-test: 23 cases pass`, `demos: 21 green, 0 red`. `sdlc/scripts/live` did not run.

## Notes (not blocking)

- The refusal pattern `mustmatch( not)? like (""|'')` misses `mustmatch -i like ""` and `mustmatch like -- ""`. Both still match any output. No page uses either form today. A later ticket could widen the pattern if one appears.
- The ticket's status line still says "in progress". The record says "awaiting code review". The owner updates both at landing.
