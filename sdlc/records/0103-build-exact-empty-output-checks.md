# 0103: Exact empty-output checks

Status: landed. A fresh review of `7d64bbfc` accepted it (`sdlc/records/0103-review.md`). Its one note, the refusal missing option forms, is fixed below. The coordinator accepted that fix without another review.

## Result

- `spec/version.md` and `spec/decide.md` check an empty stream with `test -z`, the form 0083 used in `spec/transform.md`. The empty-record line in `spec/decide.md` now captures the output first, so `set -e` also checks that the command succeeds.
- `sdlc/scripts/demos` refuses `mustmatch like ""` and `mustmatch not like ""` in `spec/*.md` and in every page under the demo root, red or green. The failure names the file and line.
- `sdlc/scripts/demos-self-test` gains the `loose-empty` case. It counts its cases instead of printing a fixed number, which read 21 while 22 cases ran.
- 37 `mustmatch like "N"` checks of short numbers in `spec/version.md` and `spec/decide.md` became exact `mustmatch "N"`. `like "2"` passes on an exit code of 12. `demos/` held none. The `mustmatch not like "0"` counts stay, because they cannot pass on a count of zero.

## Red and green

A stand-in `thinkthen` ran the real binary and then printed one planted line on standard output and one on standard error.

| Line | Old check on planted output | New check on planted output | New check on the real binary |
| --- | --- | --- | --- |
| `spec/version.md:16` | exit 0 | exit 1 | exit 0 |
| `spec/version.md:31` | exit 0 | exit 1 | exit 0 |
| `spec/decide.md:151` | exit 0 | exit 1 | exit 0 |
| `spec/decide.md:230` (now 231) | exit 0 | exit 1 | exit 0 |

With the runner from `origin/main`, `demos-self-test` printed `loose-empty exited 0 and 1 was wanted`. With the new runner over the old spec pages, `demos` printed one refusal for each of the four lines.

## Review follow-up

The review found that the refusal missed `mustmatch -i like ""` and `mustmatch like -- ""`. The runner now allows `-i`, `--ignore-case`, `-q`, `--quiet`, `--`, and `not` on either side of `like`. The self-test gains `loose-ignore-case` and `loose-dashes`. Before the runner changed, both printed `exited 0 and 1 was wanted`. After it, `demos-self-test: 25 cases pass`.

## Checks

- `sdlc/scripts/lint`: exit 0.
- `sdlc/scripts/spec` with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset: exit 0, `demos-self-test: 23 cases pass`, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run. No code changed.
