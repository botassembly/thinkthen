# warm refuses the question file that decide uses

Status: Open

Found 2026-09-25 by the code review of the talk's SQL slide. The slide asks a banded decide question from a file, `'@abbey.json'`, whose JSON holds `"threshold": "0.3:0.7"`.

## What happens

- SQLite: `SELECT thinkthen_warm('@abbey.json', title) FROM songs;` fails with `thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide`. `databases/sqlite/README.md` line 28 says a banded question goes to `thinkthen_decide` and `thinkthen_details` only.
- DuckDB: warm refuses any `'@file'` question.
- The same warm with the plain question text works. It fills the same cache entries, because the band stays out of the request body (`specification/channels.md` line 108).

## Why it matters

A user writes the question once in a file and uses it in the query. Warming the table first needs the question typed a second time, without its band. The error message sends the user to `thinkthen_decide` and never says to drop the band.

## The fix

Warm takes the same question file as decide on every database, ignores the band, and fills the entries decide will read. A test warms with the banded file, then runs decide under a loopback listener and counts zero requests. If the team keeps the refusal, the error names the fix: "warm asks the plain question; drop the threshold".
