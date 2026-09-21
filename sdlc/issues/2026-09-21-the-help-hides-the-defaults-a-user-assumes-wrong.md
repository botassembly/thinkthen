# The help hides the defaults a user assumes wrong

Status: Open

Five values and behaviors a user meets on the first run are missing from the help text or hidden behind it. Each one sends a careful user in a wrong direction.

## The five rows

| # | What the help prints today | What a user assumes | The fix |
| --- | --- | --- | --- |
| 1 | `--threshold`: "The rule: one cut T, or a band LOW:HIGH that leaves a middle unresolved" (`decide --help`) | The cut has a stated default; a user checking `$?` against an assumed 0.9 gets a yes from 0.5 | State the default cut: "It defaults to 0.5" |
| 2 | `--jobs`: "How many requests are in flight at once, from 1 to 32" (`decide --help`; `records.md:124` gives the default of 4) | One, because no number is printed | State the default, `[default: 4]` |
| 3 | Short `-h` on every verb omits `--url`, `--model`, `--record`, `--replay`, `--cache`, `--timeout`, `--jobs`, and `--max-retries` | The short help lists the options that exist | Show `--url` at least, because without it every run reaches the default vendor address |
| 4 | `find --help` documents no exit codes and never mentions exit 3 or the empty standard output when `--none` wins; `specification/find.md:33` and :57 carry both facts | The tool failed when it printed nothing | Add the exit-3 and empty-stdout sentence to the help |
| 5 | `--timeout 0` is accepted: the run fails at once with `timeout: global`, exit 4 | Zero is rejected as invalid | Refuse 0, or say that 0 times out at once |

Verified 2026-09-21: rows 1 and 2 from `decide --help`; row 3 from `decide -h` and `choose -h` (zero matches for `--url`); row 4 from `find --help` (no line mentions exit codes); row 5 from `thinkthen decide 'Is it?' --timeout 0 --max-retries 0 --url http://127.0.0.1:1`.

## How bad it is for a user

Minor. Every row is a first-week misreading that costs a boundary, a bill, a diagnosis, or a debug session.

Found by experiment 218, wave 1.5, the blind seat.
